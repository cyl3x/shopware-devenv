use colored::Colorize;
use once_cell::sync::OnceCell;

use super::Config;

static LOGGER: OnceCell<Logger> = OnceCell::new();

pub struct Logger {
    verbose: bool,
}

impl Logger {
    /// Initialize the logger with the given verbosity
    pub fn init(arg_verbose: bool) {
        let verbose = arg_verbose || Config::get().verbose;
        let _ = LOGGER.set(Self { verbose });
    }

    /// Get a reference to the logger.
    /// Returns a default Logger if no one as initialized
    pub fn get() -> &'static Self {
        LOGGER.get_or_init(|| Self { verbose: false })
    }

    pub fn verbose(&self, msg: &str, file: &str, line: u32) {
        if self.verbose {
            eprintln!("\r[{}] [{file}:{line}] {msg}", "verbose".red());
        }
    }

    pub fn info(msg: &str) {
        eprintln!("\r{} {msg}", "!".bold().yellow());
    }
}
