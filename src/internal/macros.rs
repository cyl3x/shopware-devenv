use std::env::vars_os;
use std::process::{exit, Command};

use colored::Colorize;
use directories::ProjectDirs;
use once_cell::sync::OnceCell;
use sha2::{Digest, Sha256};
use spinoff::Spinner;

use crate::context::Context;
use crate::internal::AppExitCode;

const ERR_SYMBOL: &str = "✕";
const FINISH_SYMBOL: &str = "✓";

static mut SPINNER: OnceCell<Spinner> = OnceCell::new();

#[macro_export]
macro_rules! project_dirs {
    () => {
        $crate::internal::project_dirs_fn()
    };
}

#[macro_export]
macro_rules! spinner {
    ($($str:tt)+) => {
        $crate::internal::spinner_start_fn(format!($($str)+))
    };
}

#[macro_export]
macro_rules! spinner_stop {
    () => {
        $crate::internal::spinner_stop_fn(None)
    };
    ($($str:tt)+) => {
        $crate::internal::spinner_stop_fn(Some(&format!($($str)+)))
    };
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
macro_rules! log_verbose {
    ($($arg:tt)+) => {
        $crate::Logger::get().verbose(&format!($($arg)+), file!(), line!());
    }
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)+) => {
        $crate::Logger::info(&format!($($arg)+));
    }
}

#[macro_export]
macro_rules! fail {
    // ($($arg:tt)+) => {
    //     $crate::internal::fail_fn(&format!($($arg)+), $crate::internal::AppExitCode::Runtime)
    // };
    ($exit_code:expr, $($arg:tt)+) => {
        $crate::internal::fail_fn(&format!($($arg)+), $exit_code)
    };
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)+) => {
        $crate::internal::success_fn(&format!($($arg)+))
    }
}

pub fn fail_fn(msg: &str, exit_code: AppExitCode) -> ! {
    let message = format!("{} {}", ERR_SYMBOL.red(), msg.bold());

    if unsafe { SPINNER.get().is_some() } {
        spinner_stop!("\r{message}");
    } else {
        println!("\r{message}");
    }

    exit(exit_code as i32);
}

pub fn devenv_fn(cmd: &str) -> Command {
    log_verbose!("[{}] {}", "devenv".green(), cmd);

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
    let message = format!("{} {}", FINISH_SYMBOL.green(), msg.bold());

    if unsafe { SPINNER.get().is_some() } {
        spinner_stop!("\r{message}");
    } else {
        println!("\r{message}");
    }

    exit(0);
}

pub fn spinner_start_fn(msg: String) {
    spinner_stop_fn(None);

    unsafe {
        let _ = SPINNER.set(Spinner::new(
            spinoff::spinners::Dots,
            msg,
            spinoff::Color::Blue,
        ));
    }
}

pub fn spinner_stop_fn(msg: Option<&str>) {
    unsafe {
        if let Some(spinner) = SPINNER.take() {
            if let Some(msg) = msg {
                spinner.stop_with_message(msg);
            } else {
                spinner.stop();
            }
        }
    }
}

pub fn project_dirs_fn() -> ProjectDirs {
    let Some(dirs) = ProjectDirs::from(
        "de",
        "cyl3x",
        env!("CARGO_PKG_NAME")
    ) else {
        fail!(AppExitCode::Runtime, "Failed to retrieve paths from os");
    };

    dirs
}
