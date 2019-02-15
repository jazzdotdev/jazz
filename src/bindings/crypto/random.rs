use rlua::{Error as LuaError, Context};
use sodiumoxide::randombytes;

/// Returns randomly generated size bytes of data
pub fn random_bytes(_lua: Context, size: usize) -> Result<Vec<u8>, LuaError> {
    Ok(randombytes::randombytes(size))
}
