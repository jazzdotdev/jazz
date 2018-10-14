use std::rc::Rc;
use std::cell::RefCell;
use actix_web;
use actix_web::HttpMessage;
use actix_web::client::{ClientRequest, ClientRequestBuilder, ClientResponse};
use futures::Future;
use rlua::prelude::*;
use rlua_serde;
use serde_json::{Value as JsonValue};
use rlua::{UserDataMethods, UserData};

fn map_actix_err(err: actix_web::Error) -> LuaError {
    LuaError::external(format_err!("actix_web error: {}", &err))
}

fn parse_response(lua: &Lua, res: ClientResponse) -> LuaResult<LuaTable> {
    trace!("A");
    let body_data = res.body()
        .wait()
        .map_err(|err| {
            LuaError::external(format_err!("Invalid body {}", err))
        })?;
    trace!("B");

    let body_string = String::from_utf8(body_data.iter().cloned().collect())
        .map_err(|err| {
            LuaError::external(format_err!("Invalid body {}", err))
        })?;
    trace!("C");

    let body = match res.content_type() {
        "application/json" => {
            trace!("C1");
            // TODO: It's panicing for some reason. Grave.
            let json: JsonValue = res.json().wait()
                .map_err(|err| LuaError::external(err))?;
            trace!("C2");
            rlua_serde::to_value(lua, json)
                .map_err(|err| LuaError::external(err))
        },
        _ => {
            trace!("C3");
            rlua_serde::to_value(lua, body_string.clone())
        }
    }?;
    trace!("D");

    let headers = lua.create_table()?;
    trace!("E");

    for (key, value) in res.headers().iter() {
        if let Ok(value) = value.to_str() {
            headers.set(key.as_str(), value)?;
        }
    }
    trace!("F");

    let lres = lua.create_table()?;
    lres.set("status", res.status().as_u16())?;
    lres.set("headers", headers)?;
    lres.set("body", body)?;
    lres.set("body_raw", body_string)?;
    trace!("G");

    Ok(lres)
}

/// For POST and PUT requests.
fn set_body(value: LuaValue, request_builder: &mut ClientRequestBuilder) -> LuaResult<ClientRequest> {
    match value {
        LuaValue::Table(_) => {
            let json_value: JsonValue = rlua_serde::from_value(value)
                .map_err(LuaError::external)?;
            request_builder.json(&json_value).map_err(map_actix_err)
        },
        LuaValue::String(string) => {
            let string = string.to_str()?.to_owned();
            request_builder.body(&string).map_err(map_actix_err)
        },
        _ => Err(LuaError::external(format_err!("Unsupported POST body: {:?}", value))),
    }
}

fn set_headers(value: LuaValue, request_builder: &mut ClientRequestBuilder) -> LuaResult<()> {
    if let LuaValue::Table(headers) = value.clone() {
        for pair in headers.pairs() {
            let (key, value): (String, LuaValue) = pair?;
            let value = match value {
                LuaValue::String(value) => value.to_str()?.to_owned(),
                LuaValue::Number(number) => number.to_string(),
                LuaValue::Integer(number) => number.to_string(),
                ref value @ _ => unimplemented!("Header value is not supported: {:?}", value),
            };
            request_builder.header(&key as &str, value);
        }
    } else {
        return Err(LuaError::external(format_err!("Invalid client headers {:?}", &value)))
    }

    Ok(())
}

#[derive(Clone)]
struct Builder(Rc<RefCell<ClientRequestBuilder>>);

// TODO: Should be safe since it's not really being shared across threads.
// Still, it would be better to find another way to make it work.
unsafe impl Send for Builder {}

impl UserData for Builder {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        use std::str::FromStr;
        use actix_web::http::Method;

        methods.add_method_mut("method", |_, this, method: String| {
            this.0.borrow_mut().method(Method::from_str(&method).map_err(LuaError::external)?);
            Ok(this.clone())
        });

        methods.add_method_mut("uri", |_, this, uri: String| {
            this.0.borrow_mut().uri(&uri);
            Ok(this.clone())
        });

        methods.add_method_mut("send_with_body", |lua, this, body: LuaValue| {
            let response = set_body(body, &mut this.0.borrow_mut())?
                .send()
                .wait().map_err(|err| {
                    LuaError::external(format_err!("Request failed: {}", err))
                })?;

            parse_response(lua, response)
        });

        methods.add_method_mut("headers", |_, this, headers: LuaValue| {
            set_headers(headers, &mut this.0.borrow_mut())?;
            Ok(this.clone())
        });

        methods.add_method_mut("send", |lua, this, _: ()| {
            let response = this.0.borrow_mut()
                .finish().map_err(map_actix_err)?
                .send()
                .wait().map_err(|err| {
                    LuaError::external(format_err!("Request failed: {}", err))
                })?;

            parse_response(lua, response)
        });
    }
}


pub fn init(lua: &Lua) -> Result<(), LuaError> {

    let table = lua.create_table()?;
    table.set("build", lua.create_function(|_, _: ()| {
        Ok(Builder(Rc::new(RefCell::new(ClientRequest::build()))))
    })?)?;

    lua.globals().set("ClientRequest", table)?;

    Ok(())
}
