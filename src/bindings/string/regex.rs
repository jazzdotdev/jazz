use rlua::prelude::*;
use regex;

pub struct LuaRegex(regex::Regex);
pub struct LuaCapture<'a>(regex::Captures<'a>);

impl LuaUserData for LuaRegex {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("match", |_, this: &LuaRegex, val: String| {
            Ok(this.0.is_match(&val))
        });
        methods.add_method("replace_all", |_, this: &LuaRegex, (val, patt): (String, String)| {
            Ok(this.0.replace_all(&val, patt.as_str()).into_owned())
        });
        methods.add_method("capture", |_, this: &LuaRegex, val: String| {
            Ok(this.0.captures(Box::leak(val.into_boxed_str())).map(LuaCapture))
        });
    } 
}

impl<'a> LuaUserData for LuaCapture<'a> {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get", |_, this: &LuaCapture, index: usize| {
            Ok(this.0.get(index).map(|s| s.as_str().to_string()))
        });
        methods.add_method("name", |_, this: &LuaCapture, name: String| {
            Ok(this.0.name(&name).map(|s| s.as_str().to_string()))
        });
        methods.add_method("to_table", |_, this: &LuaCapture, _: ()| {
            Ok(this.0.iter().filter_map(|s| s).map(|s| s.as_str().to_string()).collect::<Vec<String>>())
        })
    }
}

pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        let module = lua.create_table()?;

        module.set("new", lua.create_function(|_, expr: String| {
            regex::Regex::new(&expr).map(LuaRegex).map_err(LuaError::external)
        })?)?;

        lua.globals().set("regex", module)?;

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lua_regex_replace_all () {
        let lua = Lua::new();
        init(&lua).unwrap();
        lua.context(|lua| {
            lua.load(r#"
            local regex = regex.new([[(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})]])
            local before = "2012-03-14, 2013-01-01 and 2014-07-05"
            local result = regex:replace_all(before, "$m/$d/$y")

            assert(result == "03/14/2012, 01/01/2013 and 07/05/2014")
        "#).exec().unwrap();
        })
    }

    #[test]
    fn lua_regex_match () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.context(|lua| {
            lua.load(r#"
            local regex = regex.new([[^\d{4}-\d{2}-\d{2}$]])
            local date = "2014-01-01"
            local result = regex:match(date)

            assert(result == true)
        "#).exec().unwrap();
        })
    }

    #[test]
    fn lua_regex_captures () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.context(|lua| {
            lua.load(r#"
            local regex = regex.new([['([^']+)'\s+\((\d{4})\)]])
            local val = "Not my favorite movie: 'Citizen Kane' (1941)"
            local result = regex:capture(val):to_table()

            assert(result ~= nil)
            assert(result[1] == "'Citizen Kane' (1941)")
            assert(result[2] == "Citizen Kane")
            assert(result[3] == "1941")
        "#).exec().unwrap();
        })
    }
}