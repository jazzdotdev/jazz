pub mod git;
pub mod log;
pub mod markdown;
pub mod tera;


use rlua::prelude::*;

#[cfg(feature = "tantivy_bindings")]
pub mod tantivy;

// Dummy modules
#[cfg(not(feature = "tantivy_bindings"))]
pub mod tantivy {
    pub fn init(_: &rlua::Lua) -> ::Result<()> { Ok(()) }
}

pub fn init(lua: &Lua) -> ::Result<()> {
    git::init(&lua)?;
    log::init(&lua)?;
    markdown::init(&lua)?;
    tantivy::init(&lua)?;
    tera::init(&lua)?;

    Ok(())
}
