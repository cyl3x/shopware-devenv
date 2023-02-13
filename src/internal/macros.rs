use std::env::vars_os;
use std::process::{exit, Command};

use crossterm::style::Stylize;
use sha2::{Digest, Sha256};

use crate::internal::AppExitCode;

const ERR_SYMBOL: &str = "✕";

#[macro_export]
macro_rules! sha256 {
    ($($str:tt)+) => {
        $crate::internal::macros::sha256_fn(&format!($($str)+))
    }
}

#[macro_export]
macro_rules! devenv {
    ($verbose:expr, $($cmd:tt)+) => {
        $crate::internal::macros::devenv_fn(&format!($($cmd)+), $verbose)
    }
}

#[macro_export]
macro_rules! log {
    ($verbose:expr, $($arg:tt)+) => {
        $crate::internal::macros::log_fn(&format!("[{}:{}] {}", file!(), line!(), format!($($arg)+)), $verbose);
    }
}

#[macro_export]
macro_rules! crash {
    ($exit_code:expr, $($arg:tt)+) => {
        $crate::internal::macros::crash_fn(&format!($($arg)+), $exit_code)
    }
}

pub fn crash_fn(msg: &str, exit_code: AppExitCode) {
    println!("{} {}", ERR_SYMBOL.red(), msg.bold());
    exit(exit_code as i32);
}

pub fn log_fn(msg: &str, verbose: bool) {
    if verbose {
        eprintln!("[{}] {msg}", "verbose".red());
    }
}

pub fn devenv_fn(cmd: &str, verbose: bool) -> Command {
    log!(verbose, "[{}] {}", "devenv".green(), cmd);

    let mut command = Command::new("devenv");
    command
        .args(["shell", "bash", "-c", cmd])
        .envs(&mut vars_os());

    command
}

pub fn sha256_fn(string: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(string);
    let result = hasher.finalize();

    format!("{result:x}")
}
