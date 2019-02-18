use rlua::{self, Lua, UserData, UserDataMethods};

struct SplitDiff(splitdiff_rs::SplitDiff);

impl UserData for SplitDiff {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("data", |_, this, _: ()| {
            let data = this.0.process().map_err(|e| rlua::Error::external(crate::error::Error::SplitDiffError(e)))?;
            let mut output = Vec::new();
            for (i, (path, patches)) in data.0.iter().enumerate() {
                output.push(format!("File {}: {}", i + 1, path));
                for (j, patch) in patches.iter().enumerate() {
                    output.push(format!("Patch {}:", j + 1));
                    for line in patch.iter() {
                        output.push(format!("{}", line));
                    }
                }
            }
            Ok(output)
        });
    }
}

pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        let module = lua.create_table()?;
        module.set(
            "new",
            lua.create_function(|_, patch: String| {
                Ok(SplitDiff(splitdiff_rs::SplitDiff::new(&patch)))
            })?,
        )?;
        let g = lua.globals();
        g.set("splitdiff", module)?;
        Ok(())
    })
}
