use rlua::{Error as LuaError, Lua};
use checksumdir;
use rlua;

pub fn checksum(_lua: &Lua, (path, array): (String, rlua::Table))
    -> Result<String, LuaError> {
    // created so rust doesn't drop ownership of Vec<String>
    let tmp: Vec<String> = array.pairs::<usize, String>()
                                .filter_map(|e| e.ok())
                                .map(|e| e.1)
                                .collect();

    let excludes: Vec<&str> = tmp.iter().map(AsRef::as_ref).collect();

    let opts = checksumdir::ChecksumOptions::new(excludes, false, true);
    checksumdir::checksumdir_with_options(&path, opts)
        .map_err(LuaError::external)
}
