use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::thread::sleep;
use std::time::Duration;

use regex::Regex;
use sysinfo::{Pid, SystemExt};

use crate::{devenv, topic, Command, Context};

pub fn main() -> anyhow::Result<String> {
    topic!("Starting...");

    check_running_instances()?;

    Context::get()?.platform.move_cwd();

    let log_file = Context::get()?.log_file();

    println!("Creating logfile at {}...", log_file.display());
    let mut log = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .truncate(true)
        .open(log_file)?;

    topic!("Starting devenv process...");
    let mut child = devenv!["up"]
        .stdout(log.try_clone()?)
        .stderr(log.try_clone()?)
        .start();

    topic!("Waiting for successful start...");
    let success = check_successfull_start(&mut log)?;

    if !success {
        let _ = child.wait();

        super::log::main()?;

        anyhow::bail!("Starting devenv resulted in an error.");
    }

    Ok("Devenv process started".into())
}

fn check_running_instances() -> anyhow::Result<bool> {
    let Ok(pid) = Context::get()?.devenv_pid() else {
        return Ok(false);
    };

    let mut sys = sysinfo::System::new();
    sys.refresh_processes();

    if sys.process(Pid::from(pid)).is_some() {
        anyhow::bail!("Devenv service is already running")
    }

    Ok(true)
}

fn check_successfull_start(file: &mut File) -> anyhow::Result<bool> {
    let error_condition = Regex::new(r"(.*:.*system.*\|.*sending SIGTERM to)|(^error:$)")?;
    let mut contents = vec![];
    let mut text: String;
    let mut pos: usize = 0;
    let mut unchanged: usize = 0;

    for _ in 0..40 {
        contents.truncate(0);

        file.seek(SeekFrom::Start(pos as u64))?;
        pos += file.read_to_end(&mut contents)?;
        text = String::from_utf8_lossy(&contents).to_string();

        if text.is_empty() {
            if pos > 50 {
                unchanged += 1;
            }

            if unchanged > 8 {
                break;
            }
        } else {
            unchanged = 0;

            if text.lines().any(|l| error_condition.is_match(l)) {
                return Ok(false);
            };
        }

        sleep(Duration::from_millis(250));
    }

    Ok(true)
}
