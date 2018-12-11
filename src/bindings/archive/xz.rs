use rlua::prelude::*;
use std::fs;
use std::io::Read;
use xz2::read::*;

use super::ByteBuf;

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    let module = lua.create_table()?;
    module.set("decompress", lua.create_function(|_, file: (String)| {
        let file = fs::File::open(&file).map_err(LuaError::external)?;
        let mut data = XzDecoder::new(file);
        let mut buf = vec![];
        data.read_to_end(&mut buf).map_err(LuaError::external)?;
        Ok(ByteBuf(buf))
    })?)?;

    lua.globals().set("xz", module)?;

    Ok(())
}
