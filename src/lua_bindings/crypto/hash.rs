use rlua::{Error as LuaError, Lua};
use rust_sodium::crypto::hash;
use base64;

/// Returns base64 encoded SHA-512 of `msg`
pub fn hash(_lua: &Lua, msg: String) -> Result<String, LuaError> {
    let digest = hash::hash(msg.as_bytes());
    Ok(base64::encode(&digest))
}