#[macro_use] extern crate failure;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate human_panic;
#[macro_use] extern crate serde_derive;
#[cfg(feature = "tantivy_bindings")]
extern crate tantivy;

pub mod bindings;
pub mod logger;
pub mod conf;
pub mod error;

use actix::prelude::*;
use actix_lua::LuaActorBuilder;
use actix_web::{server as actix_server, App};
use rlua::prelude::*;
use std::{
    path::{Path, PathBuf},
    result,
    fs
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde_json::Value;
use crate::error::Error;

type LuaAddr = ::actix::Addr<::actix_lua::LuaActor>;
pub type Result<T> = result::Result<T, Error>;

#[derive(Clone)]
pub struct AppState {
    pub lua: Option<LuaAddr>,
    pub init_path: PathBuf,
    pub init_args: Vec<String>,
    pub package_path: Option<String>,
    pub settings: Value,
    pub app_settings: Option<(String, Value)>,
}

impl AppState {
    pub fn create_vm (&self) -> result::Result<Lua, LuaError> {
        let lua = unsafe { Lua::new_with_debug() };

        lua.exec::<_, ()>(include_str!("handlers/debug.lua"), None)?;

        bindings::app::init(&lua).map_err(LuaError::external)?;
        bindings::archive::init(&lua).map_err(LuaError::external)?;
        bindings::crypto::init(&lua).map_err(LuaError::external)?;
        bindings::string::init(&lua).map_err(LuaError::external)?;
        bindings::system::init(&lua).map_err(LuaError::external)?;
        bindings::text::init(&lua).map_err(LuaError::external)?;
        bindings::web::init(&lua).map_err(LuaError::external)?;
        bindings::number::init(&lua).map_err(LuaError::external)?;
        bindings::net::init(&lua).map_err(LuaError::external)?;

        // torchbear global table
        {
            let tb_table = lua.create_table()?;
            tb_table.set("settings", rlua_serde::to_value(&lua, &self.settings).map_err(LuaError::external)?)?;
            tb_table.set("init_filename", self.init_path.to_str())?;
            tb_table.set("version", env!("CARGO_PKG_VERSION"))?;
            let os = if cfg!(target_os = "windows") {
                "windows"
            } else if cfg!(target_os = "linux") {
                "linux"
            } else if cfg!(target_os = "macos") {
                "macos"
            } else if cfg!(target_os = "android") {
                "android"
            } else {
                "unknown"
            };
            tb_table.set("os", os)?;
            lua.globals().set("torchbear", tb_table)?;
        }

        // app table

        if let Some((name, app_settings)) = &self.app_settings {
            let tb_table = lua.create_table()?;
            tb_table.set("settings", rlua_serde::to_value(&lua, app_settings).map_err(LuaError::external)?)?;
            lua.globals().set(name.as_str(), tb_table)?;
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
        let mut cmd_args = self.init_args.clone();
        // if no command line argument is passed, push current working directory
        if cmd_args.is_empty() {
            cmd_args.push(String::from("."));
        }

        // if file path is symlink, then resolve
        match fs::read_link(&cmd_args[0]) {
            Ok(p) => cmd_args[0] = String::from(p.to_str().unwrap_or("")),
            Err(_) => (),
        };
        lua.globals().set("arg", lua.create_sequence_from(cmd_args)?)?;

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

    pub fn start (&mut self, args: Option<Vec<String>>) -> Result<()> {
        
        setup_panic!();

        let mut init_path: Option<PathBuf> = None;
        let mut init_args: Vec<String> = vec![];
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


                init_args = args.to_vec();

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

        fn get_or (map: &Value, key: &str, val: &str) -> String {
            map.get(key).map(|s| String::from(s.as_str().unwrap_or(val)) ).unwrap_or(String::from(val))
        }

        let root_path = match &init_path {
            Some(p) => p.parent().unwrap_or(Path::new(".")),
            None => Path::new("."),
        };

        let config_path = root_path.join("torchbear.scl");

        let config = if config_path.exists() {
            conf::Conf::load_file(&config_path)?
        } else {
            SettingConfig::default()
        };

        let general = config.general.unwrap_or_default();

        let app_config: Option<(String, Value)> = general.get("app-name").and_then(Value::as_str).map(PathBuf::from).and_then(|name| {
            let mut config_path = root_path.join(&name);
            config_path.set_extension("scl");
            if config_path.exists() && config_path.is_file() {
                conf::Conf::load_file(&config_path).map(|s| (name.to_string_lossy().to_string(), s)).ok()
            } else {
                None
            }
        });

        let init_path = init_path.unwrap_or(PathBuf::from(&get_or(&general, "init", "init.lua")));
        
        if !init_path.exists() || !init_path.is_file() {
            println!("Error: Specified init.lua not found. You may have not completed installing your app");
            std::process::exit(1);
        }

        let log_path = general.get("log_path").and_then(|log| log.as_str());

        logger::init(log_path, self.log_settings.clone());

        let sys = actix::System::new("torchbear");

        let mut app_state = AppState {
            lua: None,
            init_path: init_path,
            init_args: init_args,
            package_path: package_path,
            settings: general,
            app_settings: app_config,
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

            server = server.bind((host.as_str(), port))?;
            log::debug!("web server listening on port {}:{}", &host, port);

            if let Some(ssl_builder) = some_ssl {
                let host = get_or(&web, "tls_address", "0.0.0.0");
                let port = get_or(&web, "tls_port", "3001").parse().unwrap_or(3001);
                server = server.bind_ssl((host.as_str(), port), ssl_builder)?;
                log::debug!("tls server listening on port {}:{}", &host, port);
            }

            server.start();

            let _ = sys.run();
        } else {
            // Temporary fix to run non webserver apps. Doesn't start the actor
            // system, just runs a vanilla lua vm.
            debug!("Torchbear app started");
            let _ = app_state.create_vm()?;
        }

        Ok(())
    }
}
