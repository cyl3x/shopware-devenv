use nix::sys::signal::{kill, Signal::SIGINT};
use nix::unistd::Pid;
use std::fs;

pub fn main(_verbose: bool) {
    let string_pid =
        fs::read_to_string(".devenv/state/devenv.pid").expect("Cannot read pid from pidfile");

    let pid: i32 = string_pid
        .lines()
        .next()
        .expect("Malformed pidfile")
        .parse::<i32>()
        .expect("Malformed pid in pidfile");

    let result = kill(Pid::from_raw(pid), SIGINT);

    if result.is_ok() {
        println!("devenv stopped");
    } else {
        println!("devenv is not running");
    }
}
