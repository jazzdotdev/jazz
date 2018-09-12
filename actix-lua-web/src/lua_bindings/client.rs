use actix_web::client;
use actix_web::HttpMessage;
use actix_web::client::ClientResponse;
use futures::Future;
use rlua::prelude::*;
use rlua_serde;
use serde_json::{self, Value as JsonValue};

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

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    // json_or_string = client.get("https://some_url.json")
    let get = lua.create_function(|lua, url: String| {
        let response = client::get(&url)
            .finish()
            .map_err(|err| {
                LuaError::external(format_err!("Failed creating GET request: {}", err))
            })?
            .send()
            .wait()
            .map_err(|err| {
                LuaError::external(format_err!("GET request failed: {}", err))
            })?;

        parse_response(lua, response)
    })?;

    // json_or_string = client.post("https://some_url.json", { post_data = "data" })
    let post = lua.create_function(|lua, (url, body): (String, Option<LuaValue>)| {
        let mut req_builder = client::post(&url);

        if let Some(value) = body {
            match value {
                LuaValue::Table(_) => {
                    req_builder.content_type("application/json");
                    let json_value: JsonValue = rlua_serde::from_value(value)
                        .map_err(|err| {
                            LuaError::external(err)
                        })?;
                    let json_str = serde_json::to_string(&json_value)
                        .map_err(|err| {
                            LuaError::external(err)
                        })?;
                    req_builder.body(&json_str).unwrap();
                },
                _ => return Err(LuaError::external(format_err!("Unsupported POST body: {:?}", value))),
            }
        }

        let response = req_builder
            .finish()
            .map_err(|err| {
                LuaError::external(format_err!("Failed creating GET request: {}", err))
            })?
            .send()
            .wait()
            .map_err(|err| {
                LuaError::external(format_err!("GET request failed: {}", err))
            })?;

        parse_response(lua, response)
    })?;

    let module = lua.create_table()?;
    module.set("get", get)?;
    module.set("post", post)?;

    lua.globals().set("client", module)?;

    Ok(())
}
