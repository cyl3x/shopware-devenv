use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::thread::sleep;
use std::time::Duration;

use regex::Regex;
use sysinfo::{Pid, SystemExt};

use crate::{devenv, fail, success, topic, Command, OrFail, Context, DEVENV_PID, LOG_FILE};

pub fn main() {
    if check_running_instances() {
        fail!("Devenv service is already running");
    }

    Context::get().platform.move_cwd();

    println!("Creating logfile at {}...", LOG_FILE.display());
    let mut log = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .truncate(true)
        .open(&*LOG_FILE)
        .or_fail("Failed to create devenv log file");

    topic!("Starting...");
    let mut child = devenv!("up")
        .stdout(log.try_clone().expect("Cannot log into the same file?"))
        .stderr(log.try_clone().expect("Cannot log into the same file?"))
        .start();

    topic!("Waiting for successful start...");
    let success = check_successfull_start(&mut log);

    if success {
        success!("Devenv service started");
    }

    let _r = child.wait();

    super::log::main();

    fail!("Error while starting devenv.");
}

fn check_running_instances() -> bool {
    if let Ok(pid_string) = fs::read_to_string(DEVENV_PID.clone()) {
        let pid: usize = pid_string
            .lines()
            .next()
            .and_then(|p| p.parse::<usize>().ok())
            .or_fail("Malformed pid in pidfile");

        let mut sys = sysinfo::System::new();
        sys.refresh_processes();

        return sys.process(Pid::from(pid)).is_some();
    }

    false
}

fn check_successfull_start(file: &mut File) -> bool {
    let error_condition = Regex::new(r"(.*:.*system.*\|.*sending SIGTERM to)|(^error:$)")
        .or_fail("Runtime: Invalid Regex");
    let mut contents = vec![];
    let mut text: String;
    let mut pos: usize = 0;
    let mut unchanged: usize = 0;

    for _ in 0..20 {
        contents.truncate(0);

        file.seek(SeekFrom::Start(pos as u64))
            .or_fail("Runtime: Cannot seek");
        pos += file
            .read_to_end(&mut contents)
            .or_fail("Runtime: Cannot read");
        text = String::from_utf8_lossy(&contents).to_string();

        if text.is_empty() {
            if pos > 50 {
                unchanged += 1;
            }

            if unchanged > 4 {
                break;
            }
        } else {
            unchanged = 0;

            if text.lines().any(|l| error_condition.is_match(l)) {
                return false;
            };
        }

        sleep(Duration::from_millis(500));
    }

    true
}
