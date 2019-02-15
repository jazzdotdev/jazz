use rlua::prelude::*;
use std::process;

#[allow(unreachable_code)]
pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        let module = lua.create_table()?;

        module.set("exit", lua.create_function(|_, code: i32| {
            Ok(process::exit(code))
        })?)?;

        module.set("abort", lua.create_function(|_, _: ()| {
            Ok(process::abort())
        })?)?;

        module.set("id", lua.create_function(|_, _: ()| {
            Ok(process::id())
        })?)?;

        lua.globals().set("process", module)?;

        Ok(())
    })
}
