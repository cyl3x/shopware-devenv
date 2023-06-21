

use colored::Colorize;

use std::process::exit;
use std::env::vars_os;
use std::process;
use std::process::{Child, ExitStatus, Output};

use crate::{fail, verbose, OrFail, Context};

use crate::VERBOSE;

#[macro_export]
macro_rules! fail {
    ($($arg:tt)+) => {
        $crate::app::_macro_fail(&format!($($arg)+))
    };
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)+) => {
        $crate::app::_macro_success(&format!($($arg)+))
    }
}

#[macro_export]
macro_rules! topic {
    ($($arg:tt)+) => {
        $crate::app::_macro_topic(&format!($($arg)+))
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {
        $crate::app::_macro_warn(&format!($($arg)+))
    }
}

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)+) => {
        $crate::app::_macro_verbose(&format!($($arg)+))
    }
}

pub fn _macro_success(msg: &str) -> ! {
    println!("{} {}", "✓".green(), msg.bold());

    exit(0);
}

pub fn _macro_fail(msg: &str) -> ! {
    println!("{} {}", "✕".red(), msg.bold());

    exit(1);
}

pub fn _macro_topic(msg: &str) {
    println!("{} {}", ">".cyan(), msg.bold());
}

pub fn _macro_warn(msg: &str) {
    println!("{} {}", "!".yellow(), msg.bold());
}

pub fn _macro_verbose(msg: &str) {
    if *VERBOSE.get().unwrap_or(&false) {
        println!("[{}] {msg}", "verbose".red());
    }
}

pub trait Command {
    fn new_direnv(cmd: Vec<&str>) -> Self;
    fn new_devenv(cmd: Vec<&str>) -> Self;
    fn start(&mut self) -> Child;
    fn r#await(&mut self) -> ExitStatus;
    fn await_output(&mut self) -> Output;
    fn await_success(&mut self);
    fn log(&self);
    fn to_string(&self) -> String;
}

impl Command for process::Command {
    /// Creates a new command for direnv.
    ///
    /// Be aware that this command will be executed in the current directory
    fn new_direnv(cmd: Vec<&str>) -> Self {
        let mut command: Self = Self::new("direnv");
        command
            .envs(&mut vars_os())
            .env("DIRENV_LOG_FORMAT", "")
            .args(["exec", &Context::get().platform.join_str("")])
            .args(cmd);

        command
    }

    /// Creates a new command for devenv.
    ///
    /// Be aware that this command will be executed in the platform directory
    fn new_devenv(cmd: Vec<&str>) -> Self {
        let mut command: Self = Self::new("devenv");
        command
            .envs(&mut vars_os())
            .current_dir(Context::get().platform.path.clone())
            .args(cmd);

        command
    }

    /// Executes the command as a child process, returning a handle to it.
    ///
    /// Terminates app on error (e.g. direnv not found)
    fn start(&mut self) -> Child {
        self.log();

        self.spawn()
            .or_fail("Runtime: Cannot spawn cmd, is devenv/direnv ok?")
    }

    /// Executes the command as a child process and await the exit.
    fn r#await(&mut self) -> ExitStatus {
        self.start().wait().or_fail("Runtime: cannot wait for cmd")
    }

    /// Executes the command as a child process, waiting for it to finish and
    /// collecting all of its output.
    ///
    /// Terminates app on error (e.g. non-zero exit code)
    fn await_output(&mut self) -> Output {
        self.output()
            .or_fail("Runtime: Cannot spawn cmd, is devenv/direnv ok?")
    }

    /// Executes the command as a child process and wait for exit.
    ///
    /// Terminates app on error (e.g. non-zero exit code)
    fn await_success(&mut self) {
        if !self.r#await().success() {
            fail!("Non zero exit while running '{}'", self.to_string());
        }
    }

    /// Logs the command
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

    /// Converts the command into a string.
    /// Strips out `direnv exec .`...
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
        <std::process::Command as $crate::app::Command>::new_direnv(vec![$($cmd),+])
    };
}

/// Creates a new devenv command.
/// Use `start`, `await` or `await_success` to execute the command.
///
/// Don't use to execute something with devenv shell/environment.
#[macro_export]
macro_rules! devenv {
    ($($cmd:expr),+ $(,)?) => {
        <std::process::Command as $crate::app::Command>::new_devenv(vec![$($cmd),+])
    };
}

/// Creates a new git within direnv command.
/// Use `start`, `await` or `await_success` to execute the command.
#[macro_export]
macro_rules! direnv_git {
    ($($cmd:expr),+ $(,)?) => {
        $crate::app::_macro_direnv_git(vec![$($cmd),+])
    };
}


/// Creates a new git command inside direnv env.
/// Use `start`, `await` or `await_success` to execute the command.
pub fn _macro_direnv_git(cmd: Vec<&str>) -> impl Command {
    let mut command = direnv!("git");
    command.args(cmd).env("GIT_TERMINAL_PROMPT", "0");

    command
}
