use rlua::prelude::*;
use std::{fs, io};
use std::io::BufRead;
use chrono::{DateTime, Local};
use unidiff::unidiff;

fn time_format(d: &DateTime<Local>) -> String {
    d.format("%Y-%m-%d %H:%M:%S.%f %z").to_string()
}

fn mtime(path: &str) -> ::Result<DateTime<Local>> {
    Ok(DateTime::from(fs::metadata(path)?.modified()?))
}

fn read_file(path: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let file = io::BufReader::new(file);
    file.lines().collect()
}

pub fn init(lua: &Lua) -> ::Result<()> {
    let module = lua.create_table()?;

    module.set("compare_strings", lua.create_function( |_, (left, right): (String, String)| {
        let prefix = vec![
            format!("--- a\t{}", time_format(&Local::now())),
            format!("+++ b\t{}", time_format(&Local::now()))
        ];

        let diff = unidiff(
            &left
                .split("\n")
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            &right
                .split("\n")
                .map(str::to_owned)
                .collect::<Vec<_>>(),
                3
            ).map_err(LuaError::external)?;
        
        let mut res = String::new();
        prefix.iter().cloned().chain(diff).for_each(|s| { res.push_str(&s); res.push('\n') });

        Ok(res)
    })?)?;

    module.set("compare_files", lua.create_function( |_, (left, right): (String, String)| {
        let prefix = vec![
            format!("--- {}\t{}", &left, time_format(&mtime(&left).map_err(LuaError::external)?)),
            format!("+++ {}\t{}", &right, time_format(&mtime(&right).map_err(LuaError::external)?))
        ];

        let left = read_file(&left).map_err(LuaError::external)?;
        let right = read_file(&right).map_err(LuaError::external)?;

        let diff = unidiff(&left, &right, 3).map_err(LuaError::external)?;

        let mut res = String::new();
        prefix.iter().cloned().chain(diff).for_each(|s| { res.push_str(&s); res.push('\n') });

        Ok(res)
    })?)?;


    lua.globals().set("diff", module)?;

    Ok(())
}

