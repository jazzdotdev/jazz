use rlua::Lua;
use git2;
use rlua::prelude::LuaError;

/// git add
///
/// repo: path of repository
/// paths: path of files to add. These paths are relative to repo.
/// For example: current directory is /, repo is /repo, file is /repo/file, we need to call:
/// git_add("repo", &vec!["file"])
fn git_add(repo: &str, paths: &Vec<String>) -> crate::Result<()> {
    let repo = git2::Repository::open(&repo)?;
    let mut index = repo.index()?;
    index.add_all(paths.iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}

/// repo: Repository's path
/// message: commit message
/// sig: pair of (name, email). If is None, will try to use repo's config.
fn git_commit(repo: &str, message: &str, sig: Option<(String, String)>) -> crate::Result<()> {
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

fn git_clone(url: &str, into: &str) -> crate::Result<()> {
    git2::Repository::clone(url, into)?;
    Ok(())
}

fn git_pull(path: &str, remote_name: &str, branch_name: &str) -> crate::Result<()> {
    let repo = git2::Repository::open(&path)?;
    repo.find_remote(remote_name)?
        .fetch(&[branch_name], None, None)?;
    Ok(())
}

/// path: path to the repository
/// spec: revision string aka commit hash or commit reference
/// reset_type: Reset type(hard, soft or mixed)
/// example: git_reset("torchbear", "origin/master", "hard")
fn git_reset(path: &str, spec: &str, reset_type_str: &str) -> crate::Result<()> {
    let mut checkout_builder = git2::build::CheckoutBuilder::new();
    let repo = git2::Repository::open(&path)?;
    let reset_type = match reset_type_str {
        "soft" => git2::ResetType::Soft,
        "mixed" => git2::ResetType::Mixed,
        "hard" => git2::ResetType::Hard,
        _ => git2::ResetType::Soft,
    };
    let rev = repo.revparse_single(spec)?;
    repo.reset(&rev, reset_type, Some(&mut checkout_builder))?;
    Ok(())
}

pub fn init(lua: &Lua) -> crate::Result<()> {
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

    git.set(
        "log",
        lua.create_function(|lua, repo: String| {
            let repo = match git2::Repository::open(&repo) {
                Ok(repo) => repo,
                Err(_) => return Ok(None),
            };
            let mut walk = match repo.revwalk() {
                Ok(walk) => walk,
                Err(_) => return Ok(None),
            };
            if walk.push_head().is_ok() {
                let walk = walk.filter_map(|x| match x {
                    Err(_) => None,
                    Ok(id) => match repo.find_commit(id) {
                        Err(_) => None,
                        Ok(commit) => {
                            let table = lua.create_table().unwrap();
                            table.set("id", format!("{}", commit.id())).unwrap();
                            table
                                .set("message", commit.message().unwrap_or(""))
                                .unwrap();
                            Some(table)
                        }
                    },
                });
                Ok(Some(walk.collect::<Vec<_>>()))
            } else {
                Ok(Some(Vec::new()))
            }
        })?,
    )?;

    git.set(
	"clone",
	lua.create_function(|_, (url, into): (String, String)| {
	    git_clone(&url, &into).map_err(LuaError::external)
	})?,
    )?;

    git.set(
        "pull",
        lua.create_function(|_, (path, remote_name, branch_name): (String, String, String)| {
            git_pull(&path, &remote_name, &branch_name).map_err(LuaError::external)
        })?,
    )?;

    git.set(
        "reset",
        lua.create_function(|_, (path, spec, reset_type): (String, String, String)| {
            git_reset(&path, &spec, &reset_type).map_err(LuaError::external)
        })?,
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

        lua.exec::<_, Value>(r#"assert(#git.log("repo") == 0)"#, None)
            .unwrap();

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

        lua.exec::<_, Value>(
            r#"
            local log = git.log("repo")
            assert(log[1].message == "second")
            assert(log[2].message == "initial")
            "#,
            None,
        ).unwrap();
    }
}
