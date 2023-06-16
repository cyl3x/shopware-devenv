use once_cell::sync::OnceCell;
use sha2::{Digest, Sha256};
use spinoff::Spinner;

pub static mut SPINNER: OnceCell<Spinner> = OnceCell::new();

#[macro_export]
macro_rules! spinner {
    ($($str:tt)+) => {
        $crate::internal::_macro_spinner_start(format!($($str)+))
    };
}

#[macro_export]
macro_rules! spinner_stop {
    () => {
        $crate::internal::_macro_spinner_stop(None)
    };
    ($($str:tt)+) => {
        $crate::internal::_macro_spinner_stop(Some(&format!($($str)+)))
    };
}

#[macro_export]
macro_rules! sha256 {
    ($($str:tt)+) => {
        $crate::internal::_macro_sha256(&format!($($str)+))
    }
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
