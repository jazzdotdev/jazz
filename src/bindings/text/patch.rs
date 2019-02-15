use rlua::{Lua, UserData, UserDataMethods};

struct PatchProcessor(patch_rs::PatchProcessor);

impl UserData for PatchProcessor {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("process", |_, this, _: ()| {
            this.0.process().map_err(crate::error::Error::PatchError).map_err(rlua::Error::external)
        });
    }
}

pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        let module = lua.create_table()?;
        module.set(
            "new",
            lua.create_function(|_, (text, patch): (Vec<String>, String)| {
                patch_rs::PatchProcessor::converted(text, &patch).map(PatchProcessor).map_err(crate::error::Error::PatchError).map_err(rlua::Error::external)
            })?,
        )?;
        let g = lua.globals();
        g.set("patch_processor", module)?;
        Ok(())
    })
}
