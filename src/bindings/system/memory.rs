use rlua::prelude::*;
use std::{
    mem,
    io::{Cursor, SeekFrom, prelude::*}
};
use serde_json;
use rlua_serde;

//TODO: Move to having a common interface so IO can share the same binding
pub struct LuaMemory(Cursor<Vec<u8>>);

impl LuaUserData for LuaMemory {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("read", |_, this: &mut LuaMemory, len: Option<usize>|{
            let bytes = match len {
                Some(len) => {
                    let mut bytes = vec![0u8; len];
                    this.0.read(&mut bytes).map_err(LuaError::external)?;
                    bytes
                },
                None => {
                    let mut bytes = vec![];
                    this.0.read_to_end(&mut bytes).map_err(LuaError::external)?;
                    bytes
                }
            };
            Ok(bytes)
        });
        methods.add_method_mut("read_to_string", |_, this: &mut LuaMemory, _: ()|{
            let mut data = String::new();
            this.0.read_to_string(&mut data).map_err(LuaError::external)?;
            Ok(data)
        });
        methods.add_method_mut("write", |_, this: &mut LuaMemory, bytes: Vec<u8>|{
            Ok(this.0.write(bytes.as_slice()).map_err(LuaError::external)?)
        });
        methods.add_method_mut("write", |_, this: &mut LuaMemory, str: String|{
            Ok(this.0.write(str.as_bytes()).map_err(LuaError::external)?)
        });
        methods.add_method_mut("flush", |_, this: &mut LuaMemory, _: ()|{
            Ok(this.0.flush().map_err(LuaError::external)?)
        });
        methods.add_method_mut("clear", |_, this: &mut LuaMemory, _: ()|{
            mem::replace(&mut this.0, Cursor::new(Vec::new()));
            Ok(())
        });
        methods.add_method_mut("seek", |_, this: &mut LuaMemory, (pos, size): (Option<String>, Option<usize>)| {
            let size = size.unwrap_or(0);

            let seekfrom = pos.and_then(|s_pos| {
                Some(match s_pos.as_ref() {
                    "start" => SeekFrom::Start(size as u64),
                    "end" => SeekFrom::End(size as i64),
                    "current" | _ => SeekFrom::Current(size as i64),
                })
            }).unwrap_or(SeekFrom::Current(size as i64));
            Ok(this.0.seek(seekfrom).map_err(LuaError::external)?)
        });
        methods.add_method_mut("dump", |_, this: &mut LuaMemory, _: ()|{
            Ok(this.0.clone().into_inner())
        });
    }
}

pub fn init(lua: &Lua) -> crate::Result<()> {

    let module = lua.create_table()?;

    module.set("new", lua.create_function( |_, _: ()| {
        Ok(LuaMemory(Cursor::new(Vec::new())))
    })?  )?;

    lua.globals().set("memory", module)?;

    Ok(())
}