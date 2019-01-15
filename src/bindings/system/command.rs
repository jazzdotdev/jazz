use rlua::prelude::*;
use std::{
    collections::HashMap,
    process::{
        Command,
        Child,
        ExitStatus,
        Stdio,
        Output
    }
};

pub struct LuaCommand(Command);
pub struct LuaChild(Child);
pub struct LuaOutput(Output);
pub struct LuaExitStatus(ExitStatus);

impl LuaUserData for LuaCommand {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {

        fn stdio_type<A: AsRef<str>>(stdtype: A) -> Stdio {
            match stdtype.as_ref() {
                "inherit" => Stdio::inherit(),
                "null" => Stdio::null(),
                "piped" | _ => Stdio::piped(),
            }
        }

        methods.add_method_mut("arg", |_, this: &mut LuaCommand, arg: String|{
            this.0.arg(arg);
            Ok(())
        });
        methods.add_method_mut("args", |_, this: &mut LuaCommand, args: Vec<String>|{
            this.0.args(args);
            Ok(())
        });
        methods.add_method_mut("env", |_, this: &mut LuaCommand, (k, v): (String, String)|{
            this.0.env(k, v);
            Ok(())
        });
        methods.add_method_mut("envs", |_, this: &mut LuaCommand, env: HashMap<String, String>|{
            this.0.envs(env);
            Ok(())
        });
        methods.add_method_mut("env_clear", |_, this: &mut LuaCommand, key: Option<String>|{
            match key {
                Some(key) => this.0.env_remove(key),
                None => this.0.env_clear()
            };
            Ok(())
        });
        methods.add_method_mut("directory", |_, this: &mut LuaCommand, dir: String|{
            this.0.current_dir(dir);
            Ok(())
        });
        methods.add_method_mut("stdin", |_, this: &mut LuaCommand, stdtype: String|{
            this.0.stdin(stdio_type(stdtype));
            Ok(())
        });
        methods.add_method_mut("stdout", |_, this: &mut LuaCommand, stdtype: String|{
            this.0.stdout(stdio_type(stdtype));
            Ok(())
        });
        methods.add_method_mut("stderr", |_, this: &mut LuaCommand, stdtype: String|{
            this.0.stderr(stdio_type(stdtype));
            Ok(())
        });
        methods.add_method_mut("spawn", |_, this: &mut LuaCommand, _: ()|{
            this.0.spawn().map(LuaChild).map_err(LuaError::external)
        });
        methods.add_method_mut("exec", |_, this: &mut LuaCommand, _: ()|{
            this.0.output().map(LuaOutput).map_err(LuaError::external)
        });
    }
}

//TODO: Finish io implementation for child processes
impl LuaUserData for LuaChild {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("kill", |_, this: &mut LuaChild, _: ()|{
            this.0.kill().map_err(LuaError::external)
        });
        methods.add_method_mut("id", |_, this: &mut LuaChild, _: ()|{
            Ok(this.0.id())
        });
        methods.add_method_mut("wait", |_, this: &mut LuaChild, _: ()|{
            this.0.wait().map(LuaExitStatus).map_err(LuaError::external)
        });
    }
}

impl LuaUserData for LuaOutput {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("status", |_, this: &LuaOutput, _: ()|{
            Ok(LuaExitStatus(this.0.status))
        });
        methods.add_method_mut("stdout", |_, this: &mut LuaOutput, _: ()|{
            String::from_utf8(this.0.stdout.clone()).map_err(LuaError::external)
        });
        methods.add_method_mut("stderr", |_, this: &mut LuaOutput, _: ()|{
            String::from_utf8(this.0.stderr.clone()).map_err(LuaError::external)
        });
    }
}

impl LuaUserData for LuaExitStatus {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("success", |_, this: &LuaExitStatus, _: ()|{
            Ok(this.0.success())
        });
        methods.add_method("code", |_, this: &LuaExitStatus, _: ()|{
            Ok(this.0.code())
        });
    }
}

#[allow(unreachable_code)]
pub fn init(lua: &Lua) -> crate::Result<()> {
    let module = lua.create_table()?;

    module.set("new", lua.create_function( |_, (name, args): (String, Option<Vec<String>>)| {
        let mut command = Command::new(name);
        if let Some(args) = args {
            command.args(args);
        }
        Ok(LuaCommand(command))
    })? )?;

    lua.globals().set("command", module)?;

    Ok(())
}
