use std::{
    io::{self, prelude::*},
    path::Path,
    fs::File,
};
use rlua::prelude::*;
use rlua_serde;
use serde_json::Value;
use handlebars::{self, Handlebars};
use crate::error::Error;

pub struct LuaHandlebars {
    registry: Handlebars,
    directory: Option<String>,
}

fn read_template<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut fs = File::open(path)?;
    let mut data = String::new();
    fs.read_to_string(&mut data)?;
    Ok(data)
}

impl LuaUserData for LuaHandlebars {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {

        methods.add_method_mut("register_template_file", |_, this: &mut LuaHandlebars, (name, file): (String, String)| {
            let path = Path::new(match &this.directory {
                Some(dir) => dir.as_str(),
                None => "."
            });
            this.registry.register_template_file(&name, path.join(&file)).map_err(LuaError::external)
        });
        methods.add_method_mut("register_template_string", |_, this: &mut LuaHandlebars, (name, file): (String, String)| {
            this.registry.register_template_string(&name,file).map_err(LuaError::external)
        });
        methods.add_method("render", |_, this, (name, params): (String, Option<LuaValue>)| {
            let param = rlua_serde::from_value::<Value>(params.unwrap_or(LuaValue::Nil))?;
            this.registry.render(&name, &param).map_err(LuaError::external)
        });
        methods.add_method("render_string", |_, this: &LuaHandlebars, (template, params): (String, Option<LuaValue>)| {
            let param = rlua_serde::from_value::<Value>(params.unwrap_or(LuaValue::Nil))?;
            this.registry.render_template(&template, &param).map_err(LuaError::external)
        });
        methods.add_method("render_template", |_, this: &LuaHandlebars, (template, params): (String, Option<LuaValue>)| {

            let path = Path::new(match &this.directory {
                Some(dir) => dir.as_str(),
                None => "."
            });

            let template = read_template(path.join(&template)).map_err(LuaError::external)?;

            let param = rlua_serde::from_value::<Value>(params.unwrap_or(LuaValue::Nil))?;

            this.registry.render_template(&template, &param).map_err(LuaError::external)
        });
    }
}

pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        let module = lua.create_table()?;
        module.set("new", lua.create_function(|_, _directory: Option<String>| {
            Ok(LuaHandlebars { registry: Handlebars::new(), directory: _directory})
        })?)?;

        lua.globals().set("handlebars", module).map_err(Error::from)?;

        Ok(())
    })
}
