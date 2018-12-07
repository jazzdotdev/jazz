use rlua::{Error as LuaError, Lua};
use blake2::*;
use rust_sodium::crypto::hash;
use base64;

/// Returns base64 encoded SHA-512 of `msg`
pub fn hash(_lua: &Lua, msg: String) -> Result<String, LuaError> {
    let digest = hash::hash(msg.as_bytes());
    Ok(base64::encode(&digest))
}

/// Returns base64 encoded BLAKE2B of `msg`
pub fn blake2_hash(_lua: &Lua, msg: String) -> Result<String, LuaError> {
    let mut hasher = Blake2b::new();
    hasher.input(msg);
    let digest = hasher.result();
    Ok(base64::encode(&digest))
}
