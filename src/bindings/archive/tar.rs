use rlua::prelude::*;
use std::fs;
use std::path::Path;
use tar::Archive;

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    let module = lua.create_table()?;
    module.set("decompress", lua.create_function(|_, (src, dst): (String, String)| {
        let tar = fs::File::open(src).map_err(LuaError::external)?;
        let dst = Path::new(&dst);
        let mut archive = Archive::new(tar);

        for file in archive.entries().map_err(LuaError::external)? {
            file
                .and_then(|mut file| file.unpack_in(&dst))
                .map_err(LuaError::external)?;
        }

        Ok(())
    })?)?;

    lua.globals().set("tar", module)?;

    Ok(())
}
