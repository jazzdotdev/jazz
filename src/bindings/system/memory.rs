use rlua::prelude::*;
use std::{
    io::Cursor,
    sync::{Arc, Mutex},
};
use crate::bindings::system::LuaCommonIO;

pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        let module = lua.create_table()?;

        module.set("new", lua.create_function(|_, _data: Option<Vec<u8>>| {
            let fs: Arc<Mutex<Cursor<Vec<u8>>>> = Arc::new(Mutex::new(Cursor::new(Vec::new())));
            Ok(LuaCommonIO {
                inner: None,
                stdin: Some(fs.clone()),
                stdout: Some(fs.clone()),
                stderr: None,
                seek: Some(fs.clone())
            })
        })?)?;

        lua.globals().set("memory", module)?;

        Ok(())
    })
}