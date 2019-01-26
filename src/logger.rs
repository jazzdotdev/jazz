
use fern::Dispatch;
use std::path::Path;
use std::fs::{File, create_dir, OpenOptions};
use std::fmt::Display;
use colored::*;
use crate::{error::Error, Result};
pub use log::{Level, LevelFilter};

#[derive(Copy, Clone)]
pub struct Settings {
    pub level: LevelFilter,
    pub everything: bool,
}

pub fn get_log_file (path: &Path) -> Result<File> {
    let now = ::chrono::Local::now().format("%Y_%m_%d__%H_%M_%S");

    if path.exists() {
        if !path.is_dir() {
            return Err(Error::from(format!("{:?} is not a directory", path)));
        }
    } else {
        create_dir(path).map_err(|e| Error::from(format!("could not create directory {:?}: {}", path, e)))?;
    }

    let mut path_buf = path.to_path_buf();
    path_buf.push(&now.to_string());
    path_buf.set_extension("log");

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path_buf)
        .map_err(|e| Error::from(format!("could not log to {:?}: {}", &path_buf, e)))
}

pub fn init<P: AsRef<Path>>(path: Option<P>, settings: Settings) {

    let colors = ::fern::colors::ColoredLevelConfig::new()
        .trace(Color::Blue)
        .debug(Color::BrightBlue)
        .info(Color::BrightGreen)
        .warn(Color::BrightYellow)
        .error(Color::BrightRed);

    fn white <T: Display> (msg: T, colored: bool) -> String {
        if colored {
            format!("{}", format!("{}", msg).white())
        } else {
            format!("{}", msg)
        }   
    }

    macro_rules! format_msg {
        ($colored:expr) => {
            move |out, message, record| {
                out.finish(format_args!(
                    "{} {}:{} {}",
                    white(::chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), $colored),
                    if $colored {
                        format!("{}", colors.color(record.level()))
                    } else {
                        format!("{}", record.level())
                    },
                    if settings.everything {
                        white(format!(" {}", record.target()), $colored)
                    } else { "".to_owned() },
                    message
                ))
            }
        }
    }

    // CMD Logging (colored, with user specified level)
    let mut dispatch = Dispatch::new()
        .filter(move |metadata| {
            settings.everything || &metadata.target()[0..9] == "torchbear"
        })
        .level(settings.level)
        .chain(Dispatch::new()
            .format( format_msg!(true) )
            .chain(::std::io::stdout())
        );

    // File Logging (uncolored, only info or worse)
    if let Some(path) = path {
        let path = path.as_ref();
        match ::std::fs::create_dir_all(path) {
            Err(err) => error!("{}", err),
            _ => match get_log_file(path) {
                Ok(file) => {
                    dispatch = dispatch.chain(Dispatch::new()
                        .format(format_msg!(false))
                        .chain(file)
                    );
                },
                Err(err) => error!("{}", err)
            }
        };
    }


    if dispatch.apply().is_err() {
        panic!("A logger instance was already set");
    }
}