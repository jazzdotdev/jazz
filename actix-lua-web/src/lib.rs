//! A basic example on how to use request fields from inside a lua script.
extern crate actix;
extern crate actix_lua;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate tera;
extern crate rlua;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate serde_urlencoded;
extern crate rlua_serde;
extern crate uuid;
extern crate comrak;
extern crate rust_sodium;
extern crate base64;

use std::sync::Arc;
use actix::prelude::*;
use actix_lua::{LuaActor, LuaActorBuilder};
use actix_web::{server as actix_server, App};
use tera::{Tera};
use rlua::prelude::*;

mod lua_bindings;
mod server;

mod app_state {
    use std::sync::Arc;
    use actix::Addr;
    use actix_lua::{LuaActor};
    use tera::{Tera};

    pub struct AppState {
        pub lua: Addr<LuaActor>,
        pub tera: Arc<Tera>,
    }
}

fn set_vm_globals(lua: &Lua, tera: Arc<Tera>, lua_modules_path: &str) -> Result<(), LuaError> {
    lua.exec::<()>(&format!(r#"
        package.path = package.path..";{}"
    "#, lua_modules_path), None)?;

    lua_bindings::tera::init(lua, tera)?;
    lua_bindings::yaml::init(lua)?;
    lua_bindings::uuid::init(lua)?;
    lua_bindings::markdown::init(lua)?;
    lua_bindings::client::init(lua)?;

    Ok(())
}

pub struct ApplicationBuilder {
    handler_path: Option<&'static str>,
    templates_path: Option<&'static str>,
    lua_modules_path: Option<&'static str>,
    host: Option<&'static str>,
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        ApplicationBuilder {
            handler_path: None,
            templates_path: None,
            host: None,
            lua_modules_path: None,
        }
    }

    pub fn handler_path(mut self, path: &'static str) -> Self {
        self.handler_path = Some(path);
        self
    }

    pub fn host(mut self, host: &'static str) -> Self {
        self.host = Some(&host);
        self
    }

    pub fn templates_path(mut self, path: &'static str) -> Self {
        self.templates_path = Some(path);
        self
    }

    pub fn lua_modules_path(mut self, path: &'static str) -> Self {
        self.lua_modules_path = Some(path);
        self
    }

    pub fn start(self) {
        let handler_path = self.handler_path.unwrap_or("lua/handler.lua");
        let templates_path = self.templates_path.unwrap_or("templates/**/*");
        let host = self.host.unwrap_or("0.0.0.0:3000");
        let lua_modules_path = self.lua_modules_path.unwrap_or("./lua/?.lua");

        let sys = actix::System::new("actix-lua-web");
        let tera = Arc::new(compile_templates!(templates_path));

        let shared_tera = tera.clone();
        let addr = Arbiter::start(move |_| {
            let tera = shared_tera;
            let lua_actor = LuaActorBuilder::new()
                .on_handle(handler_path)
                .with_vm(move |vm| {
                    set_vm_globals(vm, tera.clone(), lua_modules_path)
                })
                .build()
                .unwrap();

            lua_actor
        });

        actix_server::new(move || {
            App::with_state(app_state::AppState { lua: addr.clone(), tera: tera.clone() })
                .default_resource(|r| r.with(server::handler))
        }).bind(host)
            .unwrap()
            .start();

        println!("Started http server: localhost:3000");
        let _ = sys.run();
    }
}
