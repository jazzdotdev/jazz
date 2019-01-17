pub mod diff;
pub mod json;
pub mod patch;
pub mod scl;
pub mod select;
pub mod splitdiff;
pub mod yaml;

use rlua::prelude::*;

pub fn init(lua: &Lua) -> crate::Result<()> {
    json::init(&lua)?;
    scl::init(&lua)?;
    select::init(&lua)?;
    yaml::init(&lua)?;
    diff::init(&lua)?;
    patch::init(&lua)?;
    splitdiff::init(&lua)?;

    Ok(())
}
