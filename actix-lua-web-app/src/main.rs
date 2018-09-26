extern crate actix_lua_web;
extern crate env_logger;

fn main() {
    env_logger::init();
    actix_lua_web::start_from_settings("Settings.toml");
}
