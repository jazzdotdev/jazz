pub mod tera;
pub mod yaml;
pub mod json;
pub mod uuid;
pub mod markdown;
pub mod client;
pub mod crypto;
pub mod stringset;
pub mod server;
pub mod time;
pub mod fs;
pub mod select;
pub mod git;
pub mod regex;
pub mod mime;
pub mod scl;
pub mod heck;
pub mod zip;
pub mod tar;

// Panics if not included (?)
//#[cfg(feature = "log_bindings")]
pub mod log;

#[cfg(feature = "tantivy_bindings")]
pub mod tantivy;


// Dummy modules

#[cfg(not(feature = "tantivy_bindings"))]
pub mod tantivy {
    pub fn init(_: &rlua::Lua) -> rlua::Result<()> { Ok(()) }
}

