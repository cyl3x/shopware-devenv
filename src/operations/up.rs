use std::env::vars_os;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use regex::Regex;
use spinoff::{spinners, Color, Spinner};
use sysinfo::{Pid, SystemExt};

use crate::internal::AppExitCode;
use crate::operations::{DEVENV_LOG, DEVENV_PID};
use crate::{crash, finish};

pub fn main() {
    if check_running_instances() {
        crash!(
            AppExitCode::DevenvStart,
            "Devenv service is already running"
        );
    }

    let spinner = Spinner::new(spinners::Dots, "Starting...", Color::Blue);

    let mut log = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .truncate(true)
        .open(DEVENV_LOG)
        .expect("Failed to create out log");

    let mut child = Command::new("devenv")
        .arg("up")
        .stdout(log.try_clone().expect("Cannot log into the same file?"))
        .stderr(log.try_clone().expect("Cannot log into the same file?"))
        .envs(&mut vars_os())
        .spawn()
        .expect("Failed to start devenv");

    let success = check_successfull_start(&mut log);
    spinner.clear();

    if success {
        finish!("Devenv service started");
    }

    let _r = child.wait();

    super::log::main();

    crash!(AppExitCode::DevenvStart, "Error while starting devenv.");
}

fn check_running_instances() -> bool {
    if let Ok(pid_string) = fs::read_to_string(DEVENV_PID.clone()) {
        let pid: usize = pid_string
            .lines()
            .next()
            .expect("Malformed pidfile")
            .parse::<usize>()
            .expect("Malformed pid in pidfile");

        let mut sys = sysinfo::System::new();
        sys.refresh_processes();

        return sys.process(Pid::from(pid)).is_some();
    }

    false
}

fn check_successfull_start(file: &mut File) -> bool {
    let error_condition =
        Regex::new(r"(.*:.*system.*\|.*sending SIGTERM to)|(^error:$)").expect("Invalid Regex");
    let mut contents = vec![];
    let mut text: String;
    let mut pos: usize = 0;
    let mut unchanged: usize = 0;

    for _ in 0..20 {
        contents.truncate(0);

        file.seek(SeekFrom::Start(pos as u64)).expect("Cannot seek");
        pos += file.read_to_end(&mut contents).expect("Cannot read");
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
