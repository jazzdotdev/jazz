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

fn parse_response(lua: &Lua, res: ClientResponse) -> LuaResult<LuaValue> {
    let body = match res.content_type() {
        "application/json" => {
            let json: JsonValue = res.json().wait()
                .map_err(|err| LuaError::external(err))?;
            rlua_serde::to_value(lua, json)
                .map_err(|err| LuaError::external(err))
        },
        _ => {
            let body = res.body()
                .wait()
                .map_err(|err| {
                    LuaError::external(format_err!("Invalid body {}", err))
                })?;
            let body_string = String::from_utf8(body.iter().cloned().collect())
                .map_err(|err| {
                    LuaError::external(format_err!("Invalid body {}", err))
                })?;

            rlua_serde::to_value(lua, body_string)
        }
    };

    body
}

/// For POST and PUT requests.
fn set_body(value: LuaValue, req_builder: &mut ClientRequestBuilder) -> LuaResult<ClientRequest> {
    match value {
        LuaValue::Table(_) => {
            let json_value: JsonValue = rlua_serde::from_value(value)
                .map_err(LuaError::external)?;
            req_builder.json(&json_value).map_err(map_actix_err)
        },
        LuaValue::String(string) => {
            let string = string.to_str()?.to_owned();
            req_builder.body(&string).map_err(map_actix_err)
        },
        _ => Err(LuaError::external(format_err!("Unsupported POST body: {:?}", value))),
    }
}

fn set_headers(value: LuaValue, req_builder: &mut ClientRequestBuilder) -> LuaResult<()> {
    if let LuaValue::Table(headers) = value.clone() {
        for pair in headers.pairs() {
            let (key, value): (String, LuaValue) = pair?;
            let value = match value {
                LuaValue::String(value) => value.to_str()?.to_owned(),
                LuaValue::Number(number) => number.to_string(),
                LuaValue::Integer(number) => number.to_string(),
                ref value @ _ => unimplemented!("Header value is not supported: {:?}", value),
            };
            req_builder.header(&key as &str, value);
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
