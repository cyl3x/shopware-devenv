use std::fs;

use sysinfo::{Pid, ProcessExt, Signal, System, SystemExt};

use crate::config::Config;
use crate::log;

pub fn main(config: &Config) {
    // TODO - another instance is running
    let mut sys = System::new();
    sys.refresh_all();

    let pid_file = fs::read_to_string(".devenv/state/devenv.pid");

    let success: bool = match pid_file {
        Ok(pid_string) => down_by_pid(config, &sys, &pid_string),
        Err(_) => down_by_process(config, &mut sys),
    };

    if success {
        println!("devenv stopped");
    } else {
        println!("devenv is not running");
    }
}

fn down_by_pid(config: &Config, sys: &System, pid_string: &str) -> bool {
    let pid: usize = pid_string
        .lines()
        .next()
        .expect("Malformed pidfile")
        .parse::<usize>()
        .expect("Malformed pid in pidfile");

    log!(config, "Found pid ({pid}) in pidfile, stopping..");

    sys.process(Pid::from(pid))
        .and_then(|p| p.kill_with(Signal::Interrupt))
        .is_some()
}

fn down_by_process(config: &Config, sys: &mut System) -> bool {
    log!(config, "Missing pidfile, try to interrupt..");

    let mut success = false;
    for p in sys.processes_by_name(".honcho-wrapped") {
        if p.kill_with(Signal::Interrupt).is_some() {
            success = true;
        }
    }

    success
}
