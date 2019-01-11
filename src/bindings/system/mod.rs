pub mod fs;
pub mod time;
pub mod path;

use rlua::prelude::*;
use std::fs::{Metadata, Permissions};
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

pub struct LuaMetadata(Metadata);
pub struct LuaPermissions(Permissions);

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
            else if _type.is_symlink() { Ok("syslink") }
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

    Ok(())
}