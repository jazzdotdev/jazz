use rlua::prelude::*;
use rlua::UserData;

pub mod tar;
pub mod xz;
pub mod zip;

#[derive(Clone)]
struct ByteBuf(Vec<u8>);

impl UserData for ByteBuf {}


pub fn init(lua: &Lua) -> LuaResult<()> {
    tar::init(&lua)?;
    xz::init(&lua)?;
    zip::init(&lua)?;

    Ok(())
}
