use rlua::prelude::*;
use std::{
    fs,
    io::Read,
    result,
    path::Path
};
use tar::Archive;
use crate::error::Error;

use super::ByteBuf;

fn extract<T: Read>(archive: &mut Archive<T>, dst: &Path) -> result::Result<(), LuaError> {
    for file in archive.entries().map_err(LuaError::external)? {
        file
            .and_then(|mut file| file.unpack_in(&dst))
            .map_err(LuaError::external)?;
    }
    Ok(())
}

pub fn init(lua: &Lua) -> crate::Result<()> {
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

    lua.globals().set("tar", module).map_err(Error::from)?;

    Ok(())
}
