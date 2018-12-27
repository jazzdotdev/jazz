use rlua::prelude::*;
use std::path;
use std::sync::Arc;
use std::fs::{Permissions, Metadata};
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
#[cfg(target_family = "unix")]
use std::os::unix::fs::MetadataExt;

pub struct LuaPath(path::PathBuf);
pub struct LuaMetadata(Metadata);
pub struct LuaPermissions(Permissions);

impl LuaUserData for LuaPath {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("file_stem", |_, this: &LuaPath, _:() |{
            Ok(this.0.file_stem().map(|p| p.to_str().map(|s| s.to_string())))
        });
        methods.add_method("file_name", |_, this: &LuaPath, _:() |{
            Ok(this.0.file_name().map(|p| p.to_str().map(|s| s.to_string())))
        });
        methods.add_method("ext", |_, this: &LuaPath, _:() |{
            Ok(this.0.extension().map(|p| p.to_str().map(|s| s.to_string())))
        });
        methods.add_method("exists", |_, this: &LuaPath, _:() |{
            Ok(this.0.exists())
        });
        methods.add_method("is_dir", |_, this: &LuaPath, _:() |{
            Ok(this.0.is_dir())
        });
        methods.add_method("is_file", |_, this: &LuaPath, _:() |{
            Ok(this.0.is_file())
        });
        methods.add_method("is_relative", |_, this: &LuaPath, _:() |{
            Ok(this.0.is_relative())
        });
        methods.add_method("is_absolute", |_, this: &LuaPath, _:() |{
            Ok(this.0.is_absolute())
        });
        methods.add_method("has_root", |_, this: &LuaPath, _:() |{
            Ok(this.0.has_root())
        });
        methods.add_method("parent", |_, this: &LuaPath, _:() |{
            Ok(this.0.parent().map(|p| LuaPath(p.to_path_buf())))
        });
        methods.add_method_mut("push", |_, this: &mut LuaPath, val: String |{
            this.0.push(&val);
            Ok(())
        });
        methods.add_method("join", |_, this: &LuaPath, path: String |{
            Ok(LuaPath(this.0.join(path)))
        });
        methods.add_method("metadata", |_, this: &LuaPath, _:() |{
            Ok(LuaMetadata(this.0.metadata().map_err(LuaError::external)?))
        });
        methods.add_method("read_dir", |lua, this: &LuaPath, _: ()| {
            match this.0.read_dir() {
                Ok(iter) => {
                    let mut arc_iter = Arc::new(Some(iter));
                    let mut f = move |_, _: ()| {
                        let result = match Arc::get_mut(&mut arc_iter).expect("entries iterator is mutably borrowed") {
                            Some(iter) => iter.next().map(|entry| entry.map(|e| e.file_name().into_string().unwrap()).ok()),
                            None => None
                        };
                        if result.is_none() { *Arc::get_mut(&mut arc_iter).unwrap() = None; }
                        Ok(result)
                    };
                    Ok(lua.create_function_mut(f)?)
                }, Err(err) => Err(LuaError::external(err))
            }
        });
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this: &LuaPath, _: ()| {
            Ok(this.0.to_str().map(|s| s.to_string()))
        });
    }
}

impl LuaUserData for LuaMetadata {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("created", |_, this: &LuaMetadata, _: ()| {
            Ok(this.0.created().map(|time| time.duration_since(::std::time::SystemTime::UNIX_EPOCH).map(|s| s.as_secs()).unwrap_or(0)).ok())
        });
        methods.add_method("modified", |_, this: &LuaMetadata, _: ()| {
            Ok(this.0.created().map(|time| time.duration_since(::std::time::SystemTime::UNIX_EPOCH).map(|s| s.as_secs()).unwrap_or(0)).ok())
        });
        methods.add_method("accessed", |_, this: &LuaMetadata, _: ()| {
            Ok(this.0.created().map(|time| time.duration_since(::std::time::SystemTime::UNIX_EPOCH).map(|s| s.as_secs()).unwrap_or(0)).ok())
        });
        methods.add_method("type", |_, this: &LuaMetadata, _: ()| {
            let _type = this.0.file_type();
            if _type.is_dir() { Ok("directory") }
            else if _type.is_file() { Ok("file") }
            else if _type.is_symlink() { Ok("syslink") }
            else { Ok("unknown") }
        });
        #[cfg(target_family = "unix")]
        methods.add_method("mode", |_, this: &LuaMetadata, _: ()| {
            Ok(this.0.mode() as u8)
        });
        #[cfg(target_family = "unix")]
        methods.add_method("set_mode", |_, this: &LuaMetadata, _: ()| {
            Ok(this.0.mode() as u8)
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
            Ok(this.0.mode() as u8)
        });
        methods.add_method("set_mode", |_, this: &LuaPermissions, _: ()| {
            Ok(this.0.mode() as u8)
        });
    }
}

pub fn init(lua: &Lua) -> ::Result<()> {
    let module = lua.create_table()?;

    module.set("empty", lua.create_function( |_, _: ()| {
        Ok(LuaPath(path::PathBuf::new()))
    })? )?;

    module.set("new", lua.create_function( |_, path: String| {
        Ok(LuaPath(path::Path::new(&path).to_path_buf()))
    })? )?;

    lua.globals().set("path", module)?;

    Ok(())
}
