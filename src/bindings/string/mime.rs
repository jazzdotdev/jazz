use rlua::prelude::*;
use mime_guess;

pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        let module = lua.create_table()?;
        module.set("get_mime_type", lua.create_function(|_, ext: String| {
            Ok(mime_guess::get_mime_type(&ext).to_string())
        })?)?;

        module.set("guess_mime_type", lua.create_function(|_, path: String| {
            Ok(mime_guess::guess_mime_type(&path).to_string())
        })?)?;

        lua.globals().set("mime", module)?;

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lua_mime_get_type () {
        let lua = Lua::new();
        init(&lua).unwrap();
        lua.context(|lua| {
            lua.load(r#"
            local mime = mime.get_mime_type("gif")
            assert(mime == "image/gif")
        "#).exec().unwrap();
        })
    }

    #[test]
    fn lua_mime_guess_type () {
        let lua = Lua::new();
        init(&lua).unwrap();
        lua.context(|lua| {
            lua.load(r#"
            local mime = mime.guess_mime_type("file.txt")
            assert(mime == "text/plain")
        "#).exec().unwrap();
        })
    }
}