use rlua::prelude::*;
use std::path;
use std::sync::Arc;

pub struct LuaPath(path::PathBuf);
//TODO: Clean up metadata for `Path::metadata` (Internal #1b8b1ed373cd1901b2)
//pub struct LuaMetadata(Metadata);

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
        methods.add_method("read_dir", |lua, this: &LuaPath, _: ()| {
            match this.0.read_dir() {
                Ok(iter) => {
                    let mut arc_iter = Arc::new(Some(iter));
                    let mut f = move |_, _: ()| {
                        let result = match Arc::get_mut(&mut arc_iter).expect("entries iterator is mutably borrowed") {
                            Some(iter) => match iter.next() {
                                Some(Ok(entry)) => Some(entry.file_name().into_string().unwrap()),
                                _ => None
                            },
                            None => None
                        };
                        if result.is_none() { *Arc::get_mut(&mut arc_iter).unwrap() = None; }
                        Ok(result)
                    };
                    Ok(lua.create_function_mut(f)?)
                }, Err(err) => Err(LuaError::ExternalError(Arc::new(::failure::Error::from_boxed_compat(Box::new(err)))))
            }
        })

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
