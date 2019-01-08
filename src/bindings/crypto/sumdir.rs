use rlua::{Error as LuaError, Lua};
use checksumdir;

pub fn dir_hash(_lua: &Lua, path: String) -> Result<String, LuaError> {
    checksumdir::dir_hash(&path[..])
        .map_err(LuaError::external)
}