use rlua::prelude::*;

pub mod client;
pub mod server;

pub fn init(lua: &Lua) -> LuaResult<()> {
    client::init(&lua)?;

    Ok(())
}