use std::env::vars_os;
use std::path::PathBuf;
use std::process::{Child, Command};

use crate::cli::ExitCode;
use crate::context::Context;
use crate::fail;

pub trait AppCommand {
    fn start(&mut self) -> Child;
    fn start_await_success(&mut self);
    fn log(&self);
}

pub trait Direnv {
    fn new(path: Option<PathBuf>, cmd: Vec<&str>) -> Self;
}

pub trait Devenv {
    fn new(cmd: Vec<&str>) -> Self;
}

impl Direnv for Command {
    fn new(path: Option<PathBuf>, cmd: Vec<&str>) -> Self {
        let path = path
            .unwrap_or_else(|| Context::get().platform.path.clone())
            .display()
            .to_string();

        let mut command: Self = Self::new("direnv");
        command
            .envs(&mut vars_os())
            .env("DIRENV_LOG_FORMAT", "")
            .args(["exec", &path])
            .args(cmd);

        command
    }
}

impl Devenv for Command {
    fn new(cmd: Vec<&str>) -> Self {
        Context::get().platform.move_to();

        let mut command: Self = Self::new("devenv");
        command.envs(&mut vars_os()).args(cmd);

        command
    }
}

impl AppCommand for Command {
    /// Executes the command as a child process, returning a handle to it.
    /// Terminates app on error (e.g. direnv not found)
    fn start(&mut self) -> Child {
        self.log();

        self.spawn().unwrap_or_else(|_| {
            fail!(
                ExitCode::DevenvStart,
                "Cannot spawn cmd, is devenv/direnv ok?"
            )
        })
    }

    /// Executes the command as a child process and wait for exit.
    /// Terminates app on error (e.g. non-zero exit code)
    fn start_await_success(&mut self) {
        if let Err(error) = self.start().wait() {
            fail!(ExitCode::DevenvExec, "Non zero exit: {error}");
        }
    }

    /// Logs the command
    fn log(&self) {
        let args = self
            .get_args()
            .filter_map(|a| a.to_str().map(std::borrow::ToOwned::to_owned))
            .collect::<Vec<String>>();

        log::debug!(
            "[{}] {}",
            self.get_program().to_str().unwrap_or("command"),
            args.join(" "),
        );
    }
}

/// Creates a new direnv command.
/// Use `start` or `start_await_success` to execute the command.
/// 
/// Use to execute a command in the devenv environment.
#[macro_export]
macro_rules! direnv {
    (path = $path:expr, $($cmd:expr),+ $(,)?) => {
        <std::process::Command as $crate::devenv::Direnv>::new($path, vec![$($cmd),+])
    };
    ($($cmd:expr),+ $(,)?) => {
        <std::process::Command as $crate::devenv::Direnv>::new(None, vec![$($cmd),+])
    };
}

/// Creates a new devenv command.
/// Use `start` or `start_await_success` to execute the command.
/// 
/// Don't use to execute something in devenv shell/environment.
#[macro_export]
macro_rules! devenv {
    ($($cmd:expr),+ $(,)?) => {
        <std::process::Command as $crate::devenv::Devenv>::new(vec![$($cmd),+])
    };
}
