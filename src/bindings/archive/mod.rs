pub mod tar;
pub mod xz;
pub mod zip;

use rlua::prelude::*;
use rlua::UserData;

#[derive(Clone)]
struct ByteBuf(Vec<u8>);

impl UserData for ByteBuf {}


pub fn init(lua: &Lua) -> crate::Result<()> {
    tar::init(&lua)?;
    xz::init(&lua)?;
    zip::init(&lua)?;

    Ok(())
}
