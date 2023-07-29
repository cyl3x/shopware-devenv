use sysinfo::{Pid, ProcessExt, Signal, System, SystemExt};

use crate::{topic, y_or_n, Context};

pub fn main() -> anyhow::Result<String> {
    topic!("Stopping...");

    let mut sys = System::new();
    sys.refresh_processes();

    Context::get()?
        .devenv_pid()
        .map_or_else(|_| down_by_process(&sys), |pid| down_by_pid(&sys, pid))?;

    Ok("Devenv service stopped".into())
}

fn down_by_pid(sys: &System, pid: usize) -> anyhow::Result<()> {
    topic!("Found pid ({pid}) in pidfile, stopping...");

    if let Some(process) = sys.process(Pid::from(pid)) {
        if process.kill_with(Signal::Interrupt).unwrap_or_default() {
            process.wait();
            return Ok(());
        }
    }

    anyhow::bail!("Devenv was not running");
}

fn down_by_process(sys: &System) -> anyhow::Result<()> {
    topic!("Missing pidfile, choose...");
    println!("Pidfile was not found, but you can kill all Nix processes to stop devenv");

    if !y_or_n!("Kill all Nix processes?") {
        anyhow::bail!("Aborted killing all Nix processes");
    }

    sys.processes()
        .values()
        .filter(|process| {
            if !process.exe().starts_with("/nix/store") {
                return false;
            }

            println!(
                "Killing {} {}...",
                process.pid(),
                process.exe().to_string_lossy()
            );

            process.kill_with(Signal::Interrupt).unwrap_or_default()
        })
        .for_each(sysinfo::ProcessExt::wait);

    Ok(())
}
