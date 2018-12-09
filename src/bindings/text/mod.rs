use rlua::prelude::*;

pub mod json;
pub mod scl;
pub mod select;
pub mod yaml;

pub fn init(lua: &Lua) -> LuaResult<()> {
    json::init(&lua)?;
    scl::init(&lua)?;
    select::init(&lua)?;
    yaml::init(&lua)?;

    Ok(())
}