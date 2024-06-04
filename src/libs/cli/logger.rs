use log::LevelFilter;
use simplelog::{Config, SimpleLogger};

pub fn init_logger(is_quiet: bool) {
    let level = if is_quiet {
        LevelFilter::Error
    } else {
        LevelFilter::Info
    };
    SimpleLogger::init(level, Config::default()).unwrap();
}
