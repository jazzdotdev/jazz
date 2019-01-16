use rlua::prelude::*;
use std::{
    mem,
    collections::HashMap,
    io::{BufReader, prelude::*},
    process::{
        Command,
        Child,
        ChildStdout,
        ChildStdin,
        ChildStderr,
        ExitStatus,
        Stdio,
        Output
    }
};
use crate::error::Error;

pub struct LuaCommand(Command);
pub struct LuaChild {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    stderr: BufReader<ChildStderr>
}
pub struct LuaOutput(Output);
pub struct LuaExitStatus(ExitStatus);

impl LuaUserData for LuaCommand {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {

        fn stdio_type(stdtype: Option<&String>) -> Stdio {
            match stdtype.map(|s| s.as_str()) {
                Some("inherit") => Stdio::inherit(),
                Some("null") => Stdio::null(),
                Some("piped") | _ => Stdio::piped(),
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
        methods.add_method_mut("spawn", |_, this: &mut LuaCommand, args: Option<HashMap<String, String>>|{
            if let Some(args) = args {
                this.0.stdin(stdio_type(args.get("stdin")))
                    .stdout(stdio_type(args.get("stdout")))
                    .stderr(stdio_type(args.get("stderr")));
            } else {
                this.0.stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped());
            }

            let mut child = this.0.spawn().map_err(LuaError::external)?;
            let stdout = mem::replace(&mut child.stdout, None).map(BufReader::new).ok_or(LuaError::external(Error::InternalError))?;
            let stderr = mem::replace(&mut child.stderr, None).map(BufReader::new).ok_or(LuaError::external(Error::InternalError))?;
            let stdin = mem::replace(&mut child.stdin, None).ok_or(LuaError::external(Error::InternalError))?;
            
            Ok(LuaChild {
                child: child,
                stdin: stdin,
                stdout: stdout,
                stderr: stderr
            })

        });
        methods.add_method_mut("exec", |_, this: &mut LuaCommand, _: ()|{
            this.0.output().map(LuaOutput).map_err(LuaError::external)
        });
    }
}

//TODO: Have stdout and stderr share a common binding. Maybe by splitting the method into stdin, and have stdout/stderr share the same interface since they both implement `Read` trait
impl LuaUserData for LuaChild {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("kill", |_, this: &mut LuaChild, _: ()|{
            this.child.kill().map_err(LuaError::external)
        });
        methods.add_method_mut("id", |_, this: &mut LuaChild, _: ()|{
            Ok(this.child.id())
        });
        methods.add_method_mut("wait", |_, this: &mut LuaChild, _: ()|{
            this.child.wait().map(LuaExitStatus).map_err(LuaError::external)
        });
        methods.add_method_mut("write", |_, this: &mut LuaChild, data: String|{
            this.stdin.write(data.as_bytes()).map_err(LuaError::external)
        });
        methods.add_method_mut("flush", |_, this: &mut LuaChild, _: ()| {
            this.stdin.flush().map_err(LuaError::external)
        });
        methods.add_method_mut("read", |_, this: &mut LuaChild, len: Option<usize>|{
            let bytes = match len {
                Some(len) => {
                    let mut bytes = vec![0u8; len];
                    this.stdout.read(&mut bytes).map_err(LuaError::external)?;
                    bytes
                },
                None => {
                    let mut bytes = vec![];
                    this.stdout.read_to_end(&mut bytes).map_err(LuaError::external)?;
                    bytes
                }
            };
            Ok(bytes)
        });
        methods.add_method_mut("read_to_string", |_, this: &mut LuaChild, _:()|{
            let mut data = String::new();
            this.stdout.read_to_string(&mut data).map_err(LuaError::external)?;
            Ok(data)
        });
        methods.add_method_mut("read_line", |_, this: &mut LuaChild, _: ()|{
            let mut data = String::new();
            this.stdout.read_line(&mut data).map_err(LuaError::external)?;
            Ok(data)
        });
        methods.add_method_mut("read_error", |_, this: &mut LuaChild, len: Option<usize>|{
            let bytes = match len {
                Some(len) => {
                    let mut bytes = vec![0u8; len];
                    this.stderr.read(&mut bytes).map_err(LuaError::external)?;
                    bytes
                },
                None => {
                    let mut bytes = vec![];
                    this.stderr.read_to_end(&mut bytes).map_err(LuaError::external)?;
                    bytes
                }
            };
            Ok(bytes)
        });
        methods.add_method_mut("read_error_to_string", |_, this: &mut LuaChild, _:()|{
            let mut data = String::new();
            this.stderr.read_to_string(&mut data).map_err(LuaError::external)?;
            Ok(data)
        });
        methods.add_method_mut("read_error_line", |_, this: &mut LuaChild, _: ()|{
            let mut data = String::new();
            this.stderr.read_line(&mut data).map_err(LuaError::external)?;
            Ok(data)
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
