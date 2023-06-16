#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used
)]
mod args;
mod context;
mod internal;
mod operations;

use std::io;

use clap::{CommandFactory, Parser};
use internal::Logger;
use nix::unistd::Uid;

use crate::args::{Args, Operation, OperationBuild, OperationPlugin, OperationWatch};
use crate::internal::ExitCode;
use crate::operations::{build, check, console, down, init, log, plugin, up, watch};

fn main() {
    if Uid::effective().is_root() {
        fail!(ExitCode::RunAsRoot, "Running swde as root is not allowed");
    }

    let args: Args = Args::parse();
    Logger::init(args.verbose);

    match args.subcommand {
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
        Operation::Log => log::main(),
        Operation::Completions { shell } => {
            clap_complete::generate(shell, &mut Args::command(), "swde", &mut io::stdout());
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
