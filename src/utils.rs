use std::io;
use std::io::Write;

use directories::ProjectDirs;
use sha2::{Digest, Sha256};

use crate::fail;

#[macro_export]
macro_rules! sha256 {
    ($($str:tt)+) => {
        $crate::utils::_macro_sha256(&format!($($str)+))
    }
}

#[macro_export]
macro_rules! y_or_n {
    ($($str:tt)+) => {
        $crate::utils::_macro_y_or_n(&format!($($str)+))
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

pub fn _macros_project_dirs() -> ProjectDirs {
    ProjectDirs::from("de", "cyl3x", env!("CARGO_PKG_NAME"))
        .or_fail("Failed to retrieve paths from os")
}

pub fn _macro_y_or_n(msg: &str) -> bool {
    print!("{msg} [y/N]: ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        fail!("Failed to read input: {e}")
    }

    input = input.trim().to_lowercase();

    input == "y" || input == "yes"
}

pub fn uid() -> u32 {
    unsafe { libc::geteuid() }
}

pub trait OrFail<T> {
    fn or_fail(self, msg: &str) -> T;
}

impl<T, E> OrFail<T> for Result<T, E> {
    fn or_fail(self, msg: &str) -> T {
        self.unwrap_or_else(|_| fail!("{msg}"))
    }
}

impl<T> OrFail<T> for Option<T> {
    fn or_fail(self, msg: &str) -> T {
        self.unwrap_or_else(|| fail!("{msg}"))
    }
}
