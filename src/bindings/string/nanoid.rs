use rlua::prelude::*;
use nanoid;

pub fn init(lua: &Lua) -> crate::Result<()> {

    lua.context(|lua| {
        let module = lua.create_table()?;
        module.set("simple", lua.create_function(|_, _: ()| {
            Ok(nanoid::simple())
        })?)?;
        module.set("generate", lua.create_function(|_, size: usize| {
            Ok(nanoid::generate(size))
        })?)?;
        lua.globals().set("nanoid", module)?;
        Ok(())
    })

}
