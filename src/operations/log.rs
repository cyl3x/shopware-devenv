use std::fs;

use crate::{topic, Context, OrFail};

pub fn main() -> anyhow::Result<String> {
    let log_file = Context::get()?.log_file();

    topic!("Opening logfile at {}...", log_file.display());

    let log = fs::read_to_string(log_file).or_error("Devenv has not been started yet")?;

    println!("{log}");

    Ok(String::new())
}
