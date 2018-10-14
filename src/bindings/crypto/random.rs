use rlua::{Error as LuaError, Lua};
use rust_sodium::randombytes;

/// Returns randomly generated size bytes of data
pub fn random_bytes(_lua: &Lua, size: usize) -> Result<Vec<u8>, LuaError> {
    Ok(randombytes::randombytes(size))
}
