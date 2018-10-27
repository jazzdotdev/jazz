use rlua::prelude::*;
use serde_json;
use rlua_serde;

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    // Encode string to a table
    let encode = lua.create_function(|lua, text: String| {
        let doc: serde_json::Value = serde_json::from_str(&text)
            .map_err(|err| {
                LuaError::external(err)
            })?;
        let lua_value = rlua_serde::to_value(lua, &doc)?;

        Ok(lua_value)
    })?;

    // Decode table to a string
    let decode = lua.create_function(|_, value: LuaValue| {
        let lua_value: serde_json::Value = rlua_serde::from_value(value)?;
        let string = serde_json::to_string(&lua_value)
            .map_err(|err| {
                LuaError::external(err)
            })?;

        Ok(string)
    })?;

    let module = lua.create_table()?;
    module.set("decode", decode)?;
    module.set("encode", encode)?;

    lua.globals().set("json", module)?;

    Ok(())
}