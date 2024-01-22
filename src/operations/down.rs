use sysinfo::{Pid, ProcessExt, Signal, System, SystemExt};

use crate::{topic, Context};

pub fn main() -> anyhow::Result<String> {
    topic!("Stopping...");

    let mut sys = System::new();
    sys.refresh_processes();

    Context::get()?
        .devenv_pid()
        .map_or_else(|_| anyhow::bail!("Unable to find devenv's process"), |pid| down_by_pid(&sys, pid))?;

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
