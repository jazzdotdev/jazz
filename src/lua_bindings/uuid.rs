use rlua::prelude::*;
use uuid::Uuid;

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    // Loads yaml from a string into a table
    let generate_uuid_v4 = lua.create_function(|_, _: ()| {
        let uuid = Uuid::new_v4().to_string();

        Ok(uuid)
    })?;

    let module = lua.create_table()?;
    module.set("v4", generate_uuid_v4)?;

    lua.globals().set("uuid", module)?;

    Ok(())
}
