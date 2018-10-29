#![warn(warnings)]

use rlua::prelude::*;
use rlua::{UserDataMethods, UserData, MetaMethod, Value, Table, Lua, FromLua};
use select;
use std::mem;
use select::node::Raw;

struct Document(select::document::Document);

fn into_send(raw: &mut Raw) {
    use select::node::Data;
    match raw.data {
        Data::Text(ref mut tendril) => {
            *tendril = unsafe { mem::transmute(tendril.clone().into_send()) };
        }
        Data::Comment(ref mut tendril) => {
            *tendril = unsafe { mem::transmute(tendril.clone().into_send()) };
        }
        Data::Element(_, ref mut vec) => {
            for (_, tendril) in vec {
                *tendril = unsafe { mem::transmute(tendril.clone().into_send()) };
            }
        }
    }
}

impl Document {
    fn from_str(text: &str) -> Document {
        let mut doc = select::document::Document::from(text);
        for raw in &mut doc.nodes {
            into_send(raw);
        }
        Document(doc)
    }
}

unsafe impl Send for Document {}

impl UserData for Document {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    }
}

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    let select = lua.create_table()?;
    
    // New Document from string
    select.set("document", lua.create_function(|lua, text: String| {
        Ok(Document::from_str(text.as_str()))
    })?)?;

    let globals = lua.globals();
    globals.set("select", select);

    Ok(())
}

#[cfg(test)]
mod tests {
    use rlua::prelude::*;
    use rlua::{UserDataMethods, UserData, MetaMethod, Value, Table, Lua, FromLua};

    #[test]
    fn test() {
        let lua = Lua::new();
        super::init(&lua).unwrap();
        lua.exec::<_, Value>(r#"
        local doc = select.document("<p>text</p>")
        "#, None).unwrap();
    }
}
