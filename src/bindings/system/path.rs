use rlua::prelude::*;
use crate::bindings::system::{LuaCommonIO, LuaMetadata};
use std::{
    collections::HashMap,
    fs, path,
    sync::{Mutex, Arc},
};

use globwalk::GlobWalkerBuilder;

pub struct LuaPath<P: AsRef<path::Path>>(pub P);

impl<P> LuaUserData for LuaPath<P> where P: AsRef<path::Path> {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("file_stem", |_, this: &LuaPath<P>, _:() |{
            Ok(this.0.as_ref().file_stem().map(|p| p.to_str().map(|s| s.to_string())))
        });
        methods.add_method("file_name", |_, this: &LuaPath<P>, _:() |{
            Ok(this.0.as_ref().file_name().map(|p| p.to_str().map(|s| s.to_string())))
        });
        methods.add_method("extension", |_, this: &LuaPath<P>, _:() |{
            Ok(this.0.as_ref().extension().map(|p| p.to_str().map(|s| s.to_string())))
        });
        methods.add_method("exists", |_, this: &LuaPath<P>, _:() |{
            Ok(this.0.as_ref().exists())
        });
        methods.add_method("is_dir", |_, this: &LuaPath<P>, _:() |{
            Ok(this.0.as_ref().is_dir())
        });
        methods.add_method("is_file", |_, this: &LuaPath<P>, _:() |{
            Ok(this.0.as_ref().is_file())
        });
        methods.add_method("create_file", |_, this: &LuaPath<P>, _: ()| {
            fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&this.0)
                .map(Box::new)
                .map(Mutex::new)
                .map(Arc::new)
                .map(|fs| LuaCommonIO {
                    inner: Some(fs.clone()),
                    stdin: Some(fs.clone()),
                    stdout: Some(fs.clone()),
                    stderr: None,
                    seek: Some(fs.clone())
                })
                .map_err(LuaError::external)
        });
        methods.add_method("create_dir", |_, this: &LuaPath<P>, opt: Option<bool>| {
            match opt {
                Some(true) => fs::create_dir_all(&this.0).map_err(LuaError::external),
                _ => fs::create_dir(&this.0).map_err(LuaError::external)
            }
        });
        methods.add_method("remove", |_, this: &LuaPath<P>, opt: Option<bool>| {
            let path = this.0.as_ref();
            if path.exists() {
                if path.is_file() {
                    return fs::remove_file(path).map_err(LuaError::external);
                } else if path.is_dir() {
                    return match opt {
                        Some(true) => fs::remove_dir_all(path).map_err(LuaError::external),
                        _ => fs::remove_dir(path).map_err(LuaError::external)
                    };
                }
            }
            Ok(())
        });
        methods.add_method("is_relative", |_, this: &LuaPath<P>, _:() |{
            Ok(this.0.as_ref().is_relative())
        });
        methods.add_method("is_absolute", |_, this: &LuaPath<P>, _:() |{
            Ok(this.0.as_ref().is_absolute())
        });
        methods.add_method("has_root", |_, this: &LuaPath<P>, _:() |{
            Ok(this.0.as_ref().has_root())
        });
        methods.add_method("parent", |_, this: &LuaPath<P>, _:() |{
            Ok(this.0.as_ref().parent().map(|p| p.to_path_buf()).map(LuaPath))
        });
        methods.add_method("join", |_, this: &LuaPath<P>, path: String |{
            Ok(LuaPath(this.0.as_ref().join(path)))
        });
        methods.add_method("metadata", |_, this: &LuaPath<P>, _:() |{
            Ok(LuaMetadata(this.0.as_ref().metadata().map_err(LuaError::external)?))
        });
        methods.add_method("canonicalize", |_, this: &LuaPath<P>, _:() |{
            fs::canonicalize(&this.0).map(LuaPath).map_err(LuaError::external)
        });
        methods.add_method("read_dir", |lua, this: &LuaPath<P>, _: ()| {
            match this.0.as_ref().read_dir() {
                Ok(iter) => {
                    let mut arc_iter = Arc::new(Some(iter));
                    let f = move |_, _: ()| {
                        let result = match Arc::get_mut(&mut arc_iter).expect("entries iterator is mutably borrowed") {
                            Some(iter) => iter.next().map(|entry| entry.map(|e| LuaPath(e.path())).ok()),
                            None => None
                        };
                        if result.is_none() { *Arc::get_mut(&mut arc_iter).unwrap() = None; }
                        Ok(result)
                    };
                    Ok(lua.create_function_mut(f)?)
                }, Err(err) => Err(LuaError::external(err))
            }
        });
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this: &LuaPath<P>, _: ()| {
            Ok(this.0.as_ref().to_str().map(String::from))
        });
    }
}

pub fn init(lua: &Lua) -> crate::Result<()> {
    let module = lua.create_table()?;

    module.set("new", lua.create_function( |_, path: Option<String>| {
        let mut path = path;
        if path.is_none() { path = Some(String::from(".")); }
        Ok(path.map(LuaPath))
    })? )?;

    module.set("pattern", lua.create_function( |lua, (path, patt, options): (String, Vec<String>, Option<HashMap<String, String>>)| {
        let mut glob = GlobWalkerBuilder::from_patterns(path, &patt);
        if let Some(opt) = options {
            for (key, val) in opt.iter().map(|(k, v)| (k.as_str(), v.as_str())) {
                glob = match key {
                    "case_insensitive" => glob.case_insensitive(val.parse().map_err(LuaError::external)?),
                    "contents_first" => glob.contents_first(val.parse().map_err(LuaError::external)?),
                    "follow_links" => glob.follow_links(val.parse().map_err(LuaError::external)?),
                    "max_depth" => glob.max_depth(val.parse().map_err(LuaError::external)?),
                    "max_open" => glob.max_open(val.parse().map_err(LuaError::external)?),
                    "min_depth" => glob.min_depth(val.parse().map_err(LuaError::external)?),
                    _ => glob,
                };
            }
        }
        let iter = glob.build().map_err(LuaError::external)?;
        let mut arc_iter = Arc::new(Some(iter));
        let f = move |_, _: ()| {
            let result = match Arc::get_mut(&mut arc_iter).expect("entries iterator is mutably borrowed") {
                Some(iter) => iter.next().map(|entry| entry.map(|e| LuaPath(e.into_path())).ok()),
                None => None
            };
            if result.is_none() { *Arc::get_mut(&mut arc_iter).unwrap() = None; }
            Ok(result)
        };
        lua.create_function_mut(f)
    })? )?;

    module.set("match", lua.create_function( |lua, patt: String| {
        let glob = globwalk::glob(&patt).map_err(LuaError::external)?;

        let mut arc_iter = Arc::new(Some(glob));
        let f = move |_, _: ()| {
            let result = match Arc::get_mut(&mut arc_iter).expect("entries iterator is mutably borrowed") {
                Some(iter) => iter.next().map(|entry| entry.map(|e| LuaPath(e.into_path())).ok()),
                None => None
            };
            if result.is_none() { *Arc::get_mut(&mut arc_iter).unwrap() = None; }
            Ok(result)
        };
        lua.create_function_mut(f)
    })? )?;

    lua.globals().set("path", module)?;

    Ok(())
}
