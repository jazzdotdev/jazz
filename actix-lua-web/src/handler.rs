use std::collections::HashMap;
use actix::prelude::*;
use actix_lua::{LuaMessage};
use actix_web::{
    http, AsyncResponder,
    FutureResponse, HttpResponse, HttpMessage, HttpRequest,
};
use futures::Future;
use serde_urlencoded;

use ::AppState;

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

    let body_table: Result<HashMap<String, LuaMessage>, _> = serde_urlencoded::from_str(&body)
        .map(|parsed_body: HashMap<String, String>| {
            parsed_body
                .into_iter()
                .map(|(k, v)| (k, LuaMessage::String(v)))
                .collect()
        });

    match body_table {
        Ok(body_table) => {
            table.insert("body".to_owned(), LuaMessage::Table(body_table));
        },
        Err(_) => {
            table.insert("body".to_owned(), LuaMessage::String(body.clone()));
        }
    }

    table.insert("req_line".to_owned(), LuaMessage::String(req_line));
    table.insert("method".to_owned(), LuaMessage::String(req.method().to_string()));
    table.insert("headers".to_owned(), LuaMessage::Table(headers));
    table.insert("query".to_owned(), LuaMessage::Table(query));
    table.insert("host".to_owned(), host);
    table.insert("fragment".to_owned(), fragment);
    table.insert("path".to_owned(), LuaMessage::String(path));
    table.insert("body_raw".to_owned(), LuaMessage::String(body));

    table
}

pub fn handler((req, body): (HttpRequest<AppState>, String)) -> FutureResponse<HttpResponse> {
    let table = extract_table_from_req(&req, body);

    req.state()
        .lua
        .send(LuaMessage::Table(table))
        .from_err()
        .and_then(|res| match res {
            LuaMessage::String(s) => Ok(HttpResponse::Ok().body(s)),
            LuaMessage::Table(params) => {
                let mut response = HttpResponse::Ok();

                let body = match params.get("body") {
                    Some(LuaMessage::String(body)) => body.to_owned(),
                    Some(ref value @ _) => unimplemented!("Invalid body: {:?}", value),
                    None => String::new(),
                };

                if let Some(LuaMessage::Table(headers)) = params.get("headers") {
                    for (key, value) in headers.iter() {
                        let value = match value {
                            LuaMessage::String(value) => value.to_owned(),
                            LuaMessage::Number(number) => number.to_string(),
                            LuaMessage::Integer(number) => number.to_string(),
                            ref value @ _ => unimplemented!("Header value is not supported: {:?}", value),
                        };
                        response.header(key as &str, value);
                    }
                }

                match params.get("status") {
                    Some(LuaMessage::String(string)) => {
                        let number: u16 = string.parse()
                            .expect("Invalid response status");
                        let status = http::StatusCode::from_u16(number)
                            .expect("Invalid response status");
                        response.status(status);
                    },
                    Some(LuaMessage::Integer(number)) => {
                        let status = http::StatusCode::from_u16(*number as u16)
                            .expect("Invalid response status");
                        response.status(status);
                    },
                    _ => (),
                }

                Ok(response.body(body))
            },
            LuaMessage::Nil => {
                Ok(HttpResponse::NotFound().finish())
            },
            _ => unimplemented!("Response {:?} is not supported", res),
        })
        .responder()
}