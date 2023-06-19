use directories::ProjectDirs;
use once_cell::sync::OnceCell;
use sha2::{Digest, Sha256};
use spinoff::Spinner;

use crate::{fail, ExitCode};

pub static mut SPINNER: OnceCell<Spinner> = OnceCell::new();

#[macro_export]
macro_rules! spinner {
    ($($str:tt)+) => {
        $crate::utils::_macro_spinner_start(format!($($str)+))
    };
}

#[macro_export]
macro_rules! spinner_stop {
    () => {
        $crate::utils::_macro_spinner_stop(None)
    };
    ($($str:tt)+) => {
        $crate::utils::_macro_spinner_stop(Some(&format!($($str)+)))
    };
}

#[macro_export]
macro_rules! sha256 {
    ($($str:tt)+) => {
        $crate::utils::_macro_sha256(&format!($($str)+))
    }
}

#[macro_export]
macro_rules! project_dirs {
    () => {
        $crate::utils::_macros_project_dirs()
    };
}

pub fn _macro_sha256(string: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(string);
    let result = hasher.finalize();

    format!("{result:x}")
}

pub fn _macro_spinner_start(msg: String) {
    _macro_spinner_stop(None);

    unsafe {
        let _ = SPINNER.set(Spinner::new(
            spinoff::spinners::Dots,
            msg,
            spinoff::Color::Blue,
        ));
    }
}

pub fn _macro_spinner_stop(msg: Option<&str>) {
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

pub fn uid() -> u32 {
    unsafe { libc::geteuid() }
}

