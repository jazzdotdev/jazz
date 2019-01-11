use rlua::{Error as LuaError, Lua};
use checksumdir;

pub fn checksum(_lua: &Lua, path: String) -> Result<String, LuaError> {
    checksumdir::checksumdir(&path[..])
        .map_err(LuaError::external)
}