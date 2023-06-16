use std::env::vars_os;
use std::path::PathBuf;
use std::process::{Child, Command};

use colored::Colorize;

use super::app::ExitCode;
use crate::context::Context;
use crate::{fail, log_verbose};

pub trait AppCommand {
    fn start(&mut self) -> Child;
    fn start_await_success(&mut self);
    fn log_verbose(&self);
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
        command.args(["exec", &path]).args(cmd).envs(&mut vars_os());

        command
    }
}

impl Devenv for Command {
    fn new(cmd: Vec<&str>) -> Self {
        log_verbose!("[{}] {}", "devenv".green(), cmd.join(" "));

        Context::get().platform.move_to();

        let mut command: Self = Self::new("devenv");
        command.args(cmd).envs(&mut vars_os());

        command
    }
}

impl AppCommand for Command {
    /// Executes the command as a child process, returning a handle to it.
    /// Terminates app on error (e.g. direnv not found)
    fn start(&mut self) -> Child {
        self.log_verbose();

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

    fn log_verbose(&self) {
        let args = self
            .get_args()
            .filter_map(|a| a.to_str().map(std::borrow::ToOwned::to_owned))
            .collect::<Vec<String>>();

        log_verbose!(
            "[{}] {}",
            self.get_program().to_str().unwrap_or("").green(),
            args.join(" ")
        );
    }
}

#[macro_export]
macro_rules! direnv {
    (path = $path:expr, $($cmd:expr),+ $(,)?) => {
        <std::process::Command as $crate::internal::Direnv>::new($path, vec![$($cmd),+])
    };
    ($($cmd:expr),+ $(,)?) => {
        <std::process::Command as $crate::internal::Direnv>::new(None, vec![$($cmd),+])
    };
}

#[macro_export]
macro_rules! devenv {
    ($($cmd:expr),+ $(,)?) => {
        <std::process::Command as $crate::internal::Devenv>::new(vec![$($cmd),+])
    };
}
