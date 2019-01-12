pub mod json;
pub mod scl;
pub mod select;
pub mod yaml;
pub mod diff;
pub mod patch;

use rlua::prelude::*;

pub fn init(lua: &Lua) -> crate::Result<()> {
    json::init(&lua)?;
    scl::init(&lua)?;
    select::init(&lua)?;
    yaml::init(&lua)?;
    diff::init(&lua)?;
    patch::init(&lua)?;

    Ok(())
}