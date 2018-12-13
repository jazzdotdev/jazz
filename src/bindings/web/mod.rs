pub mod client;
pub mod server;

use rlua::prelude::*;
use Result;

pub fn init(lua: &Lua) -> Result<()> {
    client::init(&lua)?;

    Ok(())
}