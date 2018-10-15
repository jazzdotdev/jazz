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
use actix_web::http::Method;
use std::str::FromStr;

fn map_actix_err(err: actix_web::Error) -> LuaError {
    LuaError::external(format_err!("actix_web error: {}", &err))
}

fn parse_response(lua: &Lua, res: ClientResponse) -> LuaResult<LuaTable> {
    let body_data = res.body()
        .wait()
        .map_err(|err| {
            LuaError::external(format_err!("Invalid body {}", err))
        })?;

    let body_string = String::from_utf8(body_data.iter().cloned().collect())
        .map_err(|err| {
            LuaError::external(format_err!("Invalid body {}", err))
        })?;

    /*let body = match res.content_type() {
        "application/json" => {
            // TODO: When it reaches here it panics for some reason. Grave.
            let json: JsonValue = res.json().wait()
                .map_err(|err| LuaError::external(err))?;
    
            rlua_serde::to_value(lua, json)
                .map_err(|err| LuaError::external(err))
        },
        _ => {
    
            rlua_serde::to_value(lua, body_string.clone())
        }
    }?;*/

    let headers = lua.create_table()?;

    for (key, value) in res.headers().iter() {
        if let Ok(value) = value.to_str() {
            headers.set(key.as_str(), value)?;
        }
    }

    let lres = lua.create_table()?;
    lres.set("status", res.status().as_u16())?;
    lres.set("headers", headers)?;
    //lres.set("body", body)?;
    lres.set("body_raw", body_string)?;

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

fn send_lua_request <'a> (lua: &'a Lua, val: LuaValue<'a>) -> LuaResult<LuaTable<'a>> {

    let mut builder = ClientRequest::build();

    let body = match val {
        LuaValue::String(s) => {
            builder.uri(s.to_str()?).method(Method::GET);
            None
        },
        LuaValue::Table(table) => {
            if let Some(method) = table.get::<_, Option<String>>("method")? {
                builder.method(Method::from_str(&method).map_err(LuaError::external)?);
            }

            if let Some(uri) = table.get::<_, Option<String>>("uri")? {
                builder.uri(&uri);
            }

            if let Some(headers) = table.get("headers")? {
                set_headers(headers, &mut builder)?;
            }

            table.get("body")?
        },
        _ => {
            return Err(LuaError::RuntimeError("Invalid arguments".to_string()))
        }
    };

    let response = (match body {
        Some(body) => set_body(body, &mut builder)?,
        None => builder.finish().map_err(map_actix_err)?
    }).send().wait().map_err(|err| {
        LuaError::external(format_err!("Request failed: {}", err))
    })?;

    parse_response(lua, response)
}


pub fn init(lua: &Lua) -> Result<(), LuaError> {

    let table = lua.create_table()?;
    table.set("send", lua.create_function(send_lua_request)?)?;

    lua.globals().set("client_request", table)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lua_client_request () {
        // TODO: I don't know how to make this work

        use actix_lua::{LuaMessage, LuaActorBuilder};
        use actix::Actor;

        println!("A");
        let code = ::actix::System::run(|| {
            println!("B");
            let addr = LuaActorBuilder::new().on_handle_with_lua(r#"
                assert(false)

                local response = client_request.send("https://jsonplaceholder.typicode.com/todos/1")

                print("response", response)
                for k, v in pairs(response) do
                    print("", k, v)
                    if type(v) == "table" then
                        for k, v in pairs(v) do
                            print("", "", k, v)
                        end
                    end
                end

                assert(response.status == 200)
                assert(response.body.title == "delectus aut autem")

                local response = client_request.send{
                    uri = "https://jsonplaceholder.typicode.com/posts",
                    method = "post",
                    body = [[{
                        "title": "foo",
                        "body": "bar",
                        "userId": "1"
                    }]]
                }
                assert(response.status == 201)
                assert(response.body.id == 101)
                assert(response.body.title == "foo")

                local response = client_request.send("https://jsonplaceholder.typicode.com/foo/bar")
                assert(response.status == 404)
            "#).build().unwrap().start();
            println!("C");
            let req = addr.send(LuaMessage::Nil).wait().unwrap();
            println!("D");
        });
        println!("E");
    }
}
