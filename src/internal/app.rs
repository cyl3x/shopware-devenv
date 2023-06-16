use std::process::exit;

use colored::Colorize;
use directories::ProjectDirs;

use crate::spinner_stop;

const ERR_SYMBOL: &str = "✕";
const FINISH_SYMBOL: &str = "✓";

pub enum ExitCode {
    RunAsRoot = 1,
    InvalidArgs = 2,
    AppDirsCreation = 3,

    Runtime = 9,

    // Devenv
    DevenvStart = 10,
    DevenvStop = 11,
    DevenvOnce = 12,
    DevenvExec = 13,

    // Config
    ConfigWrite = 20,
    ConfigBak = 21,

    // Context
    InvalidContext = 30,
}

#[macro_export]
macro_rules! fail {
    ($exit_code:expr, $($arg:tt)+) => {
        $crate::internal::_macro_fail(&format!($($arg)+), $exit_code)
    };
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)+) => {
        $crate::internal::_macro_success(&format!($($arg)+))
    }
}

#[macro_export]
macro_rules! project_dirs {
    () => {
        $crate::internal::_macros_project_dirs()
    };
}

pub fn _macro_success(msg: &str) -> ! {
    let message = format!("{} {}", FINISH_SYMBOL.green(), msg.bold());

    if unsafe { super::SPINNER.get().is_some() } {
        spinner_stop!("\r{message}");
    } else {
        println!("\r{message}");
    }

    exit(0);
}

pub fn _macro_fail(msg: &str, exit_code: ExitCode) -> ! {
    let message = format!("{} {}", ERR_SYMBOL.red(), msg.bold());

    if unsafe { super::SPINNER.get().is_some() } {
        spinner_stop!("\r{message}");
    } else {
        println!("\r{message}");
    }

    exit(exit_code as i32);
}

pub fn _macros_project_dirs() -> ProjectDirs {
    let Some(dirs) = ProjectDirs::from(
        "de",
        "cyl3x",
        env!("CARGO_PKG_NAME")
    ) else {
        fail!(ExitCode::Runtime, "Failed to retrieve paths from os");
    };

    dirs
}
