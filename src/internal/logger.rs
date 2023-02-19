use colored::Colorize;
use once_cell::sync::OnceCell;

use super::Config;

static LOGGER: OnceCell<Logger> = OnceCell::new();

pub struct Logger {
    verbose: bool,
}

impl Logger {
    pub fn init(arg_verbose: bool) {
        let verbose = arg_verbose || Config::get().verbose;
        LOGGER
            .set(Self { verbose })
            .ok()
            .expect("Logger already initialised");
    }

    pub fn get() -> &'static Self {
        LOGGER.get().expect("Logger not initialised")
    }

    pub fn log(&self, msg: &str, file: &str, line: u32) {
        if self.verbose {
            eprintln!("[{}] [{file}:{line}] {msg}", "verbose".red());
        }
    }
}
