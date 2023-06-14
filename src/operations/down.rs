use std::fs;

use sysinfo::{Pid, ProcessExt, Signal, System, SystemExt};

use super::DEVENV_PID;
use crate::internal::AppExitCode;
use crate::{fail, log, spinner, success};

pub fn main() {
    let spinner = spinner!("Stopping...");

    let mut sys = System::new();
    sys.refresh_processes();

    let pid_file = fs::read_to_string(DEVENV_PID.clone());

    let success: bool = match pid_file {
        Ok(pid_string) => down_by_pid(&sys, &pid_string),
        Err(_) => down_by_process(&mut sys),
    };

    spinner.clear();
    if success {
        success!("Devenv service stopped");
    } else {
        fail!(AppExitCode::DevenvStop, "Devenv service is not running");
    }
}

fn down_by_pid(sys: &System, pid_string: &str) -> bool {
    let pid: usize = pid_string
        .lines()
        .next()
        .expect("Malformed pidfile")
        .parse::<usize>()
        .expect("Malformed pid in pidfile");

    log!("Found pid ({pid}) in pidfile, stopping..");

    if let Some(p) = sys.process(Pid::from(pid)) {
        if p.kill_with(Signal::Interrupt).is_some() {
            p.wait();
            return true;
        }
    }

    false
}

fn down_by_process(sys: &mut System) -> bool {
    log!("Missing pidfile, try to interrupt..");
    // TODO - Ask user to proceed if there are multiple processes
    println!("Cannot find pidfile, trying to stop by process name. This can potentially stop other devenv processes as well.");

    let mut success = false;
    for p in sys.processes_by_name(".honcho-wrapped") {
        if p.kill_with(Signal::Interrupt).is_some() {
            p.wait();
            success = true;
        }
    }

    success
}
