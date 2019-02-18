use rlua::prelude::*;
use std::fs;
use std::io::{self, Read};
use xz2::read::*;
use super::ByteBuf;

pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        let module = lua.create_table()?;

        module.set("compress", lua.create_function(|_, (input, output, level): (String, String, Option<u32>)| {
            let level = level.unwrap_or(6);
            let file = fs::File::open(&input).map_err(LuaError::external)?;
            let mut output = fs::File::create(&output).map_err(LuaError::external)?;
            let mut data = XzEncoder::new(file, level);
            io::copy(&mut data, &mut output).map_err(LuaError::external)
        })?)?;

        module.set("decompress", lua.create_function(|_, file: String| {
            let file = fs::File::open(&file).map_err(LuaError::external)?;
            let mut data = XzDecoder::new(file);
            let mut buf = vec![];
            data.read_to_end(&mut buf).map_err(LuaError::external)?;
            Ok(ByteBuf(buf))
        })?)?;

        lua.globals().set("xz", module)?;

        Ok(())
    })
}
