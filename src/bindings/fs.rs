use rlua::prelude::*;
use std::sync::Arc;
use std::fs;
    
//fn metadata_table (::std::DirEntry)

pub fn init(lua: &Lua) -> Result<(), LuaError> {

    let module = lua.create_table()?;

    module.set("create_dir", lua.create_function( |_, (path, all): (String, Option<bool>)| {
        let result = match all {
            Some(true) => fs::create_dir_all(path),
            _ => fs::create_dir(path)
        };
        Ok(result.is_ok())
    })? )?;

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

    module.set("exists", lua.create_function( |_, path: String| {
        Ok(::std::path::Path::new(&path).exists())
    })?)?;

    module.set("is_file", lua.create_function( |_, path: String| {
        Ok(::std::path::Path::new(&path).is_file())
    })?)?;

    module.set("is_dir", lua.create_function( |_, path: String| {
        Ok(::std::path::Path::new(&path).is_dir())
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
        "#, None).unwrap();
    }
}