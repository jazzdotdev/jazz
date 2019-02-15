pub mod case;
pub mod mime;
pub mod regex;
pub mod stringset;
pub mod uuid;
pub mod base64;
pub mod ulid;

use rlua::prelude::*;

pub fn init(lua: &Lua) -> crate::Result<()> {
    case::init(lua)?;
    mime::init(lua)?;
    regex::init(lua)?;
    stringset::init(lua)?;
    uuid::init(lua)?;
    base64::init(lua)?;
    ulid::init(lua)?;
    Ok(())
}