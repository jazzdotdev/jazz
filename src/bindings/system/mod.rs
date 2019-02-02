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
        methods.add_method_mut("write", |_, this: &mut LuaCommonIO, bytes: Vec<u8>|{
            let stdin = this.clone().stdin.ok_or(LuaError::external(Error::InternalError))?;
            let mut stdin = stdin.lock().unwrap();
            stdin.write(bytes.as_slice()).map_err(LuaError::external)
        });
        methods.add_method_mut("write", |_, this: &mut LuaCommonIO, str: String|{
            let stdin = this.clone().stdin.ok_or(LuaError::external(Error::InternalError))?;
            let mut stdin = stdin.lock().unwrap();
            stdin.write(str.as_bytes()).map_err(LuaError::external)
        });
        methods.add_method_mut("flush", |_, this: &mut LuaCommonIO, _: ()|{
            let stdin = this.clone().stdin.ok_or(LuaError::external(Error::InternalError))?;
            let mut stdin = stdin.lock().unwrap();
            stdin.flush().map_err(LuaError::external)
        });

        methods.add_method_mut("read", |_, this: &mut LuaCommonIO, len: Option<usize>|{
            let stdout = this.clone().stdout.ok_or(LuaError::external(Error::InternalError))?;
            let mut stdout = stdout.lock().unwrap();
            let bytes = match len {
                Some(len) => {
                    let mut bytes = vec![0u8; len];
                    stdout.read(&mut bytes).map_err(LuaError::external)?;
                    bytes
                },
                None => {
                    let mut bytes = vec![];
                    stdout.read_to_end(&mut bytes).map_err(LuaError::external)?;
                    bytes
                }
            };
            Ok(bytes)
        });
        methods.add_method_mut("read_to_string", |_, this: &mut LuaCommonIO, _: ()|{
            let stdout = this.clone().stdout.ok_or(LuaError::external(Error::InternalError))?;
            let mut stdout = stdout.lock().unwrap();
            let mut data = String::new();
            stdout.read_to_string(&mut data).map_err(LuaError::external)?;
            Ok(data)
        });

        methods.add_method_mut("seek", |_, this: &mut LuaCommonIO, (pos, size): (Option<String>, Option<usize>)| {
            let seek = this.clone().seek.ok_or(LuaError::external(Error::InternalError))?;
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

        methods.add_method_mut("sync_all", |_, this: &mut LuaCommonIO, _: ()|{
            let fd = this.clone().inner.ok_or(LuaError::external(Error::InternalError))?;
            let fd = fd.lock().unwrap();
            fd.sync_all().map_err(LuaError::external)
        });
        methods.add_method_mut("sync_data", |_, this: &mut LuaCommonIO, _: ()|{
            let fd = this.clone().inner.ok_or(LuaError::external(Error::InternalError))?;
            let fd = fd.lock().unwrap();
            fd.sync_data().map_err(LuaError::external)
        });
        methods.add_method("metadata", |_, this: &LuaCommonIO, _: ()| {
            let fd = this.clone().inner.ok_or(LuaError::external(Error::InternalError))?;
            let fd = fd.lock().unwrap();
            fd.metadata().map(LuaMetadata).map_err(LuaError::external)
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