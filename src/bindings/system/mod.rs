use rlua::prelude::*;

pub mod fs;
pub mod time;

pub fn init(lua: &Lua) -> LuaResult<()> {
    fs::init(&lua)?;
    time::init(&lua)?;

    Ok(())
}