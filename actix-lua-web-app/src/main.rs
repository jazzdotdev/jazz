extern crate actix_lua_web;
extern crate env_logger;

fn main() {
    env_logger::init();

    actix_lua_web::ApplicationBuilder::new()
        .templates_path("templates/**/*")
        .handler_path("lua/handler.lua")
        .lua_modules_path("./lua/?.lua")
        .host("0.0.0.0:3000")
        .start();
}
