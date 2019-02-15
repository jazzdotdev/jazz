use rlua::prelude::*;
use base64;

pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        let module = lua.create_table()?;

        module.set("encode", lua.create_function(|_, text: String| {
            Ok(base64::encode(&text))
        })?)?;

        module.set("decode", lua.create_function(|_, text: String| {
            let val = base64::decode(&text).map_err(LuaError::external)?;
            String::from_utf8(val).map_err(LuaError::external)
        })?)?;

        lua.globals().set("base64", module)?;

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lua_base64_encode() {
        let lua = Lua::new();
        init(&lua).unwrap();
        lua.context(|lua| {
            lua.load(r#"
            local val = base64.encode("Hello, World!")
            assert(val == "SGVsbG8sIFdvcmxkIQ==")
            "#).exec().unwrap();
        })
    }

    #[test]
    fn lua_base64_decode() {
        let lua = Lua::new();
        init(&lua).unwrap();
        lua.context(|lua| {
            lua.load(r#"
            local val = base64.decode("SGVsbG8sIFdvcmxkIQ==")
            assert(val == "Hello, World!")
            "#).exec().unwrap();
        })
    }
}