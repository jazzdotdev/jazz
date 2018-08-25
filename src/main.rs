//! A basic example on how to use request fields from inside a lua script.
extern crate actix;
extern crate actix_lua;
extern crate actix_web;
extern crate env_logger;
extern crate futures;

use std::collections::HashMap;
use actix::prelude::*;
use actix_lua::{LuaActor, LuaActorBuilder, LuaMessage};
use actix_web::{
    http, server, App, AsyncResponder,
    FutureResponse, HttpResponse, HttpMessage, HttpRequest,
};
use futures::Future;
use http::header::ToStrError;

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

    table
}

struct AppState {
    lua: Addr<LuaActor>,
}

fn req_data((req, body): (HttpRequest<AppState>, String)) -> FutureResponse<HttpResponse> {
    let table = extract_table_from_req(&req, body);

    req.state()
        .lua
        .send(LuaMessage::Table(table))
        .from_err()
        .and_then(|res| match res {
            LuaMessage::String(s) => Ok(HttpResponse::Ok().body(s)),

            // ignore everything else
            _ => unimplemented!(),
        })
        .responder()
}

fn main() {
    env_logger::init();
    let sys = actix::System::new("actix-lua-example");

    let addr = Arbiter::start(|_| {
        LuaActorBuilder::new()
            .on_handle_with_lua(
                r#"
                    host = "0.0.0.0"
                    if ctx.msg.host then
                        host = ctx.msg.host
                    end

                    message = "Host: " .. host .. "\n" .. ctx.msg.req_line .. "\n\nHTTP headers:\n"

                    for k, v in pairs(ctx.msg.headers) do
                        message = message .. k .. ": " .. v .. "\n"
                    end

                    message = message .. "\nRequest body:\n" .. ctx.msg.body

                    print(message)

                    return "<html><head><title>Hello " ..
                        host ..
                        "</title></head><body>Hello " ..
                        host ..
                        "</body></html>"
                "#,
            )
            .build()
            .unwrap()
    });

    server::new(move || {
        App::with_state(AppState { lua: addr.clone() })
            .resource("/{_:.*}", |r| r.with(req_data))
    }).bind("localhost:3000")
        .unwrap()
        .start();

    println!("Started http server: localhost:3000");
    let _ = sys.run();
}
