use rlua::prelude::*;
use heck::*;

pub struct LuaCase(String);

impl LuaUserData for LuaCase {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("to_camel", |_, this: &LuaCase, _:()| {
            Ok(this.0.to_camel_case())
        });
        methods.add_method("to_kebab", |_, this: &LuaCase, _:()| {
            Ok(this.0.to_kebab_case())
        });
        methods.add_method("to_mixed", |_, this: &LuaCase, _:()| {
            Ok(this.0.to_mixed_case())
        });
        methods.add_method("to_shouty_snake", |_, this: &LuaCase, _:()| {
            Ok(this.0.to_shouty_snake_case())
        });
        methods.add_method("to_snake", |_, this: &LuaCase, _:()| {
            Ok(this.0.to_snake_case())
        });
        methods.add_method("to_title", |_, this: &LuaCase, _:()| {
            Ok(this.0.to_title_case())
        });
    }
}

pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        let module = lua.create_table()?;

        module.set("new", lua.create_function(|_, text: String| {
            Ok(LuaCase(text))
        })?)?;

        lua.globals().set("case", module)?;

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lua_cambel () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.context(|lua| {
            lua.load(r#"
            local val = case.new("We are not in the least afraid of ruins")
            assert(val:to_camel() == "WeAreNotInTheLeastAfraidOfRuins")
            "#).exec().unwrap();
        });
    }

    #[test]
    fn lua_kebab () {
        let lua = Lua::new();
        init(&lua).unwrap();
        lua.context(|lua| {
            lua.load(r#"
            local val = case.new("We are going to inherit the earth")
            assert(val:to_kebab() == "we-are-going-to-inherit-the-earth")
        "#).exec().unwrap();
        })
    }

    #[test]
    fn lua_mixed () {
        let lua = Lua::new();
        init(&lua).unwrap();
        lua.context(|lua| {
            lua.load(r#"
            local val = case.new("It is we who built these palaces and cities")
            assert(val:to_mixed() == "itIsWeWhoBuiltThesePalacesAndCities")
        "#).exec().unwrap();
        })
    }

    #[test]
    fn lua_shouty() {
        let lua = Lua::new();
        init(&lua).unwrap();
        lua.context(|lua| {
            lua.load(r#"
            local val = case.new("That world is growing in this minute")
            assert(val:to_shouty_snake() == "THAT_WORLD_IS_GROWING_IN_THIS_MINUTE")
        "#).exec().unwrap();
        })
    }

    #[test]
    fn lua_snake () {
        let lua = Lua::new();
        init(&lua).unwrap();
        lua.context(|lua| {
            lua.load(r#"
            local val = case.new("We carry a new world here, in our hearts")
            assert(val:to_snake() == "we_carry_a_new_world_here_in_our_hearts")
        "#).exec().unwrap();
        })
    }

    #[test]
    fn lua_title () {
        let lua = Lua::new();
        init(&lua).unwrap();
        lua.context(|lua| {
            lua.load(r#"
            local val = case.new("We have always lived in slums and holes in the wall")
            assert(val:to_title() == "We Have Always Lived In Slums And Holes In The Wall")
        "#).exec().unwrap();
        })
    }
}