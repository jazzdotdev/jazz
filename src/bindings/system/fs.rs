use rlua::prelude::*;
use std::{
    sync::{Mutex, Arc},
    env,
    fs::{self, OpenOptions},
    io,
    path::Path,
};
#[cfg(target_family = "windows")]
use std::os::windows::fs::symlink_file as symlink;
#[cfg(target_family = "unix")]
use std::os::unix::fs::symlink;

use serde_json;
use rlua_serde;
use crate::bindings::system::LuaCommonIO;
use regex::Regex;

//TODO: Move to having a common interface so IO can share the same binding
pub struct LuaFile(pub fs::File);

pub fn fs_open(_: &Lua, (path, mode): (String, Option<String>)) -> Result<LuaCommonIO, LuaError> {
    let mut option = OpenOptions::new();
    if let Some(mode) = mode {
        match mode.as_ref() {
            "r" => option.read(true).write(false),
            "w" => option.create(true).read(false).write(true),
            "w+" => option.create(true).read(true).write(true).truncate(true),
            "a" => option.append(true),
            "rw" | _ => option.create(true).read(true).write(true),
        };
    } else {
        option.create(true).read(true).write(true);
    }

    option.open(path)
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

}

pub fn init(lua: &Lua) -> crate::Result<()> {

    let module = lua.create_table()?;

    module.set("open", lua.create_function( fs_open)? )?;

    module.set("canonicalize", lua.create_function( |lua, path: String| {
        match fs::canonicalize(path).map_err(|err| LuaError::external(err)) {
            Ok(i) => Ok(Some(lua.create_string(&i.to_str().unwrap()).unwrap())),
            _ => Ok(None)
        }
    })? )?;

    //Deprecated for path:create_dir
    module.set("create_dir", lua.create_function( |_, (path, all): (String, Option<bool>)| {
        let result = match all {
            Some(true) => fs::create_dir_all(path),
            _ => fs::create_dir(path)
        };
        Ok(result.is_ok())
    })? )?;

    //Deprecated for path:read_dir
    module.set("entries", lua.create_function( |lua, path: String| {
        match fs::read_dir(path) {
            Ok(iter) => {
                let mut arc_iter = Arc::new(Some(iter));
                let f = move |_, _: ()| {
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

    ////Deprecated for fs:read
    module.set("read_file", lua.create_function( |lua, path: String| {
        let data = fs::read(path).map_err(|err| LuaError::external(err))?;
        Ok(lua.create_string(&String::from_utf8_lossy(&data[..]).to_owned().to_string())?)
    })?)?;

    module.set("chdir", lua.create_function(|_, path: String| {
        env::set_current_dir(path).map_err(LuaError::external)
    })?)?;

    module.set("current_dir", lua.create_function(|_, _:()| {
        env::current_dir().map(|path| path.to_str().map(|s| s.to_string())).map_err(LuaError::external)
    })?)?;

    //Probably deprecate for path:exists
    module.set("exists", lua.create_function( |_, path: String| {
        Ok(::std::path::Path::new(&path).exists())
    })?)?;

    //Probably deprecate for path:is_file
    module.set("is_file", lua.create_function( |_, path: String| {
        Ok(::std::path::Path::new(&path).is_file())
    })?)?;

    //Probably deprecate for path:is_dir
    module.set("is_dir", lua.create_function( |_, path: String| {
        Ok(::std::path::Path::new(&path).is_dir())
    })?)?;

    module.set("symlink", lua.create_function( |_, (src_path, symlink_dest): (String, String)| {
        symlink(src_path, symlink_dest).map_err(LuaError::external)
    })?)?;

    //Probably deprecate for path:remove
    module.set("remove_dir", lua.create_function( |_, (path, all): (String, Option<bool>)| {
        match all {
            Some(true) => fs::remove_dir_all(&path).map_err(LuaError::external),
            _ => fs::remove_dir(&path).map_err(LuaError::external)
        }
    })?)?;

    //TODO: Rename to something suitable other than touch
    //Probably deprecate for path:create_file
    module.set("touch", lua.create_function( |_, path: String| {
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .map(|_| ())
            .map_err(LuaError::external)
    })?)?;

	module.set("copy_file", lua.create_function(|_, (src, dest): (String, String)| {
		copy_file(src, dest)
	})?)?;

	// This binding has a known side effect that this doesn't copy .git directory
	module.set("copy_dir", lua.create_function(|_, (src, dest): (String, String)| {
		recursive_copy(src, dest).map_err(LuaError::external) 
	})?)?; 

    //Deprecated for fs:metadata
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

                table.set("created", md.created().map(|time| time.duration_since(::std::time::SystemTime::UNIX_EPOCH).map(|s| s.as_secs()).unwrap_or(0)).ok())?;
                table.set("accessed", md.accessed().map(|time| time.duration_since(::std::time::SystemTime::UNIX_EPOCH).map(|s| s.as_secs()).unwrap_or(0)).ok())?;
                table.set("modified", md.modified().map(|time| time.duration_since(::std::time::SystemTime::UNIX_EPOCH).map(|s| s.as_secs()).unwrap_or(0)).ok())?;
                Ok(Some(table))
            },
            _ => Ok(None)
        }
    })? )?;

    lua.globals().set("fs", module)?;

    Ok(())
}

//TODO: Have it set to use either `syslink_file` or `syslink_dir` depending on if the endpoint is a file or directory in the `src_path`
//      Probably move functions into path binding.

fn copy_file<S: AsRef<Path>, D: AsRef<Path>>(src: S, dest: D) -> LuaResult<()> {
	let mut dest = dest.as_ref().to_path_buf();
    if dest.is_dir() {
		let file_name = src.as_ref()
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .ok_or(LuaError::external(io::Error::from(io::ErrorKind::InvalidInput)))?;
		dest.push(file_name);
    };
    fs::copy(src, dest).map(|_| ())
        .map_err(LuaError::external)
}

fn recursive_copy<A: AsRef<Path>, B: AsRef<Path>>(src: A, dest: B) -> io::Result<()> {
    let path = src.as_ref();
    if !src.as_ref().exists() {
       return Err(io::Error::from(io::ErrorKind::NotFound));
    }
    if !dest.as_ref().exists() {
        fs::create_dir(&dest)?;
    }
    for entry in path.read_dir()? {
        let src = entry.map(|e| e.path())?;
        let src_name = match src.file_name().map(|s| s.to_string_lossy().to_string()) {
            Some(s) => s,
            None => return Err(io::Error::from(io::ErrorKind::InvalidData))
        }; 
		let re = Regex::new(r"^\.git").unwrap();
		// don't copy .git directory
		if re.is_match(&src_name) { 
			continue;
		}
        let dest = dest.as_ref().join(src_name); 
        if src.is_file() {
            fs::copy(src, &dest)?;
        } 
		else {
            fs::create_dir_all(&dest)?;
            recursive_copy(src, &dest)?;
        }
    }
    Ok(())
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
