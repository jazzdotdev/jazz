use rlua::prelude::*;
use uuid::Uuid;

pub fn init(lua: &Lua) -> Result<(), LuaError> {

    let generate_uuid_v4 = lua.create_function(|_, _: ()| {
        let uuid = Uuid::new_v4().to_string();
        Ok(uuid)
    })?;

    let check_uuid_string = lua.create_function(|_, s: String| {
        Ok(Uuid::parse_str(&s).is_ok())
    })?;


    let module = lua.create_table()?;
    module.set("v4", generate_uuid_v4)?;
    module.set("check", check_uuid_string)?;

    lua.globals().set("uuid", module)?;

    Ok(())
}
