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
extern crate serde;
extern crate serde_yaml;
extern crate rlua_serde;

use std::collections::HashMap;
use std::sync::Arc;
use actix::prelude::*;
use actix_lua::{LuaActor, LuaActorBuilder, LuaMessage};
use actix_web::{
    http, server, App, AsyncResponder,
    FutureResponse, HttpResponse, HttpMessage, HttpRequest,
};
use futures::Future;
use tera::{Tera};
use rlua::prelude::*;

mod lua_bindings;

struct AppState {
    lua: Addr<LuaActor>,
    tera: Arc<Tera>,
}

/// Creates a lua table from a HttpRequest
fn extract_table_from_req(req: &HttpRequest<AppState>, body: String) -> HashMap<String, LuaMessage> {
    let mut table = HashMap::new();

    let query: HashMap<_, _> = req.query().iter()
        .map(|(key, value)| (key.clone(), LuaMessage::String(value.clone())))
        .collect();
    let headers: HashMap<_, _> = req.headers().iter()
        .map(|(key, value)| (
            key.as_str().to_owned(),
            LuaMessage::String(value.to_str().unwrap().to_owned()),
        ))
        .collect();
    let host = req.uri().host()
        .map(|host| LuaMessage::String(host.to_owned()))
        .unwrap_or(LuaMessage::Nil);
    let fragment = req.uri().to_string()
        .rsplit("#")
        .next()
        .map(|fragment| LuaMessage::String(fragment.to_owned()))
        .unwrap_or(LuaMessage::Nil);
    let path = req.path().to_string();

    let req_line = if req.query_string().is_empty() {
        format!(
            "{} {} {:?}",
            req.method(),
            req.path(),
            req.version()
        )
    } else {
        format!(
            "{} {}?{} {:?}",
            req.method(),
            req.path(),
            req.query_string(),
            req.version()
        )
    };

    table.insert("req_line".to_owned(), LuaMessage::String(req_line));
    table.insert("headers".to_owned(), LuaMessage::Table(headers));
    table.insert("body".to_owned(), LuaMessage::String(body));
    table.insert("query".to_owned(), LuaMessage::Table(query));
    table.insert("host".to_owned(), host);
    table.insert("fragment".to_owned(), fragment);
    table.insert("path".to_owned(), LuaMessage::String(path));

    table
}

fn req_data((req, body): (HttpRequest<AppState>, String)) -> FutureResponse<HttpResponse> {
    let table = extract_table_from_req(&req, body);

    req.state()
        .lua
        .send(LuaMessage::Table(table))
        .from_err()
        .and_then(|res| match res {
            LuaMessage::String(s) => Ok(HttpResponse::Ok().body(s)),
//            LuaMessage::Table(params) => {
//                // TODO: Take response params from the table
//            }

            // ignore everything else
            _ => unimplemented!(),
        })
        .responder()
}

fn set_vm_globals(lua: &Lua, tera: Arc<Tera>) -> Result<(), LuaError> {
    lua_bindings::tera::init(lua, tera)?;
    lua_bindings::yaml::init(lua)?;

    Ok(())
}

fn main() {
    env_logger::init();
    let sys = actix::System::new("actix-lua-example");
    let tera = Arc::new(compile_templates!("templates/**/*"));

    let shared_tera = tera.clone();
    let addr = Arbiter::start(move |_| {
        let tera = shared_tera;
        let lua_actor = LuaActorBuilder::new()
            .on_handle("src/handler.lua")
            .with_vm(move |vm| {
                set_vm_globals(vm, tera.clone())
            })
            .build()
            .unwrap();

        lua_actor
    });

    server::new(move || {
        App::with_state(AppState { lua: addr.clone(), tera: tera.clone() })
            .resource("/{_:.*}", |r| r.with(req_data))
    }).bind("0.0.0.0:3000")
        .unwrap()
        .start();

    println!("Started http server: localhost:3000");
    let _ = sys.run();
}
