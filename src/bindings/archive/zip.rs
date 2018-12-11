use rlua::prelude::*;
use zip::ZipArchive;
use std::fs;
use std::io;
use std::path::Path;

pub fn init(lua: &Lua) -> Result<(), LuaError> {

    let module = lua.create_table()?;
    module.set("decompress", lua.create_function(|_, (src, dst): (String, String)| {
        let zip = fs::File::open(src).map_err(LuaError::external)?;

        ZipArchive::new(zip).map_err(LuaError::external).and_then(|mut archive|{
            let path = Path::new(&dst);
            for i in 0..archive.len() {
                let mut temp = archive.by_index(i).map_err(LuaError::external)?;
                let outpath = temp.sanitized_name();
                if (&*temp.name()).ends_with('/') {
                    fs::create_dir_all(path.join(&outpath)).map_err(LuaError::external)?;
                } else {
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            fs::create_dir_all(path.join(p)).map_err(LuaError::external)?;
                        }
                    }
                    let p = path.join(&outpath);
                    fs::File::create(p)
                        .and_then(|mut output| io::copy(&mut temp, &mut output) ).map_err(LuaError::external)?;
                }

            }

            Ok(())
        })

    })?)?;

    lua.globals().set("zip", module)?;

    Ok(())
}