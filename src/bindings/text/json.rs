use rlua::prelude::*;
use serde_json;
use rlua_serde;

pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        // Decode string to a table
        let module = lua.create_table()?;
        module.set("to_table", lua.create_function(|lua, text: String| {
            let doc: serde_json::Value = serde_json::from_str(&text).map_err(LuaError::external)?;
            let lua_value = rlua_serde::to_value(lua, &doc)?;

            Ok(lua_value)
        })?)?;

        // Encode table to a string
        module.set("from_table", lua.create_function(|_, value: LuaValue| {
            let lua_value: serde_json::Value = rlua_serde::from_value(value)?;
            let string = serde_json::to_string(&lua_value).map_err(LuaError::external)?;

            Ok(string)
        })?)?;

        lua.globals().set("json", module)?;

        Ok(())
    })
}