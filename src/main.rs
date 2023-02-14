#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used
)]
mod args;
mod config;
mod context;
mod internal;
mod operations;

use std::env;

use clap::Parser;
use nix::unistd::Uid;

use crate::args::{Args, Operation, OperationBuild, OperationWatch};
use crate::config::Config;
use crate::context::Context;
use crate::internal::AppExitCode;
use crate::operations::{build, check, down, init, log, up, watch};

fn main() {
    if Uid::effective().is_root() {
        crash!(
            AppExitCode::RunAsRoot,
            "Running swde as root is not allowed"
        );
    }

    let args: Args = Args::parse();

    let mut config = Config::new();

    if !config.verbose {
        config.verbose = args.verbose;
    }

    let current_dir = env::current_dir().expect("Insufficient permissions or invalid current path");
    let Some(context) = Context::new(&config, &current_dir) else {
        crash!(
            AppExitCode::InvalidContext,
            "Current directory has not a valid swde context"
        );
    };
    context.platform.move_to();

    match args.subcommand {
        Operation::Up => up::main(&config),
        Operation::Down => down::main(&config),
        Operation::Init => init::main(),
        Operation::Watch { watchable } => match watchable {
            OperationWatch::Admin => watch::admin(&config),
            OperationWatch::Storefront => watch::storefront(&config),
            OperationWatch::StorefrontJest => watch::storefront_jest(&config),
            OperationWatch::AdminJest => watch::admin_jest(&config),
        },
        Operation::Build { buildable } => match buildable {
            OperationBuild::Storefront => build::storefront(&config),
            OperationBuild::Admin => build::admin(&config),
            OperationBuild::Platform {
                demodata,
                skip_test_db,
            } => build::platform(&config, demodata, !skip_test_db),
            OperationBuild::TestDB => build::test_db(&config),
        },
        Operation::Check {
            paths,
            no_ecs,
            no_phpstan,
        } => check::main(&config, &context, paths, no_ecs, no_phpstan),
        Operation::Log => log::main(),
    }
}
