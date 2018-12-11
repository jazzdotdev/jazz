extern crate torchbear_lib;
#[macro_use]
extern crate clap;
extern crate log;

use clap::{Arg, App as ClapApp};

fn main() {
    let matches = ClapApp::new("torchbear")
        .version(crate_version!())
        .author(crate_authors!())
        .about("TorchBear Application Framework")
                .arg(Arg::with_name("log")
           .long("log")
           .value_name("LEVEL")
           .help("Prints messages with log level <LEVEL>")
           .default_value("info")
           .takes_value(true))
        .arg(Arg::with_name("log scope")
           .long("log-scope")
           .value_name("SCOPE")
           .help("Whether to log everything in the dependency tree")
           .default_value("torchbear")
           .takes_value(true))
        .arg(Arg::with_name("run")
           .long("run")
           .value_name("COMMAND")
           .help("Command to run")
           .takes_value(true))
        .get_matches();

    torchbear_lib::ApplicationBuilder::new()
        .log_level(match matches.value_of("log").unwrap() {
            "error" => log::Level::Error,
            "warn" => log::Level::Warn,
            "info" => log::Level::Info,
            "debug" => log::Level::Debug,
            "trace" => log::Level::Trace,
            l => {
                println!("{} is not a valid log level, available levels are:\n\terror, warn, info, debug or trace", l);
                std::process::exit(1)
            }
        })
        .log_everything(match matches.value_of("log scope").unwrap() {
            "torchbear" => false,
            "everything" => true,
            l => {
                println!("{} is not a valid log scope, available levels are 'torchbear' and 'everything'", l);
                std::process::exit(1)
            }
        })
        .runcommand(matches.value_of("run"))
        .start()
}
