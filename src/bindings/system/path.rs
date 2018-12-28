use rlua::prelude::*;
use bindings::system::LuaMetadata;
use std::{fs, path};
use std::sync::Arc;

pub struct LuaPath(path::PathBuf);

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
        methods.add_method("create_file", |_, this: &LuaPath, _: ()| {
            fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(this.0.as_path())
                .map(|_| ())
                .map_err(LuaError::external)
        });
        methods.add_method("create_dir", |_, this: &LuaPath, opt: Option<bool>| {
            match opt {
                Some(true) => fs::create_dir_all(this.0.as_path()).map_err(LuaError::external),
                _ => fs::create_dir(this.0.as_path()).map_err(LuaError::external)
            }
        });
        methods.add_method("remove", |_, this: &LuaPath, opt: Option<bool>| {
            if this.0.exists() {
                if this.0.is_file() {
                    return fs::remove_file(&this.0).map_err(LuaError::external);
                } else if this.0.is_dir() {
                    return match opt {
                        Some(true) => fs::create_dir_all(&this.0).map_err(LuaError::external),
                        _ => fs::create_dir(&this.0).map_err(LuaError::external)
                    };
                }
            }
            Ok(())
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
            Ok(this.0.push(&val))
        });
        methods.add_method("join", |_, this: &LuaPath, path: String |{
            Ok(LuaPath(this.0.join(path)))
        });
        methods.add_method("metadata", |_, this: &LuaPath, _:() |{
            Ok(LuaMetadata(this.0.metadata().map_err(LuaError::external)?))
        });
        methods.add_method("canonicalize", |_, this: &LuaPath, _:() |{
            fs::canonicalize(&this.0).map(|can| can.to_str().map(|s| s.to_string())).map_err(LuaError::external)
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
