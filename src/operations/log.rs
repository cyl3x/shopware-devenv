use std::fs;

use crate::fail;
use crate::internal::{ExitCode, LOG_FILE};

pub fn main() {
    let Ok(out) = fs::read_to_string(&*LOG_FILE) else {
        fail!(ExitCode::DevenvOnce, "Devenv has not been started yet");
    };

    println!("{out}");
}
