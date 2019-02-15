use rlua::prelude::*;
use std::{
    collections::HashMap,
    env
};

pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        let module = lua.create_table()?;

        module.set("current_dir", lua.create_function(|_, _: ()| {
            env::current_dir().map(|path| path.to_str().map(|s| s.to_string())).map_err(LuaError::external)
        })?)?;

        module.set("current_exe", lua.create_function(|_, _: ()| {
            env::current_exe().map(|path| path.to_str().map(|s| s.to_string())).map_err(LuaError::external)
        })?)?;

        module.set("remove_var", lua.create_function(|_, var: String| {
            Ok(env::remove_var(var))
        })?)?;

        module.set("set_current_dir", lua.create_function(|_, path: String| {
            env::set_current_dir(path).map_err(LuaError::external)
        })?)?;

        module.set("set_var", lua.create_function(|_, (k, v): (String, String)| {
            Ok(env::set_var(k, v))
        })?)?;

        module.set("var", lua.create_function(|_, k: String| {
            env::var(k).map_err(LuaError::external)
        })?)?;

        module.set("vars", lua.create_function(|_, _: ()| {
            //We are going to use "vars_os" due to the nature of "vars" when iterating over the list of variable that could result in a panic
            let list: HashMap<String, String> = env::vars_os()
                .into_iter()
                .map(|(k, v)| (k.into_string().unwrap(), v.into_string().unwrap()))
                .collect();

            Ok(list)
        })?)?;

        lua.globals().set("env", module)?;

        Ok(())
    })
}
