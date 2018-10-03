extern crate torchbear_lib;
extern crate env_logger;
extern crate clap;

use clap::{Arg, App as ClapApp, SubCommand};

fn main() {
    let matches = ClapApp::new("actix-lua-web")
        .version("0.1")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .get_matches();

    env_logger::init();
    torchbear_lib::start();
}
