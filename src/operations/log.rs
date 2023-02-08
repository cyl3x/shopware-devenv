use std::fs;

use crate::internal::DEVENV_LOG;

pub fn main(_verbose: bool) {
    let out = fs::read_to_string(DEVENV_LOG).expect("Cannot read out log");
    println!("{out}");
}
