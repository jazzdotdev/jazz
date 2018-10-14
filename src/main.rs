extern crate torchbear_lib;
extern crate clap;

use clap::{Arg, App as ClapApp, SubCommand};

fn main() {
    let matches = ClapApp::new("actix-lua-web")
        .version("0.1")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("log scope")
           .long("log-scope")
           .value_name("SCOPE")
           .help("Wether to log everything in the dependency tree")
           .default_value("torchbear")
           .takes_value(true))
        .get_matches();

    torchbear_lib::start();
}
