use rlua::prelude::*;
use std::sync::Arc;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{SeekFrom, prelude::*};
use serde_json;
use rlua_serde;
use bindings::system::LuaMetadata;
use fs_extra;
use std::path;

pub struct LuaFile(fs::File);

pub fn fs_open(_: &Lua, (path, mode): (String, Option<String>)) -> Result<LuaFile, LuaError> {
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
        .map(LuaFile)
        .map_err(LuaError::external)
}

impl LuaUserData for LuaFile {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("read", |_, this: &mut LuaFile, len: Option<usize>|{
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
        methods.add_method_mut("read_to_string", |_, this: &mut LuaFile, _: ()|{
            let mut data = String::new();
            this.0.read_to_string(&mut data).map_err(LuaError::external)?;
            Ok(data)
        });
        methods.add_method_mut("write", |_, this: &mut LuaFile, bytes: Vec<u8>|{
            Ok(this.0.write(bytes.as_slice()).map_err(LuaError::external)?)
        });
        methods.add_method_mut("write", |_, this: &mut LuaFile, str: String|{
            Ok(this.0.write(str.as_bytes()).map_err(LuaError::external)?)
        });
        methods.add_method_mut("flush", |_, this: &mut LuaFile, _: ()|{
            Ok(this.0.flush().map_err(LuaError::external)?)
        });
        methods.add_method_mut("sync_all", |_, this: &mut LuaFile, _: ()|{
            Ok(this.0.sync_all().map_err(LuaError::external)?)
        });
        methods.add_method_mut("sync_data", |_, this: &mut LuaFile, _: ()|{
            Ok(this.0.sync_data().map_err(LuaError::external)?)
        });
        methods.add_method("metadata", |_, this: &LuaFile, _: ()| {
            Ok(LuaMetadata(this.0.metadata().map_err(LuaError::external)?))
        });
        methods.add_method_mut("seek", |_, this: &mut LuaFile, (pos, size): (Option<String>, Option<usize>)| {
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

    }
}

pub fn init(lua: &Lua) -> ::Result<()> {

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
        create_symlink(src_path, symlink_dest).map_err(LuaError::external)
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
		fs::copy(src, dest)
			.map_err(LuaError::external)
	})?)?;

	// TODO custom implementation
	module.set("copy_dir", lua.create_function(|_, (src, dest): (String, String)| {
		if !path::Path::new(&dest).exists() {
			match fs::create_dir_all(&dest) {
				Ok(()) => (),
				Err(_) => error!("Could not create directory")
			}
		}
		let mut copy_options = fs_extra::dir::CopyOptions::new();
		copy_options.overwrite = true;
		fs_extra::dir::copy(src, dest, &copy_options)
			.map_err(LuaError::external)
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
#[cfg(target_family = "windows")]
fn create_symlink(src_path: String, dest: String) -> std::io::Result<()> {
    use std::os::windows::fs::symlink_file;
    symlink_file(src_path, dest)
}
#[cfg(target_family = "unix")]
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
