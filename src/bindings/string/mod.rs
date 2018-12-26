pub mod heck;
pub mod mime;
pub mod regex;
pub mod stringset;
pub mod uuid;
pub mod base64;

use rlua::prelude::*;

pub fn init(lua: &Lua) -> ::Result<()> {
    heck::init(&lua)?;
    mime::init(&lua)?;
    regex::init(&lua)?;
    stringset::init(&lua)?;
    uuid::init(&lua)?;
    base64::init(&lua)?;

    Ok(())
}