use sysinfo::{Pid, ProcessesToUpdate};

use crate::{devenv, Command, Context};

pub fn main() -> anyhow::Result<String> {
    if !is_devenv_running()? {
        anyhow::bail!("Devenv service is not running")
    };
    
    devenv!["processes", "down"].await_success()?;

    Ok(String::new())
}

fn is_devenv_running() -> anyhow::Result<bool> {
    let Ok(pid) = Context::get()?.devenv_pid() else {
        return Ok(false);
    };

    let pid = Pid::from(pid);

    let mut sys = sysinfo::System::new();
    sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), false);

    Ok(sys.process(pid).is_some())
}
