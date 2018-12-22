pub mod fs;
pub mod time;
pub mod path;

use rlua::prelude::*;

pub fn init(lua: &Lua) -> ::Result<()> {
    fs::init(&lua)?;
    time::init(&lua)?;
    path::init(&lua)?;

    Ok(())
}