use rlua::prelude::*;

pub mod heck;
pub mod mime;
pub mod regex;
pub mod stringset;
pub mod uuid;

pub fn init(lua: &Lua) -> LuaResult<()> {
    heck::init(&lua)?;
    mime::init(&lua)?;
    regex::init(&lua)?;
    stringset::init(&lua)?;
    uuid::init(&lua)?;

    Ok(())
}