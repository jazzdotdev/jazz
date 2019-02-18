pub mod client;
pub mod server;
pub mod sass;

use rlua::prelude::*;
use crate::Result;

pub fn init(lua: &Lua) -> Result<()> {
    client::init(lua)?;
    sass::init(lua)?;
    Ok(())
}