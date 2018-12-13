pub mod heck;
pub mod mime;
pub mod regex;
pub mod stringset;
pub mod uuid;

use rlua::prelude::*;

pub fn init(lua: &Lua) -> ::Result<()> {
    heck::init(&lua)?;
    mime::init(&lua)?;
    regex::init(&lua)?;
    stringset::init(&lua)?;
    uuid::init(&lua)?;

    Ok(())
}