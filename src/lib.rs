//! A Lua application framework for Rust libraries.
extern crate actix;
extern crate actix_lua;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
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
extern crate chrono;
#[macro_use]
extern crate log;
extern crate fern;
extern crate colored;
extern crate log_panics;
extern crate select;
#[macro_use]
extern crate serde_derive;
extern crate git2;
extern crate regex;
extern crate openssl;
extern crate mime_guess;
extern crate heck;
extern crate zip;
extern crate tar;
extern crate xz2;
extern crate unidiff;
extern crate blake2;

#[cfg(feature = "tantivy_bindings")]
extern crate tantivy;
extern crate scl;

pub mod bindings;
pub mod logger;
pub mod conf;

use actix::prelude::*;
use actix_lua::LuaActorBuilder;
use actix_web::{server as actix_server, App};
use rlua::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::io;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde_json::Value;

type LuaAddr = ::actix::Addr<::actix_lua::LuaActor>;

#[derive(Clone)]
pub struct AppState {
    pub lua: Option<LuaAddr>,
    pub init_path: PathBuf,
    pub init_args: Option<Vec<String>>,
    pub package_path: Option<String>,
    pub settings: Value,
}

impl AppState {
    pub fn create_vm (&self) -> Result<Lua, LuaError> {
        let lua = unsafe { Lua::new_with_debug() };

        lua.exec::<_, ()>(include_str!("handlers/debug.lua"), None)?;

        bindings::app::init(&lua)?;
        bindings::archive::init(&lua)?;
        bindings::crypto::init(&lua)?;
        bindings::string::init(&lua)?;
        bindings::system::init(&lua)?;
        bindings::text::init(&lua)?;
        bindings::web::init(&lua)?;

        // torchbear global table
        {
            let tb_table = lua.create_table()?;
            tb_table.set("settings", rlua_serde::to_value(&lua, &self.settings).map_err(LuaError::external)?)?;
            tb_table.set("init_filename", self.init_path.to_str())?;
            tb_table.set("version", env!("CARGO_PKG_VERSION"))?;
            lua.globals().set("torchbear", tb_table)?;
        }

        // Lua package.path
        match self.package_path {
            Some(ref package_path) => {
                let package: LuaTable = lua.globals().get("package")?;
                let mut path: String = package.get("path")?;
                path.push_str(";");
                path.push_str(package_path);
                package.set("path", path)?;
            },
            None => ()
        }

        // Lua arg
        match self.init_args {
            Some(ref init_args) => lua.globals().set("arg", lua.create_sequence_from(init_args.clone())?)?,
            None => ()
        }

        // Lua Bridge
        lua.exec::<_, ()>(include_str!("handlers/bridge.lua"), None)?;

        Ok(lua)
    }

    pub fn create_addr (&self) -> LuaAddr {
        let vm = self.create_vm().unwrap();
        Arbiter::start(move |_| {
            let lua_actor = LuaActorBuilder::new()
                .on_handle_with_lua(include_str!("handlers/web_server.lua"))
                .build_with_vm(vm)
                .unwrap();
            lua_actor
        })
    }
}

//TODO: Implement a better error handler for `ApplicationBuilder` or across torchbear
fn server_handler<H, F>(srv: io::Result<actix_web::server::HttpServer<H, F>>) -> actix_web::server::HttpServer<H, F>
    where H: actix_web::server::IntoHttpHandler,
          F: Fn() -> H + Send + Clone
{
    match srv {
        Ok(srv) => srv,
        Err(e) => if e.kind() == io::ErrorKind::AddrInUse {
            println!("Error: Address already in use.");
            std::process::exit(1);
        } else {
            println!("Unknown error as occurred: {:?}", e);
            std::process::exit(1);
        }
    }
}

pub struct ApplicationBuilder {
    log_settings: logger::Settings,
}

#[derive(Debug, Default, Deserialize)]
pub struct SettingConfig {
    general: Option<Value>,
    #[serde(rename = "web-server")]
    web_server: Option<Value>,
}

impl ApplicationBuilder {
    pub fn new () -> Self {
        Self {
            log_settings: logger::Settings{
                level: logger::LevelFilter::Info,
                everything: false,
            }
        }
    }

    pub fn log_level (&mut self, level: logger::Level) -> &mut Self {
        self.log_settings.level = level.to_level_filter(); self
    }

    pub fn log_everything (&mut self, b: bool) -> &mut Self {
        self.log_settings.everything = b; self
    }

