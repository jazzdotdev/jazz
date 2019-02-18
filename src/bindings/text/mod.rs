pub mod diff;
pub mod json;
pub mod patch;
pub mod scl;
pub mod select;
pub mod splitdiff;
pub mod combinediff;
pub mod interdiff;
pub mod yaml;

#[cfg(target_os = "windows")]
pub const NULL_SOURCE: &str = "nul";
#[cfg(target_os = "linux")]
pub const NULL_SOURCE: &str = "/dev/null";

use rlua::prelude::*;

pub fn init(lua: &Lua) -> crate::Result<()> {
    json::init(lua)?;
    scl::init(lua)?;
    select::init(lua)?;
    yaml::init(lua)?;
    diff::init(lua)?;
    patch::init(lua)?;
    splitdiff::init(lua)?;
    combinediff::init(lua)?;
    interdiff::init(lua)?;
    Ok(())
}
