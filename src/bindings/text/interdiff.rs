use rlua::Lua;

use std::{io, fs};
use patch_rs::{Patch, PatchProcessor};

const EMPTY_PATCH: &str = "/dev/null";

pub fn init(lua: &Lua) -> crate::Result<()> {
    let module = lua.create_table()?;

    module.set(
        "interdiff",
        lua.create_function(|_, (patch_1, patch_2): (String, String)| {
            if patch_1 == EMPTY_PATCH && patch_2 == EMPTY_PATCH {
                return Err(rlua::Error::external(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Both patches cannot be empty"
                )));
            }

            let patch_1 = if patch_1 != EMPTY_PATCH {
                let patch_1 = fs::read_to_string(patch_1).map_err(rlua::Error::external)?;
                PatchProcessor::convert(&patch_1).map_err(rlua::Error::external)?
            } else {
                Patch::default()
            };

            let patch_2 = if patch_2 != EMPTY_PATCH {
                let patch_2 = fs::read_to_string(patch_2).map_err(rlua::Error::external)?;
                PatchProcessor::convert(&patch_2).map_err(rlua::Error::external)?
            } else {
                Patch::default()
            };

            Ok(interdiff_rs::interdiff(patch_1, patch_2, 3).to_string())
        })?,
    )?;

    lua.globals().set("interdiff", module)?;

    Ok(())
}
