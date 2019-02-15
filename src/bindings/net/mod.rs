pub mod ipv4;
pub mod ipv6;

use rlua::prelude::*;

pub fn init(lua: &Lua) -> crate::Result<()> {
    ipv4::init(lua)?;
    ipv6::init(lua)?;

    Ok(())
}