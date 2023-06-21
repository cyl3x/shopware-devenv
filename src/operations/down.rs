use std::fs;

use sysinfo::{Pid, ProcessExt, Signal, System, SystemExt};

use crate::{fail, success, topic, y_or_n, OrFail, DEVENV_PID};

pub fn main() {
    topic!("Stopping...");

    let mut sys = System::new();
    sys.refresh_processes();

    let success: bool = fs::read_to_string(&*DEVENV_PID)
        .map_or_else(|_| down_by_process(&sys), |pid| down_by_pid(&sys, &pid));

    if success {
        success!("Devenv service stopped");
    }

    fail!("Devenv service is not running");
}

fn down_by_pid(sys: &System, pid_string: &str) -> bool {
    let pid = pid_string
        .lines()
        .next()
        .and_then(|p| p.parse::<usize>().ok())
        .or_fail("Malformed pid or pidfile");

    topic!("Found pid ({pid}) in pidfile, stopping...");

    if let Some(process) = sys.process(Pid::from(pid)) {
        if process.kill_with(Signal::Interrupt).unwrap_or_default() {
            process.wait();
            return true;
        }
    }

    false
}

fn down_by_process(sys: &System) -> bool {
    topic!("Missing pidfile, choose...");
    println!("Pidfile was not found, but you can kill all nix processes to stop devenv");

    if !y_or_n!("Kill all nix processes?") {
        fail!("Failed to stop devenv");
    }

    sys.processes().values().filter(|process| {
        if !process.exe().starts_with("/nix/store") {
            return false;
        }

        println!("Killing {}...", process.exe().to_string_lossy());

        process.kill_with(Signal::Interrupt).unwrap_or_default()
    }).for_each(sysinfo::ProcessExt::wait);

    true
}
