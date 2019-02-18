use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rlua::prelude::*;
use rlua_serde;
use tera::{Tera, Value as JsonValue, Context as TeraContext};
use crate::error::Error;

struct LuaTera (Arc<Mutex<Tera>>);

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

impl LuaUserData for LuaTera {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {

        methods.add_method("extend", |_, this, dir: String| {
            let mut tera = this.0.try_lock().unwrap();
            let new_tera = Tera::parse(&dir).map_err(|err| {
                LuaError::external(format_err!("{}", err.to_string()))
            })?;
            tera.extend(&new_tera).map_err(|err| {
                LuaError::external(format_err!("{}", err.to_string()))
            })
        });

        methods.add_method("reload", |_, this, _: ()| {
            let mut tera = this.0.try_lock().unwrap();
            tera.full_reload().map_err(|err| {
                LuaError::external(format_err!("{}", err.to_string()))
            })
        });

        methods.add_method("add_raw_templates", |_, this, hashmap: HashMap<String, String>| {
            let mut templates = Vec::new();
            for (key, val) in &hashmap {
                templates.push( (key.as_str(), val.as_str()) );
            }
            let mut tera = this.0.try_lock().unwrap();
            tera.add_raw_templates(templates).map_err(|err| {
                LuaError::external(format_err!("{}", err.to_string()))
            })
        });

        methods.add_method("render", |_, this, (path, params): (String, Option<HashMap<String, LuaValue>>)| {
            let tera = this.0.try_lock().unwrap();
            let text = match params {
                Some(params) => {
                    let context = get_tera_context_from_table(&params)?;
                    tera.render(&path, context)
                },
                None => {
                    tera.render(&path, TeraContext::new())
                },
            }.map_err(|err| {
                // https://github.com/Keats/tera/issues/381
                //crate::error::create_lua_error(err)
                LuaError::external(format_err!("{:?}", err.to_string()))
            })?;

            Ok(text)
        });
    }
}

pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        let new_tera = lua.create_function(move |_, dir: String| {
            let tera = Tera::new(&dir).unwrap();
            let arc_mutex = Arc::new(Mutex::new(tera));
            Ok(LuaTera(arc_mutex))
        })?;

        let module = lua.create_table()?;
        module.set("new", new_tera)?;
        lua.globals().set("tera", module).map_err(Error::from)?;

        Ok(())
    })
}