    pub fn start (&mut self, args: Option<Vec<String>>) {

        let mut init_path: Option<PathBuf> = None;
        let mut init_args: Option<Vec<String>> = None;
        let mut package_path: Option<String> = None;

        match args {
            // Interpreter
            Some(args) => {

                init_path = Path::new(args.first().expect("Missing first argument."))
                    .canonicalize()
                    .map_err(|e| {
                        println!("Error getting the absolute path: {}", e);
                        std::process::exit(1);
                    })
                    .ok();


                init_args = Some(args.to_vec());

                package_path = match &init_path {
                    Some(p) => p.parent().map(|p| {
                            let mut t = p.to_str().expect("Error getting the directory.").to_string();
                            t.push_str("/?.lua"); t
                    }),
                    None => None
                };
            },
            // Server
            None => ()
        }

        let setting_file = Path::new("torchbear.scl");

        let config = if setting_file.exists() {
            conf::Conf::load_file(&setting_file)
        } else {
            SettingConfig::default()
        };
        //let config = settings.try_into::<SettingConfig>().unwrap_or_default();

        fn get_or (map: &Value, key: &str, val: &str) -> String {
            map.get(key).map(|s| String::from(s.as_str().unwrap_or(val)) ).unwrap_or(String::from(val))
        }
        
        let general = config.general.unwrap_or_default();

        let init_path = init_path.unwrap_or(Path::new(&get_or(&general, "init", "init.lua")).to_path_buf());
        let log_path = get_or(&general, "log_path", "log");
        
        if !init_path.exists() {
            println!("Error: Specified init.lua not found. You may have not completed installing your app");
            std::process::exit(1);
        }

        logger::init(::std::path::Path::new(&log_path), self.log_settings.clone());
        //log_panics::init();

        let sys = actix::System::new("torchbear");

        let mut app_state = AppState {
            lua: None,
            init_path: init_path,
            init_args: init_args,
            package_path: package_path,
            settings: general
        };

        if let Some(web) = config.web_server {

            if let Some(Some(bootstrap)) = web.get("bootstrap_path").map(|s| { s.as_str() }) {
                let vm = app_state.create_vm().unwrap();
                vm.globals().get::<_, LuaTable>("torchbear").unwrap().set("bootstrap", bootstrap).unwrap();

                if !vm.exec::<_, bool>(include_str!("handlers/bootstrap.lua"), Some("bootstrap")).unwrap()
                { std::process::exit(1); }
            }

            let single_actor = match web.get("single_actor").map(|s| { s.as_bool() }) {
                None => false,
                Some(Some(b)) => b,
                _ => {
                    println!("Error: Setting web_server.single_actor must be a boolean value");
                    std::process::exit(1);
                },
            };

            if single_actor {
                app_state.lua = Some(app_state.create_addr());
            }

            log::debug!("web server section in settings, starting seting up web server");
            let host = get_or(&web, "address", "0.0.0.0");
            let port = get_or(&web, "port", "3000").parse().unwrap_or(3000);

            let some_ssl = match (web.get("tls_private"), web.get("tls_certificate")) {
                (None, None) => None,
                (Some(priv_path), Some(cert_path)) => {
                    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
                    builder.set_private_key_file(priv_path.as_str().unwrap(), SslFiletype::PEM).unwrap();
                    builder.set_certificate_chain_file(cert_path.as_str().unwrap()).unwrap();
                    Some(builder)
                },
                _ => {
                    println!("Error: SSL needs both tls_private and tls_certificate settings.");
                    std::process::exit(1);
                }
            };

            let mut server = actix_server::new(move || {
                App::with_state(app_state.clone())
                    .default_resource(|r| r.with(bindings::web::server::handler))
            });

            server = server_handler(server.bind((host.as_str(), port)));
            log::debug!("web server listening on port {}:{}", &host, port);

            if let Some(ssl_builder) = some_ssl {
                let host = get_or(&web, "tls_address", "0.0.0.0");
                let port = get_or(&web, "tls_port", "3001").parse().unwrap_or(3001);
                server = server_handler(server.bind_ssl((host.as_str(), port), ssl_builder));
                log::debug!("tls server listening on port {}:{}", &host, port);
            }

            server.start();

            let _ = sys.run();
        } else {
            // Temporary fix to run non webserver apps. Doesn't start the actor
            // system, just runs a vanilla lua vm.
            debug!("Torchbear app started");
            let _ = app_state.create_vm().unwrap();
        }
    
    }
}
