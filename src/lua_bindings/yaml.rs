use rlua::prelude::*;
use serde_yaml;
use rlua_serde;

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    // Loads yaml from a string into a table
    let load_yaml = lua.create_function(|lua, text: String| {
        let doc: serde_yaml::Value = serde_yaml::from_str(&text)
            .map_err(|err| {
                LuaError::external(err)
            })?;
        let lua_value = rlua_serde::to_value(lua, &doc)?;

        Ok(lua_value)
    })?;

    // Loads yaml from a string into a table
    let dump_yaml = lua.create_function(|_, value: LuaValue| {
        let lua_value: serde_yaml::Value = rlua_serde::from_value(value)?;
        let string = serde_yaml::to_string(&lua_value)
            .map_err(|err| {
                LuaError::external(err)
            })?;

        Ok(string)
    })?;

    let module = lua.create_table()?;
    module.set("load", load_yaml)?;
    module.set("dump", dump_yaml)?;

    lua.globals().set("yaml", module)?;

    Ok(())
}
