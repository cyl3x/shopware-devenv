use std::fs;

use crate::crash;
use crate::internal::AppExitCode;
use crate::operations::DEVENV_LOG;

pub fn main() {
    let Ok(out) = fs::read_to_string(DEVENV_LOG) else {
        crash!(AppExitCode::DevenvOnce, "Devenv has not been started yet");
    };

    println!("{out}");
}
