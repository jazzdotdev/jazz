use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rlua::prelude::*;
use rlua_serde;
use tera::{Tera, Value as JsonValue, Context as TeraContext};

fn get_tera_context_from_table(table: &HashMap<String, LuaValue>) -> Result<TeraContext, LuaError> {
    let mut context = TeraContext::new();

    for (key, value) in table.iter() {
        match value {
            LuaValue::Integer(num) => context.insert(key, num),
            LuaValue::Number(num) => context.insert(key, num),
            LuaValue::String(string) => context.insert(key, string.to_str()?),
            LuaValue::Boolean(boolean) => context.insert(key, boolean),
            value @ LuaValue::Table(_) => {
                let value: JsonValue = rlua_serde::from_value(value.clone())
                    .map_err(|err| LuaError::external(err))?;
                context.insert(key, &value);
            },
            LuaValue::Nil => context.insert(key, &()),
            value @ _ => unimplemented!("Value {:?} is not implemented as a template parameter", value),
        }
    }

    Ok(context)
}

pub fn init(lua: &Lua, _tera: Arc<Mutex<Tera>>) -> Result<(), LuaError> {

    let tera = _tera.clone();
    let render_template = lua.create_function(move |_, (path, params): (String, Option<HashMap<String, LuaValue>>)| {
        let tera = tera.try_lock().unwrap();
        let text = match params {
            Some(params) => {
                let mut context = get_tera_context_from_table(&params)?;
                tera.render(&path, &context)
            },
            None => {
                tera.render(&path, &())
            },
        }.map_err(|err| {
            // can't convert error_chain to failure directly
            LuaError::external(format_err!("{}", err.to_string()))
        })?;

        Ok(text)
    })?;

    let tera = _tera.clone();
    let extend = lua.create_function(move |_, dir: String| {
        let mut tera = tera.try_lock().unwrap();
        let new_tera = Tera::parse(&dir).map_err(|err| {
            LuaError::external(format_err!("{}", err.to_string()))
        })?;
        tera.extend(&new_tera).map_err(|err| {
            LuaError::external(format_err!("{}", err.to_string()))
        })
    })?;

    let tera = _tera.clone();
    let reload = lua.create_function(move |_, _: ()| {
        let mut tera = tera.try_lock().unwrap();
        tera.full_reload().map_err(|err| {
            LuaError::external(format_err!("{}", err.to_string()))
        })
    })?;

    let globals = lua.globals();
    globals.set("render", render_template.clone())?;

    let module = lua.create_table()?;
    module.set("render", render_template)?;
    module.set("extend", extend)?;
    module.set("reload", reload)?;

    globals.set("tera", module)?;


    Ok(())
}
