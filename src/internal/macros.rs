use std::env::vars_os;
use std::process::{exit, Command};

use colored::Colorize;
use sha2::{Digest, Sha256};
use spinoff::{spinners, Color, Spinner};

use crate::context::Context;
use crate::internal::AppExitCode;

const ERR_SYMBOL: &str = "✕";
const FINISH_SYMBOL: &str = "✓";

#[macro_export]
macro_rules! spinner {
    ($msg:expr) => {
        $crate::internal::spinner_fn($msg)
    }
}

#[macro_export]
macro_rules! sha256 {
    ($($str:tt)+) => {
        $crate::internal::sha256_fn(&format!($($str)+))
    }
}

#[macro_export]
macro_rules! devenv {
    ($($cmd:tt)+) => {
        $crate::internal::devenv_fn(&format!($($cmd)+))
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)+) => {
        $crate::Logger::get().log(&format!($($arg)+), file!(), line!());
    }
}

#[macro_export]
macro_rules! fail {
    ($exit_code:expr, $($arg:tt)+) => {
        $crate::internal::fail_fn(&format!($($arg)+), $exit_code)
    }
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)+) => {
        $crate::internal::success_fn(&format!($($arg)+))
    }
}

pub fn fail_fn(msg: &str, exit_code: AppExitCode) -> ! {
    println!("{} {}", ERR_SYMBOL.red(), msg.bold());
    exit(exit_code as i32);
}

pub fn devenv_fn(cmd: &str) -> Command {
    log!("[{}] {}", "devenv".green(), cmd);

    Context::get().platform.move_to();

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

pub fn success_fn(msg: &str) -> ! {
    println!("{} {}", FINISH_SYMBOL.green(), msg.bold());
    exit(0);
}

pub fn spinner_fn(msg: &'static str) -> Spinner {
    Spinner::new(spinners::Dots, msg, Color::Blue)
}
