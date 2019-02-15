use rlua::prelude::*;
use rlua::{UserDataMethods, UserData, MetaMethod, Lua};
use chrono::prelude::*;

#[derive(Clone)]
struct LuaTime (DateTime<Utc>);

impl UserData for LuaTime {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, _: ()| {
            Ok(this.0.to_rfc2822())
        });

        methods.add_meta_method(MetaMethod::Eq, |_, this, that: LuaTime| {
            Ok(this.0 == that.0)
        });

        methods.add_meta_method(MetaMethod::Lt, |_, this, that: LuaTime| {
            Ok(this.0 < that.0)
        });

        methods.add_meta_method(MetaMethod::Le, |_, this, that: LuaTime| {
            Ok(this.0 <= that.0)
        });
    }
}

pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        let module = lua.create_table()?;

        module.set("now", lua.create_function(|_, _: ()| {
            Ok(LuaTime(Utc::now()))
        })?)?;

        module.set("new", lua.create_function(|_, s: String| {
            DateTime::parse_from_rfc2822(&s).map(
                |t| LuaTime(t.with_timezone(&Utc))
            ).map_err(
                |_| LuaError::RuntimeError("Invalid time string".to_string())
            )
        })?)?;

        lua.globals().set("time", module)?;

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lua_time () {
        let lua = Lua::new();
        init(&lua).unwrap();

        lua.context(|lua| {
            lua.load(r#"
            print(time.now())
            print(time.new("Fri, 28 Nov 2014 12:00:09 +0000"))
        "#).exec().unwrap();

        assert!(lua.load("print(time.new('lol'))").exec().is_err());
        })

    }
}