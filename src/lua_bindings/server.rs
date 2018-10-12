use std::collections::HashMap;
use actix_lua::{LuaMessage};
use actix_web::{
    http, AsyncResponder,
    FutureResponse, HttpResponse, HttpMessage, HttpRequest,
};
use futures::Future;
use serde_urlencoded;
use serde_json;

use ::app_state::AppState;

/// Creates a lua table from a HttpRequest
fn extract_table_from_request(request: &HttpRequest<AppState>, body: String) -> HashMap<String, LuaMessage> {
    let mut table = HashMap::new();

    let query: HashMap<_, _> = request.query().iter()
        .map(|(key, value)| (key.clone(), LuaMessage::String(value.clone())))
        .collect();
    let headers: HashMap<_, _> = request.headers().iter()
        .map(|(key, value)| (
            key.as_str().to_owned(),
            LuaMessage::String(value.to_str().unwrap().to_owned()),
        ))
        .collect();
    let host = request.uri().host()
        .map(|host| LuaMessage::String(host.to_owned()))
        .unwrap_or(LuaMessage::Nil);
    let fragment = request.uri().to_string()
        .rsplit("#")
        .next()
        .map(|fragment| LuaMessage::String(fragment.to_owned()))
        .unwrap_or(LuaMessage::Nil);
    let path = request.path().to_string();

    let request_line = if request.query_string().is_empty() {
        format!(
            "{} {} {:?}",
            request.method(),
            request.path(),
            request.version()
        )
    } else {
        format!(
            "{} {}?{} {:?}",
            request.method(),
            request.path(),
            request.query_string(),
            request.version()
        )
    };

    let body_hashmap: Option<HashMap<String, String>> =
        match request.headers().get(::actix_web::http::header::CONTENT_TYPE).map(|h| h.to_str()) {
            Some(Ok("application/x-www-form-urlencoded")) => serde_urlencoded::from_str(&body).ok(),
            Some(Ok("application/json")) => serde_json::from_str(&body).ok(),
            Some(Ok(header)) => {
                eprintln!("content type {} not supported", header);
                None
            },
            Some(err@Err(_)) => {
                eprintln!("could not parse content type into a valid string: {:?}", err);
                None
            },
            _ => None
        };

    table.insert("body".to_owned(), match body_hashmap {
        Some(body_hashmap) => {
            LuaMessage::Table(
                body_hashmap
                    .into_iter()
                    .map(|(k, v)| (k, LuaMessage::String(v)))
                    .collect()
            )
        },
        _ => LuaMessage::String(body.clone())
    });

    table.insert("request_line".to_owned(), LuaMessage::String(request_line));
    table.insert("method".to_owned(), LuaMessage::String(request.method().to_string()));
    table.insert("headers".to_owned(), LuaMessage::Table(headers));
    table.insert("query".to_owned(), LuaMessage::Table(query));
    table.insert("host".to_owned(), host);
    table.insert("fragment".to_owned(), fragment);
    table.insert("path".to_owned(), LuaMessage::String(path));
    table.insert("body_raw".to_owned(), LuaMessage::String(body));

    table
}

pub fn handler((request, body): (HttpRequest<AppState>, String)) -> FutureResponse<HttpResponse> {
    let table = extract_table_from_request(&request, body);

    request.state()
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