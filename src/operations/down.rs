use std::fs;

use sysinfo::{Pid, ProcessExt, Signal, System, SystemExt};

use super::DEVENV_PID;
use crate::internal::AppExitCode;
use crate::{crash, finish, log};

pub fn main() {
    let mut sys = System::new();
    sys.refresh_processes();

    let pid_file = fs::read_to_string(DEVENV_PID.clone());

    let success: bool = match pid_file {
        Ok(pid_string) => down_by_pid(&sys, &pid_string),
        Err(_) => down_by_process(&mut sys),
    };

    if success {
        finish!("Devenv service stopped");
    } else {
        crash!(AppExitCode::DevenvStop, "Devenv service is not running");
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

    sys.process(Pid::from(pid))
        .and_then(|p| p.kill_with(Signal::Interrupt))
        .is_some()
}

fn down_by_process(sys: &mut System) -> bool {
    log!("Missing pidfile, try to interrupt..");

    let mut success = false;
    for p in sys.processes_by_name(".honcho-wrapped") {
        if p.kill_with(Signal::Interrupt).is_some() {
            success = true;
        }
    }

    success
}
