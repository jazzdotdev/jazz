use rlua::prelude::*;
use sass_rs::{compile_file, compile_string, Options, OutputStyle};
use std::collections::HashMap;

pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        let module = lua.create_table()?;
        module.set("compile", lua.create_function(|_, (data, opt): (String, Option<HashMap<String, LuaValue>>)| {
            let mut options = Options::default();
            let mut is_file = true;

            if let Some(option) = opt {
                for (key, val) in option.iter().map(|(k, v)| (k.as_str(), v)) {
                    match (key, val) {
                        ("file", LuaValue::Boolean(val)) => is_file = *val,
                        ("style", LuaValue::String(val)) => match val.to_str()? {
                            "expanded" => options.output_style = OutputStyle::Expanded,
                            "compact" => options.output_style = OutputStyle::Compact,
                            "compressed" => options.output_style = OutputStyle::Compressed,
                            "nested" | _ => options.output_style = OutputStyle::Nested,
                        },
                        ("precision", LuaValue::Integer(val)) => options.precision = *val as usize,
                        ("indented", LuaValue::Boolean(val)) => options.indented_syntax = *val,
                        ("paths", LuaValue::Table(val)) => options.include_paths = val.clone().sequence_values().into_iter().filter_map(Result::ok).collect(),
                        _ => (),
                    };
                }
            }

            match is_file {
                true => compile_file(&data, options).map_err(LuaError::external),
                false => compile_string(&data, options).map_err(LuaError::external)
            }

        })?)?;

        lua.globals().set("sass", module)?;

        Ok(())
    })
}