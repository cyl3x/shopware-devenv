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

use std::{env, io, str::FromStr};

use clap::{CommandFactory, Parser};
use clap_complete::Shell;

pub use crate::app::Command;
use crate::cli::{Cli, Operation, OperationBuild, OperationPlugin, OperationWatch};
pub use crate::constants::*;
pub use crate::context::Context;
use crate::operations::{build, config, console, down, dump_server, plugin, up, update, watch};
pub use crate::utils::OrFail;

fn main() {
    color_eyre::install().or_panic("Failed to init color_eyre".into());

    if utils::uid() == 0 {
        fail!("Running swde as root is not allowed");
    }

    env::var("SWDE_VERBOSE")
        .ok()
        .and_then(|v| {
            v.parse::<bool>().ok().or_else(|| {
                warn!("SWDE_VERBOSE => {v} doesn't match one of [false, true, 0, 1]");
                None
            })
        })
        .and_then(|v| VERBOSE.set(v).ok());

    let cli: Cli = Cli::parse();
    VERBOSE.get_or_init(|| cli.verbose);

    let result: Result<String, anyhow::Error> = match cli.subcommand {
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
        Operation::Completions { mut shell } => {
            if shell.is_none() {
                shell = env::var("SHELL")
                    .ok()
                    .and_then(|s| s.split('/').last().map(str::to_string))
                    .and_then(|s| Shell::from_str(&s).ok());
            }

            clap_complete::generate(
                shell.or_panic("Could not determine shell".into()),
                &mut Cli::command(),
                env!("CARGO_PKG_NAME"),
                &mut io::stdout(),
            );

            Ok(String::new())
        },
        Operation::Plugin { action } => match action {
            OperationPlugin::Refresh => plugin::refresh(),
            OperationPlugin::Install {
                name,
                no_activation,
            } => plugin::install(&name, no_activation),
            OperationPlugin::Activate { name } => plugin::activate(&name),
            OperationPlugin::Uninstall { name } => plugin::uninstall(&name),
            OperationPlugin::Reinstall { name } => plugin::reinstall(&name),
            OperationPlugin::List => plugin::list(),
        },
        Operation::Update => update::main(),
        Operation::DumpServer { port } => dump_server::main(port),
    };

    result.map_or_else(|error| fail!("{error}"), |msg| success!("{msg}"));
}
