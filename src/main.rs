#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used
)]
mod cli;
mod constants;
mod context;
mod devenv;
mod logger;
mod operations;
mod utils;

use std::io;

use clap::{CommandFactory, Parser};
use nix::unistd::Uid;

pub use crate::cli::ExitCode;
use crate::cli::{Cli, Operation, OperationBuild, OperationPlugin, OperationWatch};
pub use crate::constants::*;
pub use crate::context::Context;
pub use crate::devenv::AppCommand;
use crate::operations::{build, check, console, down, init, plugin, up, watch};

extern crate log;

fn main() {
    if Uid::effective().is_root() {
        fail!(ExitCode::RunAsRoot, "Running swde as root is not allowed");
    }

    let cli: Cli = Cli::parse();
    logger::init(cli.verbose.log_level_filter());

    match cli.subcommand {
        Operation::Up => up::main(),
        Operation::Down => down::main(),
        Operation::Init => init::main(),
        Operation::Watch { watchable } => match watchable {
            OperationWatch::Admin => watch::admin(),
            OperationWatch::Storefront => watch::storefront(),
            OperationWatch::StorefrontJest => watch::storefront_jest(),
            OperationWatch::AdminJest => watch::admin_jest(),
        },
        Operation::Build { buildable } => match buildable {
            OperationBuild::Storefront => build::storefront(),
            OperationBuild::Admin => build::admin(),
            OperationBuild::Platform {
                demodata,
                skip_test_db,
            } => build::platform(demodata, !skip_test_db),
            OperationBuild::TestDB => build::test_db(),
            OperationBuild::Demodata { arguments } => build::demodata(&arguments),
        },
        Operation::Check {
            paths,
            no_ecs,
            no_phpstan,
        } => check::main(paths, no_ecs, no_phpstan),
        Operation::Console { arguments } => console::main(&arguments),
        Operation::Log => operations::log::main(),
        Operation::Completions { shell } => {
            clap_complete::generate(
                shell,
                &mut Cli::command(),
                env!("CARGO_PKG_NAME"),
                &mut io::stdout(),
            );
        },
        Operation::Plugin { action } => match action {
            OperationPlugin::Install {
                name,
                no_activation,
            } => plugin::install(&name, no_activation),
            OperationPlugin::Uninstall { name } => plugin::uninstall(&name),
            OperationPlugin::Reinstall { name } => plugin::reinstall(&name),
            OperationPlugin::Refresh => plugin::refresh(),
            OperationPlugin::List => plugin::list(),
        },
    }
}
