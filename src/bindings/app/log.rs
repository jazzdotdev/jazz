use rlua::prelude::*;
use rlua::{Variadic, Value};

pub fn init(lua: &Lua) -> crate::Result<()> {

    fn tostr (lua: &Lua, args: Variadic<Value>) -> LuaResult<String> {
        let f: LuaFunction = lua.globals().get("tostring")?;
        let mut s =  String::new();
        for value in args.into_iter() {
            let vs: String = f.call(value)?;
            s.push('\t');
            s.push_str(&vs);
        }
        Ok(s)
    }

    let module = lua.create_table()?;

    module.set("error", lua.create_function( |lua, args: _| {
        error!("{}", tostr(lua, args)?);
        Ok(())
    })? )?;

    module.set("warn", lua.create_function( |lua, args: _| {
        warn!("{}", tostr(lua, args)?);
        Ok(())
    })? )?;

    module.set("info", lua.create_function( |lua, args: _| {
        info!("{}", tostr(lua, args)?);
        Ok(())
    })? )?;

    module.set("debug", lua.create_function( |lua, args: _| {
        debug!("{}", tostr(lua, args)?);
        Ok(())
    })? )?;

    module.set("trace", lua.create_function( |lua, args: _| {
        trace!("{}", tostr(lua, args)?);
        Ok(())
    })? )?;

    lua.globals().set("_log", module)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lua_log () {

        let colors = ::fern::colors::ColoredLevelConfig::new();

        ::fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{}: {}", colors.color(record.level()), message
                ))
            })
            .level(::log::LevelFilter::Trace)
            .chain(::std::io::stdout())
            .apply().unwrap();

        let lua = Lua::new();
        init(&lua).unwrap();

        lua.exec::<_, ()>(r#"
            _log.info(4, "foo", nil, {})
            _log.error("Some Scary Error")
            _log.warn("Warning")
            _log.debug("Debug")
            _log.trace("Trace", "with", {}, "data")
        "#, None).unwrap();
    }
}