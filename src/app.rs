use std::env::vars_os;
use std::process;
use std::process::{exit, Child, ExitStatus, Output};

use colored::Colorize;

use crate::{verbose, Context, OrFail, VERBOSE};

#[macro_export]
macro_rules! fail {
    ($($arg:tt)+) => {
        $crate::app::macro_fail(&format!($($arg)+))
    };
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)+) => {
        $crate::app::macro_success(&format!($($arg)+))
    }
}

#[macro_export]
macro_rules! topic {
    ($($arg:tt)+) => {
        $crate::app::macro_topic(&format!($($arg)+))
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {
        $crate::app::macro_warn(&format!($($arg)+))
    }
}

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)+) => {
        $crate::app::macro_verbose(&format!($($arg)+))
    }
}

pub fn macro_success(msg: &str) -> ! {
    if !msg.is_empty() {
        println!("{} {}", "✓".green(), msg.bold());
    }

    exit(0);
}

pub fn macro_fail(msg: &str) -> ! {
    println!("{} {}", "✕".red(), msg.bold());

    exit(1);
}

pub fn macro_topic(msg: &str) {
    println!("{} {}", ">".cyan(), msg.bold());
}

pub fn macro_warn(msg: &str) {
    println!("{} {}", "!".yellow(), msg.yellow());
}

pub fn macro_verbose(msg: &str) {
    if *VERBOSE.get().unwrap_or(&false) {
        println!("[{}] {msg}", "verbose".red());
    }
}

pub trait Command {
    /// Creates a new command for direnv.
    ///
    /// Be aware that this command will be executed in the platform directory
    ///
    /// # Errors
    /// Returns the context error if the context is invalid
    fn new_direnv(cmd: Vec<&str>) -> anyhow::Result<Self>
    where
        Self: std::marker::Sized;

    /// Creates a new command for devenv.
    ///
    /// Be aware that this command will be executed in the platform directory
    ///
    /// # Errors
    /// Returns the context error if the context is invalid
    fn new_devenv(cmd: Vec<&str>) -> anyhow::Result<Self>
    where
        Self: std::marker::Sized;

    /// Executes the command as a child process, returning a handle to it.
    ///
    /// Terminates app on error (e.g. direnv not found)
    fn start(&mut self) -> Child;

    /// Executes the command as a child process and await the exit.
    fn r#await(&mut self) -> ExitStatus;

    /// Executes the command as a child process, waiting for it to finish and
    /// collecting all of its output.
    ///
    /// Terminates app on error (e.g. non-zero exit code)
    fn await_output(&mut self) -> Output;

    /// Executes the command as a child process and wait for exit.
    ///
    /// Terminates app on error (e.g. non-zero exit code)
    ///
    /// # Errors
    /// Fails if the exit code is non-zero
    fn await_success(&mut self) -> anyhow::Result<()>;

    /// Logs the command
    fn log(&self);

    /// Converts the command into a string.
    /// Strips out `direnv exec .`...
    fn to_string(&self) -> String;
}

impl Command for process::Command {
    fn new_direnv(cmd: Vec<&str>) -> anyhow::Result<Self> {
        let mut command: Self = Self::new("direnv");
        command
            .envs(vars_os())
            .current_dir(Context::get()?.platform.path.clone())
            .env("DIRENV_LOG_FORMAT", "")
            .args(["exec", &Context::get()?.platform.join_str("")])
            .args(cmd);

        Ok(command)
    }

    fn new_devenv(cmd: Vec<&str>) -> anyhow::Result<Self> {
        let mut command: Self = Self::new("devenv");
        command
            .envs(vars_os())
            .current_dir(Context::get()?.platform.path.clone())
            .args(cmd);

        Ok(command)
    }

    fn start(&mut self) -> Child {
        self.log();

        self.spawn().or_panic("Cannot spawn process".into())
    }

    fn r#await(&mut self) -> ExitStatus {
        self.start()
            .wait()
            .or_panic("Cannot wait for process to finish".into())
    }

    fn await_output(&mut self) -> Output {
        self.output().or_panic("Cannot spawn process".into())
    }

    fn await_success(&mut self) -> anyhow::Result<()> {
        if !self.r#await().success() {
            anyhow::bail!("Non zero exit while running '{}'", self.to_string());
        }

        Ok(())
    }

    fn log(&self) {
        let args = self
            .get_args()
            .filter_map(|a| a.to_str().map(std::borrow::ToOwned::to_owned))
            .collect::<Vec<String>>();

        verbose!(
            "[{}] {}",
            self.get_program().to_str().unwrap_or("command"),
            args.join(" "),
        );
    }

    fn to_string(&self) -> String {
        let mut command = vec![];
        if self.get_program().to_str() == Some("direnv") {
            let s = self.get_args().skip(2).filter_map(std::ffi::OsStr::to_str);
            command.extend(s);
        } else {
            command.push(self.get_program().to_str().unwrap_or_default());
            command.extend(self.get_args().filter_map(std::ffi::OsStr::to_str));
        }

        command.join(" ")
    }
}

/// Creates a new direnv command.
/// Use `start`, `await` or `await_success` to execute the command.
#[macro_export]
macro_rules! direnv {
    ($($cmd:expr),+ $(,)?) => {
        <std::process::Command as $crate::app::Command>::new_direnv(vec![$($cmd),+])?
    };
}

/// Creates a new devenv command.
/// Use `start`, `await` or `await_success` to execute the command.
///
/// Don't use to execute something with devenv shell/environment.
#[macro_export]
macro_rules! devenv {
    ($($cmd:expr),+ $(,)?) => {
        <std::process::Command as $crate::app::Command>::new_devenv(vec![$($cmd),+])?
    };
}

/// Creates a new git within direnv command.
/// Use `start`, `await` or `await_success` to execute the command.
#[macro_export]
macro_rules! direnv_git {
    ($($cmd:expr),+ $(,)?) => {
        $crate::app::macro_direnv_git(vec![$($cmd),+])?
    };
}

/// Creates a new git command inside direnv env.
/// Use `start`, `await` or `await_success` to execute the command.
pub fn macro_direnv_git(cmd: Vec<&str>) -> anyhow::Result<impl Command> {
    let mut command = direnv!["git"];
    command.args(cmd).env("GIT_TERMINAL_PROMPT", "0");

    Ok(command)
}
