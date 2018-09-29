extern crate actix_lua_web;
extern crate env_logger;
extern crate clap;

use clap::{Arg, App as ClapApp, SubCommand};

fn main() {
    let matches = ClapApp::new("myapp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("setting")
           .long("setting")
           .value_name("SETTING_FILE")
           .index(1)
           .help("Selects the setting toml file")
           .takes_value(true))
        .get_matches();

    let setting_dir = matches.value_of("setting").unwrap_or("Setting.toml");

    env_logger::init();
    actix_lua_web::start_from_settings(setting_dir);
}
