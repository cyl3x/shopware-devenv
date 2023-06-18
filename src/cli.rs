use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand, ValueHint};
use clap_complete::Shell;
use clap_verbosity_flag::{Verbosity, WarnLevel};
use colored::Colorize;

use crate::utils::SPINNER;

#[derive(Debug, Clone, Parser)]
#[clap(bin_name = env!("CARGO_PKG_NAME"), name = "Shopware Devenv", version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Operation,

    /// Sets the level of verbosity
    #[command(flatten)]
    pub verbose: Verbosity<WarnLevel>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Operation {
    /// Generate a SHELL completion script and print to stdout
    #[clap(name = "comp")]
    Completions {
        /// The shell to generate completions for
        #[clap(name = "shell", action = ArgAction::Set)]
        shell: Shell,
    },

    /// Init
    #[clap(name = "init")]
    Init,

    /// Start devenv in background
    #[clap(name = "up")]
    Up,

    /// Stop devenv in background
    #[clap(name = "down")]
    Down,

    /// Show devenv logs
    #[clap(name = "log")]
    Log,

    /// Build storefront/admin/platform
    #[clap(name = "build")]
    Build {
        /// Can be storefront/admin
        #[clap(subcommand)]
        buildable: OperationBuild,
    },

    /// Check code for ci issues
    #[clap(name = "check")]
    Check {
        /// Path to check
        /// If not set, platform will be checked
        #[clap(name = "paths", value_hint = ValueHint::AnyPath)]
        paths: Option<Vec<PathBuf>>,

        #[clap(name = "no-ecs", long)]
        no_ecs: bool,

        #[clap(name = "no-phpstan", long)]
        no_phpstan: bool,
    },

    /// Watch storefront/admin/unit/jest
    #[clap(name = "watch")]
    Watch {
        /// Can be storefront/admin/unit/jest
        #[clap(subcommand)]
        watchable: OperationWatch,
    },

    /// bin/console from platform
    #[clap(name = "console")]
    Console {
        /// Arguments for bin/console
        #[clap(trailing_var_arg = true)]
        arguments: Vec<String>,
    },

    /// Install / Uninstall / Reinstall plugins
    #[clap(name = "plugin")]
    Plugin {
        /// Can be Install / Uninstall / Reinstall
        #[clap(subcommand)]
        action: OperationPlugin,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum OperationWatch {
    /// Enable hot module reloading for Storefront
    #[clap(name = "storefront", alias = "store")]
    Storefront,

    /// Hot module reloading for Administration
    #[clap(name = "admin", alias = "administration")]
    Admin,

    /// Launch the interactive jest unit test-suite watcher for Storefront
    #[clap(name = "storefront-jest", alias = "store-jest")]
    StorefrontJest,

    /// Launch the interactive jest unit test-suite watcher for Administration
    #[clap(name = "admin-jest", alias = "administration-jest")]
    AdminJest,
}

#[derive(Debug, Clone, Subcommand)]
pub enum OperationBuild {
    /// Rebuild Storefront
    #[clap(name = "storefront", alias = "store")]
    Storefront,

    /// Rebuild Administration
    #[clap(name = "admin", alias = "administration")]
    Admin,

    /// Build test db
    #[clap(name = "test-db")]
    TestDB,

    /// Build platform, use --demodata to generate with demodata
    #[clap(name = "platform")]
    Platform {
        /// Additionally fill database with demodata
        #[clap(long)]
        demodata: bool,

        #[clap(long, name = "skip-test-db")]
        skip_test_db: bool,
    },

    /// Generate demodata
    #[clap(name = "demodata")]
    Demodata {
        /// Arguments for demo data generator
        #[clap(trailing_var_arg = true)]
        arguments: Vec<String>,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum OperationPlugin {
    /// Install and activate a plugin
    #[clap(name = "install", alias = "i")]
    Install {
        /// Fuzzy matched plugin name
        #[clap(name = "plugin-name")]
        name: String,

        /// Do not activate plugin after installation
        #[clap(name = "no-activation", long, short)]
        no_activation: bool,
    },

    /// Uninstall a plugin
    #[clap(name = "uninstall", alias = "u", alias = "remove")]
    Uninstall {
        /// Fuzzy matched plugin name
        #[clap(name = "plugin-name")]
        name: String,
    },

    /// Reinstall a plugin
    #[clap(name = "reinstall", alias = "r")]
    Reinstall {
        /// Fuzzy matched plugin name
        #[clap(name = "plugin-name")]
        name: String,
    },

    /// Refresh plugins
    #[clap(name = "refresh")]
    Refresh,

    /// List plugins
    #[clap(name = "list")]
    List,
}

pub enum ExitCode {
    RunAsRoot = 1,
    InvalidArgs = 2,
    AppDirsCreation = 3,

    Runtime = 9,

    // Devenv
    DevenvStart = 10,
    DevenvStop = 11,
    DevenvOnce = 12,
    DevenvExec = 13,

    // Config
    ConfigRead = 20,
    ConfigWrite = 21,
    ConfigBak = 22,

    // Context
    InvalidContext = 30,
}

#[macro_export]
macro_rules! fail {
    ($exit_code:expr, $($arg:tt)+) => {
        $crate::cli::_macro_fail(&format!($($arg)+), $exit_code)
    };
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)+) => {
        $crate::cli::_macro_success(&format!($($arg)+))
    }
}

pub fn _macro_success(msg: &str) -> ! {
    let message = format!("{} {}", "✓".green(), msg.bold());

    if unsafe { SPINNER.get().is_some() } {
        crate::spinner_stop!("\r{message}");
    } else {
        println!("\r{message}");
    }

    std::process::exit(0);
}

pub fn _macro_fail(msg: &str, exit_code: ExitCode) -> ! {
    let message = format!("{} {}", "✕".red(), msg.bold());

    if unsafe { SPINNER.get().is_some() } {
        crate::spinner_stop!("\r{message}");
    } else {
        println!("\r{message}");
    }

    std::process::exit(exit_code as i32);
}
