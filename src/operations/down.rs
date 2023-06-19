use std::fs;
use std::process::Command;

use sysinfo::{Pid, ProcessExt, Signal, System, SystemExt};

use crate::{fail, spinner, success, ExitCode, DEVENV_PID};

pub fn main() {
    spinner!("Stopping...");

    let mut sys = System::new();
    sys.refresh_processes();

    let pid_file = fs::read_to_string(&*DEVENV_PID);

    let success: bool = pid_file.map_or_else(|_| down_by_process(), |pid_string| down_by_pid(&sys, &pid_string));

    if success {
        success!("Devenv service stopped");
    } else {
        fail!(ExitCode::DevenvStop, "Devenv service is not running");
    }
}

fn down_by_pid(sys: &System, pid_string: &str) -> bool {
    let Some(pid) = pid_string
        .lines()
        .next()
        .and_then(|p| p.parse::<usize>().ok()) else {
            fail!(ExitCode::Runtime, "Malformed pid or pidfile")
        };

    log::info!("Found pid ({pid}) in pidfile, stopping..");

    if let Some(p) = sys.process(Pid::from(pid)) {
        if p.kill_with(Signal::Interrupt).is_some() {
            p.wait();
            return true;
        }
    }

    false
}

fn down_by_process() -> bool {
    log::info!("Missing pidfile, try to interrupt..");
    // TODO - Ask user to proceed if there are multiple processes
    println!("Cannot find pidfile, trying to stop by process name. This can potentially stop other devenv processes as well.");

    Command::new("bash")
        .args(["-c", r#""kill $(ps -ax | grep /nix/store  | awk '{print $1}')""#])
        .spawn()
        .and_then(|mut c| c.wait())
        .is_ok()
}
