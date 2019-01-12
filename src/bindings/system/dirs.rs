use rlua::prelude::*;
use dirs;
use crate::bindings::system::path::LuaPath;
pub fn init(lua: &Lua) -> crate::Result<()> {
    let module = lua.create_table()?;

    module.set("home", lua.create_function( |_, _: ()| {
        Ok(dirs::home_dir().map(LuaPath))
    })? )?;
    module.set("audio", lua.create_function( |_, _: ()| {
        Ok(dirs::audio_dir().map(LuaPath))
    })? )?;
    module.set("config", lua.create_function( |_, _: ()| {
        Ok(dirs::config_dir().map(LuaPath))
    })? )?;
    module.set("cache", lua.create_function( |_, _: ()| {
        Ok(dirs::cache_dir().map(LuaPath))
    })? )?;
    module.set("data", lua.create_function( |_, _: ()| {
        Ok(dirs::data_dir().map(LuaPath))
    })? )?;
    module.set("data_local", lua.create_function( |_, _: ()| {
        Ok(dirs::data_local_dir().map(LuaPath))
    })? )?;
    module.set("executable", lua.create_function( |_, _: ()| {
        Ok(dirs::executable_dir().map(LuaPath))
    })? )?;
    module.set("runtime", lua.create_function( |_, _: ()| {
        Ok(dirs::executable_dir().map(LuaPath))
    })? )?;
    module.set("desktop", lua.create_function( |_, _: ()| {
        Ok(dirs::desktop_dir().map(LuaPath))
    })? )?;
    module.set("document", lua.create_function( |_, _: ()| {
        Ok(dirs::document_dir().map(LuaPath))
    })? )?;
    module.set("download", lua.create_function( |_, _: ()| {
        Ok(dirs::download_dir().map(LuaPath))
    })? )?;
    module.set("font", lua.create_function( |_, _: ()| {
        Ok(dirs::font_dir().map(LuaPath))
    })? )?;
    module.set("picture", lua.create_function( |_, _: ()| {
        Ok(dirs::picture_dir().map(LuaPath))
    })? )?;
    module.set("public", lua.create_function( |_, _: ()| {
        Ok(dirs::public_dir().map(LuaPath))
    })? )?;
    module.set("template", lua.create_function( |_, _: ()| {
        Ok(dirs::template_dir().map(LuaPath))
    })? )?;
    module.set("video", lua.create_function( |_, _: ()| {
        Ok(dirs::video_dir().map(LuaPath))
    })? )?;


    lua.globals().set("dirs", module)?;

    Ok(())
}
