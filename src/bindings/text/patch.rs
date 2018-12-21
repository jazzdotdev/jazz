use rlua::{Lua, UserData, UserDataMethods};

struct PatchParser(patch_rs::PatchParser);

impl UserData for PatchParser {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("process", |_, this, _: ()| {
            this.0.process().map_err(rlua::Error::external)
        });
    }
}

pub fn init(lua: &Lua) -> Result<(), rlua::Error> {
    let module = lua.create_table()?;
    module.set(
        "new",
        lua.create_function(|_, (text, patch): (Vec<String>, String)| {
            Ok(PatchParser(patch_rs::PatchParser::new(text, patch)))
        })?,
    )?;
    let g = lua.globals();
    g.set("patch_parser", module)?;
    Ok(())
}
