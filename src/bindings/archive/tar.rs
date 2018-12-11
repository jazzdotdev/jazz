use rlua::prelude::*;
use std::fs;
use std::io::Read;
use std::path::Path;
use tar::Archive;

use super::ByteBuf;

fn extract<T: Read>(archive: &mut Archive<T>, dst: &Path) -> Result<(), LuaError> {
    for file in archive.entries().map_err(LuaError::external)? {
        file
            .and_then(|mut file| file.unpack_in(&dst))
            .map_err(LuaError::external)?;
    }
    Ok(())
}

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    let module = lua.create_table()?;
    module.set("decompress", lua.create_function(|_, (src, dst): (String, String)| {
        let tar = fs::File::open(src).map_err(LuaError::external)?;
        let dst = Path::new(&dst);
        let mut archive = Archive::new(tar);
        extract(&mut archive, &dst)
    })?)?;

    module.set("decompress_buf", lua.create_function(|_, (data, dst): (ByteBuf, String)| {
        let dst = Path::new(&dst);
        let mut archive = Archive::new(&data.0[..]);
        extract(&mut archive, &dst)
    })?)?;

    lua.globals().set("tar", module)?;

    Ok(())
}
