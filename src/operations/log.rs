use std::fs;

use crate::fail;
use crate::internal::{AppExitCode, LOG_FILE};

pub fn main() {
    let Ok(out) = fs::read_to_string(&*LOG_FILE) else {
        fail!(AppExitCode::DevenvOnce, "Devenv has not been started yet");
    };

    println!("{out}");
}
