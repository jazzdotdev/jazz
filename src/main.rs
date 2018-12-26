extern crate torchbear_lib;
#[macro_use]
extern crate clap;
extern crate log;

use clap::{Arg, App as ClapApp};
use std::{
    io,
    collections::HashMap
};
use torchbear_lib::error::Error;

fn main() {

    let levels: HashMap<&str, log::Level> = [
        ("error", log::Level::Error),
        ("warn",  log::Level::Warn),
        ("info",  log::Level::Info),
        ("debug", log::Level::Debug),
        ("trace", log::Level::Trace)
    ].iter().cloned().collect();

    let matches = ClapApp::new("torchbear")
        .version(crate_version!())
        .author(crate_authors!())
        .about("TorchBear Application Framework")
        .setting(clap::AppSettings::TrailingVarArg)
        .arg(Arg::with_name("log")
            .long("log")
            .value_name("LEVEL")
            .help("Prints messages with log level <LEVEL>")
            .possible_values(&levels.iter().map(|(&k, _)| k).collect::<Vec<_>>()[..])
            .default_value("info")
            .takes_value(true))
        .arg(Arg::with_name("log scope")
            .long("log-scope")
            .value_name("SCOPE")
            .help("Whether to log everything in the dependency tree")
            .possible_values(&["torchbear", "everything"])
            .default_value("torchbear")
            .takes_value(true))
        .arg(Arg::with_name("interpreter")
            .index(1)
            .multiple(true))
        .get_matches();

    match torchbear_lib::ApplicationBuilder::new()
        .log_level(*matches.value_of("log").map(|l| levels.get(&l).unwrap()).unwrap())
        .log_everything(matches.value_of("log scope").unwrap() == "everything")
        .start(matches.values_of("interpreter").map(|val| val.map(|s| s.to_string()).collect())) {
        Ok(_) => {},
        Err(e) => {
            //To handle "AddrInUse". Will move this away in a later commit when refactoring
            if let Error::IoError(err) = e {
                if err.kind() == io::ErrorKind::AddrInUse {
                    println!("Error: Address already in use.");
                } else {
                    println!("Unknown error as occurred: {:?}", err);
                }
            } else {
                println!("Error: {}", e);
            }

        }
    }

}
