use std::collections::HashMap;
use std::sync::Arc;
use rlua::prelude::*;
use rlua_serde;
use tera::{Tera, Value as JsonValue, Context as TeraContext};

fn get_tera_context_from_table(table: &HashMap<String, LuaValue>) -> Result<TeraContext, LuaError> {
    let mut context = TeraContext::new();

    for (key, value) in table.iter() {
        match value {
            LuaValue::Integer(num) => context.add(key, num),
            LuaValue::Number(num) => context.add(key, num),
            LuaValue::String(string) => context.add(key, string.to_str()?),
            LuaValue::Boolean(boolean) => context.add(key, boolean),
            value @ LuaValue::Table(_) => {
                let value: JsonValue = rlua_serde::from_value(value.clone())
                    .map_err(|err| LuaError::external(err))?;
                context.add(key, &value);
            },
            LuaValue::Nil => context.add(key, &()),
            value @ _ => unimplemented!("Value {:?} is not implemented as a template parameter", value),
        }
    }

    Ok(context)
}

pub fn init(lua: &Lua, tera: Arc<Tera>) -> Result<(), LuaError> {
    let render_template = lua.create_function(move |_, (path, params): (String, Option<HashMap<String, LuaValue>>)| {
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

    let globals = lua.globals();
    globals.set("render", render_template)?;

    Ok(())
}
