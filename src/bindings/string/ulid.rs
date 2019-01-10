use rlua::prelude::*;
use ulid::Ulid;

pub fn init(lua: &Lua) -> ::Result<()> {

    let module = lua.create_table()?;
    module.set("new", lua.create_function(|_, _: ()| {
        Ok(Ulid::new().to_string())
    })?)?;

    lua.globals().set("ulid", module)?;

    Ok(())
}
