use rlua::prelude::*;
use rlua::{Lua, UserData, UserDataMethods};

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    let git = lua.create_table()?;

    git.set(
        "init",
        lua.create_function(|_, path: String| Ok(::git2::Repository::init(&path).is_ok()))?,
    )?;

    let globals = lua.globals();
    globals.set("git", git)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use rlua::{Lua, Value};
    #[test]
    fn test() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::env::set_current_dir(root).unwrap();

        let lua = Lua::new();
        super::init(&lua).unwrap();

        lua.exec::<_, Value>(r#"assert(git.init("repo") == true)"#, None).unwrap();
        git2::Repository::open("repo").unwrap();
    }
}
