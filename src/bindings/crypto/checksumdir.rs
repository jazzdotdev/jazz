use rlua::{Error as LuaError, Lua};
use checksumdir;

pub fn checksumdir(_lua: &Lua, path: String) -> Result<String, LuaError> {
    checksumdir::checksumdir(&path[..])
        .map_err(LuaError::external)
}