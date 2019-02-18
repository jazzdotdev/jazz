use rlua::prelude::*;
use ulid::Ulid;

pub fn init(lua: &Lua) -> crate::Result<()> {

    lua.context(|lua| {
        let module = lua.create_table()?;
        module.set("new", lua.create_function(|_, _: ()| {
            Ok(Ulid::new().to_string())
        })?)?;

        lua.globals().set("ulid", module)?;

        Ok(())
    })
}
