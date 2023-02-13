use std::env::vars_os;
use std::fs::File;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use crate::internal::{AppExitCode, DEVENV_LOG};
use crate::{crash, log};

pub fn main(verbose: bool) {
    let log = File::create(DEVENV_LOG).expect("Failed to create out log");

    let mut child = Command::new("devenv")
        .arg("up")
        .envs(&mut vars_os())
        .stdout(log.try_clone().expect("Cannot log into the same file?"))
        .stderr(log)
        .spawn()
        .expect("Failed to start devenv");

    sleep(Duration::from_secs(2));

    if matches!(child.try_wait(), Ok(None)) {
        return;
    }

    log!(verbose, "Devenv has crashed");

    log::main(verbose);

    crash!(
        AppExitCode::DevenvStart,
        "Error while starting devenv. Please run 'devenv up' manually for better visualisation"
    );
}
