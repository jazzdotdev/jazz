use rlua::prelude::*;
use heck::*;

pub struct LuaHeck(String);

impl LuaUserData for LuaHeck {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("to_camel_case", |_, this: &LuaHeck, _:()| {
            Ok(this.0.to_camel_case())
        });
        methods.add_method("to_kebab_case", |_, this: &LuaHeck, _:()| {
            Ok(this.0.to_kebab_case())
        });
        methods.add_method("to_mixed_case", |_, this: &LuaHeck, _:()| {
            Ok(this.0.to_mixed_case())
        });
        methods.add_method("to_shouty_snake_case", |_, this: &LuaHeck, _:()| {
            Ok(this.0.to_shouty_snake_case())
        });
        methods.add_method("to_snake_case", |_, this: &LuaHeck, _:()| {
            Ok(this.0.to_snake_case())
        });
        methods.add_method("to_title_case", |_, this: &LuaHeck, _:()| {
            Ok(this.0.to_title_case())
        });
    }
}

pub fn init(lua: &Lua) -> crate::Result<()> {

    let module = lua.create_table()?;

    module.set("new", lua.create_function(|_, text: String| {
        Ok(LuaHeck(text))
    })?)?;

    lua.globals().set("heck", module)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lua_heck_cambel () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            local val = heck.new("We are not in the least afraid of ruins")
            assert(val:to_camel_case() == "WeAreNotInTheLeastAfraidOfRuins")
        "#, None).unwrap();
    }

    #[test]
    fn lua_heck_kebab () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            local val = heck.new("We are going to inherit the earth")
            assert(val:to_kebab_case() == "we-are-going-to-inherit-the-earth")
        "#, None).unwrap();
    }

    #[test]
    fn lua_heck_mixed () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            local val = heck.new("It is we who built these palaces and cities")
            assert(val:to_mixed_case() == "itIsWeWhoBuiltThesePalacesAndCities")
        "#, None).unwrap();
    }

    #[test]
    fn lua_heck_shouty() {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            local val = heck.new("That world is growing in this minute")
            assert(val:to_shouty_snake_case() == "THAT_WORLD_IS_GROWING_IN_THIS_MINUTE")
        "#, None).unwrap();
    }

    #[test]
    fn lua_heck_snake () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            local val = heck.new("We carry a new world here, in our hearts")
            assert(val:to_snake_case() == "we_carry_a_new_world_here_in_our_hearts")
        "#, None).unwrap();
    }

    #[test]
    fn lua_heck_title () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            local val = heck.new("We have always lived in slums and holes in the wall")
            assert(val:to_title_case() == "We Have Always Lived In Slums And Holes In The Wall")
        "#, None).unwrap();
    }
}