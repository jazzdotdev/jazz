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
extern crate config;
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

#[cfg(feature = "tantivy_bindings")]
extern crate tantivy;

pub mod bindings;
pub mod logger;

use actix::prelude::*;
use actix_lua::LuaActorBuilder;
use actix_web::{server as actix_server, App};
use rlua::prelude::*;
use std::path::Path;
use std::io;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde_json::Value;


pub struct AppState {
    pub lua: ::actix::Addr<::actix_lua::LuaActor>
}

fn create_vm(init_path: &str, settings: Value) -> Result<Lua, LuaError> {
    let lua = unsafe { Lua::new_with_debug() };

    lua.exec::<_, ()>(include_str!("handlers/debug.lua"), None)?;

    bindings::tera::init(&lua)?;
    bindings::yaml::init(&lua)?;
    bindings::json::init(&lua)?;
    bindings::uuid::init(&lua)?;
    bindings::markdown::init(&lua)?;
    bindings::client::init(&lua)?;
    bindings::crypto::init(&lua)?;
    bindings::stringset::init(&lua)?;
    bindings::time::init(&lua)?;
    bindings::fs::init(&lua)?;
    bindings::select::init(&lua)?;
    bindings::git::init(&lua)?;
    bindings::regex::init(&lua)?;
    bindings::tantivy::init(&lua)?;
    bindings::mime::init(&lua)?;

    // torchbear crashes if there's no log binding
    //if cfg!(feature = "log_bindings") {
        bindings::log::init(&lua)?;
    //}

    // torchbear global table
    {
        let tb_table = lua.create_table()?;
        tb_table.set("settings", rlua_serde::to_value(&lua, settings).map_err(LuaError::external)?)?;
        tb_table.set("init_filename", init_path)?;
        tb_table.set("version", env!("CARGO_PKG_VERSION"))?;
        lua.globals().set("torchbear", tb_table)?;
    }

    // Lua Bridge
    lua.exec::<_, ()>(include_str!("handlers/bridge.lua"), None)?;

    Ok(lua)
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

    pub fn start (&mut self) {

        let mut settings = config::Config::new();
        
        let setting_file = Path::new("torchbear.toml");
        if setting_file.exists() {
            match settings.merge(config::File::with_name("torchbear.toml")) {
                Err(err) => {
                    println!("Error opening torchbear.toml: {}", err);
                    std::process::exit(1);
                },
                _ => ()
            };
            settings.merge(config::Environment::with_prefix("torchbear")).unwrap();
        }

        let config = settings.try_into::<SettingConfig>().unwrap_or_default();

        fn get_or (map: &Value, key: &str, val: &str) -> String {
            map.get(key).map(|s| String::from(s.as_str().unwrap_or(val)) ).unwrap_or(String::from(val))
        }
        
        let general = config.general.unwrap_or_default();

        let init_path = get_or(&general, "init", "init.lua");
        let log_path = get_or(&general, "log_path", "log");
        
        if !Path::new(&init_path).exists() {
            println!("Error: Torchbear needs an app to run. Change to the directory containing your application and run torchbear again.");
            std::process::exit(1);
        }

        logger::init(::std::path::Path::new(&log_path), self.log_settings.clone());
        //log_panics::init();

        let sys = actix::System::new("torchbear");

        let vm = create_vm(&init_path, general).unwrap();
        
        let addr = Arbiter::start(move |_| {
            let lua_actor = LuaActorBuilder::new()
                .on_handle_with_lua(include_str!("handlers/web_server.lua"))
                .build_with_vm(vm)
                .unwrap();
            lua_actor
        });

        if let Some(web) = config.web_server {            
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
                App::with_state(AppState { lua: addr.clone() })
                    .default_resource(|r| r.with(bindings::server::handler))
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
        }
    
    }
}
