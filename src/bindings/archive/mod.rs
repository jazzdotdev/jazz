use rlua::prelude::*;

pub mod tar;
pub mod zip;

pub fn init(lua: &Lua) -> LuaResult<()> {
    tar::init(&lua)?;
    zip::init(&lua)?;

    Ok(())
}