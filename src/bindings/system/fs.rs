use rlua::prelude::*;
use std::sync::Arc;
use std::env;
use std::fs;
use serde_json;
use rlua_serde;

pub fn init(lua: &Lua) -> ::Result<()> {

    let module = lua.create_table()?;

    module.set("canonicalize", lua.create_function( |lua, path: String| {
        match fs::canonicalize(path).map_err(|err| LuaError::external(err)) {
            Ok(i) => Ok(Some(lua.create_string(&i.to_str().unwrap()).unwrap())),
            _ => Ok(None)
        }
    })? )?;

    module.set("create_dir", lua.create_function( |_, (path, all): (String, Option<bool>)| {
        let result = match all {
            Some(true) => fs::create_dir_all(path),
            _ => fs::create_dir(path)
        };
        Ok(result.is_ok())
    })? )?;

    //TODO: 'fs.entries' works sometimes, while at random it fails to work and returns
    //      a nil value in test as well as in actual use
    //      Sort out and correct the issues in relations to this problem
    //      Possibly related to https://github.com/foundpatterns/torchbear/issues/84
    module.set("entries", lua.create_function( |lua, path: String| {
        match fs::read_dir(path) {
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
    })? )?;

    module.set("read_dir", lua.create_function( |lua, path: String| {
        let mut _list: Vec<String> = Vec::new();
        for entry in fs::read_dir(path).map_err(|err| LuaError::external(err))? {
            let entry = entry.map_err(|err| LuaError::external(err))?;
            _list.push(entry.path().file_name().unwrap_or_default().to_string_lossy().to_string());      
        }
        let list_value: serde_json::Value = serde_json::to_value(_list).map_err(|err| LuaError::external(err) )?;
        let lua_value = rlua_serde::to_value(lua, &list_value)?;
        Ok(lua_value)
    })?)?;

    module.set("read_file", lua.create_function( |lua, path: String| {
        let data = fs::read(path).map_err(|err| LuaError::external(err))?;
        Ok(lua.create_string(&String::from_utf8_lossy(&data[..]).to_owned().to_string())?)
    })?)?;

    module.set("chdir", lua.create_function(|_, path: String| {
        env::set_current_dir(path).map_err(LuaError::external)
    })?)?;

    module.set("exists", lua.create_function( |_, path: String| {
        Ok(::std::path::Path::new(&path).exists())
    })?)?;

    module.set("is_file", lua.create_function( |_, path: String| {
        Ok(::std::path::Path::new(&path).is_file())
    })?)?;

    module.set("is_dir", lua.create_function( |_, path: String| {
        Ok(::std::path::Path::new(&path).is_dir())
    })?)?;

    module.set("symlink", lua.create_function( |_, (src_path, symlink_dest): (String, String)| {
        create_symlink(src_path, symlink_dest).map_err(|err| LuaError::external(err))
    })?)?;

    module.set("metadata", lua.create_function( |lua, path: String| {
        match fs::metadata(path) {
            Ok(md) => {
                let table = lua.create_table()?;

                table.set("type", {
                    let file_type = md.file_type();
                    if file_type.is_file() { "file" }
                    else if file_type.is_dir() { "directory" }
                    else { unreachable!() }
                })?;

                table.set("size", md.len())?;

                // TODO: Unix permissions when in Unix
                table.set("readonly", md.permissions().readonly())?;

                // TODO: modified, created and accesed timestamps
                
                Ok(Some(table))
            },
            _ => Ok(None)
        }
    })? )?;

    lua.globals().set("fs", module)?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn create_symlink(src_path: String, dest: String) -> std::io::Result<()> {
    use std::os::windows::fs::symlink_file;
    symlink_file(src_path, dest)
}
#[cfg(not(target_os = "windows"))]
fn create_symlink(src_path: String, dest: String) -> std::io::Result<()> {
    use std::os::unix::fs::symlink;
    symlink(src_path, dest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lua_fs () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            for entry in fs.entries("./") do
                local md = fs.metadata(entry)
                print(md.type .. ": " .. entry)
            end

            assert(fs.canonicalize("."), "expected path")
            assert(fs.canonicalize("/no/such/path/here") == nil, "expected nil")
        "#, None).unwrap();
    }
}
