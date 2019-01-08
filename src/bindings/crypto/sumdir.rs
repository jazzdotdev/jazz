use rlua::{Error as LuaError, Lua};
use sumdir;

pub fn gen_checksum(_lua: &Lua, path: String) -> Result<String, LuaError> {
    Ok(sumdir::dir_hash(&path[..]))
}