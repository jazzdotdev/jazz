use rlua::Lua;

use std::{io, fs};
use patch_rs::{Patch, PatchProcessor};

use super::NULL_SOURCE;

pub fn init(lua: &Lua) -> crate::Result<()> {
    let module = lua.create_table()?;

    module.set(
        "combinediff",
        lua.create_function(|_, (patch_1, patch_2): (String, String)| {
            if patch_1 == NULL_SOURCE && patch_2 == NULL_SOURCE {
                return Err(rlua::Error::external(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Both patches cannot be null"
                )));
            }

            let patch_1 = if patch_1 != NULL_SOURCE {
                let patch_1 = fs::read_to_string(patch_1).map_err(rlua::Error::external)?;
                PatchProcessor::convert(&patch_1).map_err(rlua::Error::external)?
            } else {
                Patch::default()
            };

            let patch_2 = if patch_2 != NULL_SOURCE {
                let patch_2 = fs::read_to_string(patch_2).map_err(rlua::Error::external)?;
                PatchProcessor::convert(&patch_2).map_err(rlua::Error::external)?
            } else {
                Patch::default()
            };

            Ok(combinediff_rs::combinediff(patch_1, patch_2, 3).to_string())
        })?,
    )?;

    lua.globals().set("combinediff", module)?;

    Ok(())
}
