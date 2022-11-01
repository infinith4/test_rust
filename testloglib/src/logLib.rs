use log::error;
use simplelog::*;

use std::fs::File;

pub struct Logger {}
pub trait LogTrait {
    fn error(&self, error_str: &str) {
        log::error!("{}", error_str);
    }
}
impl LogTrait for Logger {}

pub struct LogLib {
    file_name: String,
}

impl LogLib {
    pub fn new(file_name: &str) -> LogLib {
        CombinedLogger::init(vec![
            TermLogger::new(
                LevelFilter::Warn,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(
                LevelFilter::Info,
                Config::default(),
                File::create(file_name).unwrap(),
            ),
        ])
        .unwrap();

        LogLib {
            file_name: (file_name).to_string(),
        }
    }
}
