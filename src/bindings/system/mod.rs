pub mod fs;
pub mod time;
pub mod path;
pub mod env;
pub mod dirs;
pub mod process;
pub mod command;
pub mod memory;

use rlua::prelude::*;
use std::{
    io::{SeekFrom, prelude::*},
    fs::{File, Metadata, Permissions},
    sync::{Mutex, Arc},
};
use crate::error::Error;

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

//TODO: Investigate the regression(?) between rlua and rust to determine the best course to take when calls from rust to use within lua
//      due to the performance decrease with any IO calls. This could also be what caused tantivy to have a degrade in performance
//      when using it with rlua.

//Mutex will be used for the time being but will change in the future.
#[derive(Clone)]
pub struct LuaCommonIO {
    inner: Option<Arc<Mutex<Box<File>>>>,
    stdin: Option<Arc<Mutex<Write>>>,
    stdout: Option<Arc<Mutex<Read>>>,
    stderr: Option<Arc<Mutex<Read>>>,
    seek: Option<Arc<Mutex<Seek>>>,
}

unsafe impl Send for LuaCommonIO {}
unsafe impl Sync for LuaCommonIO {}

pub struct LuaMetadata(Metadata);
pub struct LuaPermissions(Permissions);

impl LuaUserData for LuaCommonIO {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("write", |_, this: &mut LuaCommonIO, data: LuaValue|{
            let stdin = this.stdin.clone().ok_or(LuaError::external(Error::InternalError))?;
            let mut stdin = stdin.lock().unwrap();
            Ok(match data {
                    LuaValue::String(ref string) => stdin.write(string.as_bytes()).ok(),
                    LuaValue::Table(table) => {
                        let data: Vec<u8> = table.sequence_values().into_iter().filter_map(Result::ok).collect();
                        stdin.write(data.as_slice()).ok()
                    },
                    _ => None
            })
        });
        methods.add_method_mut("flush", |_, this: &mut LuaCommonIO, _: ()|{
            let stdin = this.stdin.clone().ok_or(LuaError::external(Error::InternalError))?;
            let mut stdin = stdin.lock().unwrap();
            stdin.flush().map_err(LuaError::external)
        });

        methods.add_method_mut("read", |lua: &Lua, this: &mut LuaCommonIO, (data, _opt): (Option<LuaValue>, Option<LuaValue>)|{
            let stdout = this.stdout.clone().ok_or(LuaError::external(Error::InternalError))?;
            let mut stdout = stdout.lock().unwrap();
            match data {
                Some(LuaValue::Integer(len)) => {
                    let mut bytes = vec![0u8; len as usize];
                    stdout.read(&mut bytes).map_err(LuaError::external)?;
                    lua.create_sequence_from(bytes).map(LuaValue::Table)
                },
                Some(LuaValue::String(mode)) => {
                    //TODO: Implement other modes for reading
                    match mode.to_str().ok() {
                        Some("string") | _ => {
                            let mut data = String::new();
                            stdout.read_to_string(&mut data).map_err(LuaError::external)?;
                            lua.create_string(&data).map(LuaValue::String)
                        },
                    }
                },
                None | _ => {
                    let mut bytes = vec![];
                    stdout.read_to_end(&mut bytes).map_err(LuaError::external)?;
                    lua.create_sequence_from(bytes).map(LuaValue::Table)
                }
            }
        });
        methods.add_method_mut("seek", |_, this: &mut LuaCommonIO, (pos, size): (Option<String>, Option<usize>)| {
            let seek = this.seek.clone().ok_or(LuaError::external(Error::InternalError))?;
            let mut seek = seek.lock().unwrap();

            let size = size.unwrap_or(0);

            let seekfrom = pos.and_then(|s_pos| {
                Some(match s_pos.as_ref() {
                    "start" => SeekFrom::Start(size as u64),
                    "end" => SeekFrom::End(size as i64),
                    "current" | _ => SeekFrom::Current(size as i64),
                })
            }).unwrap_or(SeekFrom::Current(size as i64));
            seek.seek(seekfrom).map_err(LuaError::external)
        });
        methods.add_method_mut("sync", |_, this: &mut LuaCommonIO, opt: Option<String>|{
            let fd = this.inner.clone().ok_or(LuaError::external(Error::InternalError))?;
            let fd = fd.lock().unwrap();
            match opt.as_ref().map(|s| s.as_str()) {
                Some("data") => fd.sync_data().map_err(LuaError::external),
                Some("all") | _ => fd.sync_all().map_err(LuaError::external)
            }
        });
        methods.add_method("metadata", |_, this: &LuaCommonIO, _: ()| {
            let fd = this.inner.clone().ok_or(LuaError::external(Error::InternalError))?;
            let fd = fd.lock().unwrap();
            fd.metadata().map(LuaMetadata).map_err(LuaError::external)
        });
        methods.add_method_mut("close", |_, this: &mut LuaCommonIO, _: ()| {
            this.inner = None;
            this.stdin = None;
            this.stdout = None;
            this.stderr = None;
            Ok(())
        });
    }
}

impl LuaUserData for LuaMetadata {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("created", |_, this: &LuaMetadata, _: ()| {
            this.0.created().map(|time| time.duration_since(::std::time::SystemTime::UNIX_EPOCH).map(|s| s.as_secs()).unwrap_or(0)).map_err(LuaError::external)
        });
        methods.add_method("modified", |_, this: &LuaMetadata, _: ()| {
            this.0.modified().map(|time| time.duration_since(::std::time::SystemTime::UNIX_EPOCH).map(|s| s.as_secs()).unwrap_or(0)).map_err(LuaError::external)
        });
        methods.add_method("accessed", |_, this: &LuaMetadata, _: ()| {
            this.0.accessed().map(|time| time.duration_since(::std::time::SystemTime::UNIX_EPOCH).map(|s| s.as_secs()).unwrap_or(0)).map_err(LuaError::external)
        });
        methods.add_method("type", |_, this: &LuaMetadata, _: ()| {
            let _type = this.0.file_type();
            if _type.is_dir() { Ok("directory") }
            else if _type.is_file() { Ok("file") }
            else if _type.is_symlink() { Ok("symlink") }
            else { Ok("unknown") }
        });
        methods.add_method("size", |_, this: &LuaMetadata, _: ()| {
            Ok(this.0.len())
        });
        methods.add_method("permissions", |_, this: &LuaMetadata, _: ()| {
            Ok(LuaPermissions(this.0.permissions()))
        });
    }
}

impl LuaUserData for LuaPermissions {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("readonly", |_, this: &LuaPermissions, _: ()| {
            Ok(this.0.readonly())
        });
        methods.add_method_mut("set_readonly", |_, this: &mut LuaPermissions, val: bool| {
            this.0.set_readonly(val);
            Ok(())
        });
        #[cfg(target_family = "unix")]
        methods.add_method("mode", |_, this: &LuaPermissions, _: ()| {
            Ok(this.0.mode())
        });
        #[cfg(target_family = "unix")]
        methods.add_method_mut("set_mode", |_, this: &mut LuaPermissions, mode: u32| {
            Ok(this.0.set_mode(mode))
        });
    }
}

pub fn init(lua: &Lua) -> crate::Result<()> {
    fs::init(&lua)?;
    time::init(&lua)?;
    path::init(&lua)?;
    env::init(&lua)?;
    dirs::init(&lua)?;
    process::init(&lua)?;
    command::init(&lua)?;
    memory::init(&lua)?;
    Ok(())
}