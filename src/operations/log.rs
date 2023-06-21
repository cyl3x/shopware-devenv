use std::fs;

use crate::{topic, OrFail, LOG_FILE};

pub fn main() {
    topic!("Opening logfile at {}...", LOG_FILE.display());
    println!(
        "{}",
        fs::read_to_string(&*LOG_FILE).or_fail("Devenv has not been started yet")
    );
}
