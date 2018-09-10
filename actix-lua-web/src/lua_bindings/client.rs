use actix_web::client;
use actix_web::HttpMessage;
use futures::Future;
use rlua::prelude::*;
use rlua_serde;
use serde_json::{Value as JsonValue};

pub fn init(lua: &Lua) -> Result<(), LuaError> {
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

        let body = match response.content_type() {
            "application/json" => {
                let json: JsonValue = response.json().wait()
                    .map_err(|err| LuaError::external(err))?;
                rlua_serde::to_value(lua, json)
                    .map_err(|err| LuaError::external(err))?
            },
            _ => {
                let body = response.body().wait()                    .map_err(|err| {
                    LuaError::external(format_err!("Invalid body {}", err))
                })?;
                let body_string = String::from_utf8(body.iter().cloned().collect())
                    .map_err(|err| {
                        LuaError::external(format_err!("Invalid body {}", err))
                    })?;

                rlua_serde::to_value(lua, body_string).unwrap()
            }
        };

        Ok(body)
    })?;

    let module = lua.create_table()?;
    module.set("get", get)?;

    lua.globals().set("client", module)?;

    Ok(())
}
