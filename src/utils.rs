use std::io;
use std::io::Write;

use color_eyre::owo_colors::OwoColorize;
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

pub fn _macro_sha256(string: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(string);
    let result = hasher.finalize();

    format!("{result:x}")
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
    /// # Errors
    /// Convert a Result or Option to an `anyhow::Result`<T> with a custom error
    /// message
    fn or_error(self, msg: &str) -> anyhow::Result<T>;
    fn or_panic(self, msg: Option<&str>) -> T;
}

impl<T, E> OrFail<T> for Result<T, E>
where
    E: std::fmt::Debug,
{
    fn or_error(self, msg: &str) -> anyhow::Result<T> {
        self.map_err(|e| anyhow::anyhow!("{msg}\n   Reason: {e:?}"))
    }

    fn or_panic(self, msg: Option<&str>) -> T {
        self.unwrap_or_else(|e| {
            let e = format!("{e:?}");
            msg.map_or_else(
                || panic!("{e:?}"),
                |msg| panic!("{msg}\n{}Error: {}", "".default_color(), e.red()),
            )
        })
    }
}

impl<T> OrFail<T> for Option<T> {
    fn or_error(self, msg: &str) -> anyhow::Result<T> {
        self.ok_or_else(|| anyhow::anyhow!("{msg}"))
    }

    fn or_panic(self, msg: Option<&str>) -> T {
        self.unwrap_or_else(|| msg.map_or_else(|| panic!(), |msg| panic!("{msg}")))
    }
}
