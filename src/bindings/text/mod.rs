pub mod json;
pub mod scl;
pub mod select;
pub mod yaml;

use rlua::prelude::*;

pub fn init(lua: &Lua) -> ::Result<()> {
    json::init(&lua)?;
    scl::init(&lua)?;
    select::init(&lua)?;
    yaml::init(&lua)?;

    Ok(())
}