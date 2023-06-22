#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used
)]
mod app;
mod cli;
mod constants;
mod context;
mod operations;
mod utils;

use std::env;
use std::io;

use clap::{CommandFactory, Parser};

use crate::cli::{Cli, Operation, OperationBuild, OperationPlugin, OperationWatch};
pub use crate::constants::*;
pub use crate::context::Context;
pub use crate::app::Command;
use crate::operations::{
    build, config, console, down, dump_server, plugin, up, update, watch,
};
pub use crate::utils::OrFail;

fn main() {
    if utils::uid() == 0 {
        fail!("Running swde as root is not allowed");
    }

    if let Some(verbose) = env::var("SWDE_VERBOSE").ok().and_then(|v| v.parse::<bool>().ok()) {
        VERBOSE.set(verbose).ok(); // Result only indicates if the value was set
    }

    let cli: Cli = Cli::parse();
    VERBOSE.get_or_init(|| cli.verbose);

    match cli.subcommand {
        Operation::Up => up::main(),
        Operation::Down => down::main(),
        Operation::Config => config::main(),
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
            OperationPlugin::Activate { name } => plugin::activate(&name),
            OperationPlugin::Uninstall { name } => plugin::uninstall(&name),
            OperationPlugin::Reinstall { name } => plugin::reinstall(&name),
            OperationPlugin::Refresh => plugin::refresh(),
            OperationPlugin::List => plugin::list(),
        },
        Operation::Update => update::main(),
        Operation::DumpServer { port } => dump_server::main(port),
    }
}
