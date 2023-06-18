use std::io::{self, Write};

use colored::Colorize;
use env_logger::fmt::Formatter;
use log::{Level, Record};

pub fn init(log_level_filter: log::LevelFilter) {
    env_logger::Builder::new()
        .format_timestamp(None)
        .format(format)
        .filter_level(log_level_filter)
        .init();
}

fn format(buf: &mut Formatter, record: &Record<'_>) -> io::Result<()> {
    let level = match record.level() {
        l @ Level::Error => l.as_str().bright_red(),
        l @ Level::Warn => l.as_str().yellow(),
        l @ Level::Info => l.as_str().green(),
        l @ Level::Debug => l.as_str().blue(),
        l @ Level::Trace => l.as_str().cyan(),
    };

    let base_name = format!("{}::", env!("CARGO_PKG_NAME"));
    let module = record.target().replace(&base_name, "").bold();

    writeln!(buf, "[{level}] [{module}] {}", record.args())
}
