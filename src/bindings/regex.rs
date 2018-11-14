use std::sync::Arc;
use rlua::prelude::*;
use serde_json;
use rlua_serde;
use regex;

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    let module = lua.create_table()?;

    module.set("match", lua.create_function(|lua, (expr, val): (String, String)| {
        let re = regex::Regex::new(&expr).map_err(LuaError::external)?;
        Ok(re.is_match(&val))
    })?)?;

    module.set("replace_all", lua.create_function(|lua, (expr, val, patt): (String, String, String)| {
        let re = regex::Regex::new(&expr).map_err(LuaError::external)?;
        let res: String = re.replace_all(&val, patt.as_str()).into_owned();
        Ok(res)
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
            local expr = "(%d+)-(%d+)-(%d+)"
            local before = "2012-03-14, 2013-01-01 and 2014-07-05"
            local result = regex.replace_all(expr, before, "$m/$d/$y")

            print(result)
        "#, None).unwrap();
    }

    #[test]
    fn lua_regex_match () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            local expr = "(%d+)-(%d+)-(%d+)"
            local date = "2012-03-14"
            local result = regex.match(expr, date)

            print(result)
        "#, None).unwrap();
    }
}