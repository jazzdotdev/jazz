pub mod fs;
pub mod time;

use rlua::prelude::*;

pub fn init(lua: &Lua) -> ::Result<()> {
    fs::init(&lua)?;
    time::init(&lua)?;

    Ok(())
}