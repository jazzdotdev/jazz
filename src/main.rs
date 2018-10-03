extern crate torchbear_lib;
extern crate env_logger;
extern crate clap;

use clap::{Arg, App as ClapApp, SubCommand};

fn main() {
    let matches = ClapApp::new("actix-lua-web")
        .version("0.1")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("path")
           .index(1)
           .help("The torchbear application path")
           .takes_value(true)
           .default_value("./"))
        .get_matches();

    let path = matches.value_of("path").unwrap();

    env_logger::init();
    torchbear_lib::start(path);
}
