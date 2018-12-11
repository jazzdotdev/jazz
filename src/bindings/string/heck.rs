use rlua::prelude::*;
use heck::*;

pub fn init(lua: &Lua) -> Result<(), LuaError> {

    let module = lua.create_table()?;

    module.set("to_camel_case", lua.create_function(|_, text: String| {
        Ok(text.to_camel_case())
    })?)?;

    module.set("to_kebab_case", lua.create_function(|_, text: String| {
        Ok(text.to_kebab_case())
    })?)?;

    module.set("to_mixed_case", lua.create_function(|_, text: String| {
        Ok(text.to_mixed_case())
    })?)?;

    module.set("to_shouty_snake_case", lua.create_function(|_, text: String| {
        Ok(text.to_shouty_snake_case())
    })?)?;

    module.set("to_snake_case", lua.create_function(|_, text: String| {
        Ok(text.to_snake_case())
    })?)?;

    module.set("to_title_case", lua.create_function(|_, text: String| {
        Ok(text.to_title_case())
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
            local val = "We are not in the least afraid of ruins"
            assert(heck.to_camel_case(val) == "WeAreNotInTheLeastAfraidOfRuins")
        "#, None).unwrap();
    }

    #[test]
    fn lua_heck_kebab () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            local val = "We are going to inherit the earth"
            assert(heck.to_kebab_case(val) == "we-are-going-to-inherit-the-earth")
        "#, None).unwrap();
    }

    #[test]
    fn lua_heck_mixed () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            local val = "It is we who built these palaces and cities"
            assert(heck.to_mixed_case(val) == "itIsWeWhoBuiltThesePalacesAndCities")
        "#, None).unwrap();
    }

    #[test]
    fn lua_heck_shouty() {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            local val = "That world is growing in this minute"
            assert(heck.to_shouty_snake_case(val) == "THAT_WORLD_IS_GROWING_IN_THIS_MINUTE")
        "#, None).unwrap();
    }

    #[test]
    fn lua_heck_snake () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            local val = "We carry a new world here, in our hearts"
            assert(heck.to_snake_case(val) == "we_carry_a_new_world_here_in_our_hearts")
        "#, None).unwrap();
    }

    #[test]
    fn lua_heck_title () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            local val = "We have always lived in slums and holes in the wall"
            assert(heck.to_title_case(val) == "We Have Always Lived In Slums And Holes In The Wall")
        "#, None).unwrap();
    }
}