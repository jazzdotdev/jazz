use rlua::prelude::*;
use regex;

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    let module = lua.create_table()?;

    module.set("match", lua.create_function(|_, (expr, val): (String, String)| {
        let re = regex::Regex::new(&expr).map_err(LuaError::external)?;
        Ok(re.is_match(&val))
    })?)?;

    module.set("replace_all", lua.create_function(|_, (expr, val, patt): (String, String, String)| {
        let re = regex::Regex::new(&expr).map_err(LuaError::external)?;
        Ok(re.replace_all(&val, patt.as_str()).into_owned())
    })?)?;

    lua.globals().set("regex", module)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lua_regex_replace_all () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            local expr = [[(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})]]
            local before = "2012-03-14, 2013-01-01 and 2014-07-05"
            local result = regex.replace_all(expr, before, "$m/$d/$y")

            assert(result == "03/14/2012, 01/01/2013 and 07/05/2014")
        "#, None).unwrap();
    }

    #[test]
    fn lua_regex_match () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            local expr = [[^\d{4}-\d{2}-\d{2}$]]
            local date = "2014-01-01"
            local result = regex.match(expr, date)

            assert(result == true)
        "#, None).unwrap();
    }
}