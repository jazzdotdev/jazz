pub mod math;

use rlua::prelude::*;

pub fn init(lua: &Lua) -> crate::Result<()> {
    math::init(&lua)?;
    Ok(())
}