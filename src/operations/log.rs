use std::fs;

use crate::crash;
use crate::internal::{AppExitCode, DEVENV_LOG};

pub fn main(_verbose: bool) {
    let Ok(out) = fs::read_to_string(DEVENV_LOG) else {
        crash!(AppExitCode::DevenvOnce, "Devenv has not been started yet");
    };

    println!("{out}");
}
