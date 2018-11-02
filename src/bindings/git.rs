use rlua::prelude::*;
use rlua::{Lua, UserData, UserDataMethods};

/// git add
///
/// repo: path of repository
/// paths: path of files to add. These paths are relative to repo.
/// For example: current directory is /, repo is /repo, file is /repo/file, we need to call:
/// git_add("repo", &vec!["file"])
fn git_add(repo: &str, paths: &Vec<String>) -> Result<(), git2::Error> {
    let repo = git2::Repository::open(&repo)?;
    let mut index = repo.index()?;
    index.add_all(paths.iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}

/// repo: Repository's path
/// message: commit message
/// sig: pair of (name, email). If is None, will try to use repo's config.
fn git_commit(repo: &str, message: &str, sig: Option<(String, String)>) -> Result<(), git2::Error> {
    let repo = git2::Repository::open(&repo)?;
    let sig = if let Some((name, email)) = sig {
        git2::Signature::now(&name, &email)?
    } else {
        repo.signature()?
    };
    if let Ok(head) = repo.head() {
        // Not first commit
        let head = head.target().unwrap();
        let head = repo.find_commit(head)?;
        let mut index = repo.index()?;
        let id = index.write_tree()?;
        let tree = repo.find_tree(id)?;
        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&head])?;
    } else {
        // First commit
        let mut index = repo.index()?;
        let id = index.write_tree()?;
        let tree = repo.find_tree(id)?;
        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[])?;
    }
    Ok(())
}

pub fn init(lua: &Lua) -> Result<(), LuaError> {
    let git = lua.create_table()?;

    git.set(
        "init",
        lua.create_function(|_, path: String| Ok(::git2::Repository::init(&path).is_ok()))?,
    )?;

    git.set(
        "add",
        lua.create_function(|_, (repo, paths): (String, Vec<String>)| {
            Ok(git_add(&repo, &paths).is_ok())
        })?,
    )?;

    git.set(
        "commit",
        lua.create_function(
            |_, (repo, message, name, email): (String, String, Option<String>, Option<String>)| {
                let sig = if name.is_some() && email.is_some() {
                    Some((name.unwrap(), email.unwrap()))
                } else {
                    None
                };
                Ok(git_commit(&repo, &message, sig).is_ok())
            },
        )?,
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
        std::env::set_current_dir(dir.path()).unwrap();

        let lua = Lua::new();
        super::init(&lua).unwrap();

        lua.exec::<_, Value>(r#"assert(git.init("repo") == true)"#, None)
            .unwrap();
        let repo = git2::Repository::open("repo").unwrap();

        std::fs::write("repo/file", b"").unwrap();
        lua.exec::<_, Value>(r#"assert(git.add("repo", {"file"}) == true)"#, None)
            .unwrap();
        assert!(repo.status_file("file".as_ref()).unwrap().is_index_new());

        lua.exec::<_, Value>(
            r#"assert(git.commit("repo", "initial", "user", "user@gmail.com") == true)"#,
            None,
        ).unwrap();

        lua.exec::<_, Value>(
            r#"assert(git.commit("repo", "second", "user", "user@gmail.com") == true)"#,
            None,
        ).unwrap();
    }
}
