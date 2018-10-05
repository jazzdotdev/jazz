
use fern::Dispatch;
use std::path::Path;
use std::fs::{File, create_dir, OpenOptions};

pub use log::{Level, LevelFilter};

pub fn get_log_file (path: &Path) -> Result<File, String> {
    let now = ::chrono::Local::now().format("%Y_%m_%d__%H_%M_%S");

    if path.exists() {
        if !path.is_dir() {
            return Err(format!("{:?} is not a directory", path));
        }
    } else {
        match create_dir(path) {
            Err(e) => return Err(
                format!("could not create directory {:?}: {}", path, e)
            ), Ok(_) => {},
        }
    }

    let mut path_buf = path.to_path_buf();
    path_buf.push(&now.to_string());
    path_buf.set_extension("log");

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path_buf)
        .map_err(|e| format!("could not log to {:?}: {}", &path_buf, e))
}

pub fn init (path: &Path, level: LevelFilter) {

    let colors = ::fern::colors::ColoredLevelConfig::new();

    // CMD Logging (colored, with user specified level)
    let mut dispatch = Dispatch::new()
        .chain(Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{} {}: {}",
                    ::chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                    colors.color(record.level()),
                    message
                ))
            })
            .level(level)
            .chain(::std::io::stdout())
        );

    // File Logging (uncolored, only info or worse)
    let file_err = match get_log_file(path) {
        Ok(file) => {
            dispatch = dispatch.chain(Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{} {}: {}",
                        ::chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                        record.level(),
                        message
                    ))
                })
                .level(LevelFilter::Info)
                .chain(file)
            );
            None
        },
        Err(err) => Some(err)
    };

    if dispatch.apply().is_err() {
        panic!("A logger instance was already set");
    }

    match file_err {
        Some(err) => error!("{}", err),
        None => {}
    }
}