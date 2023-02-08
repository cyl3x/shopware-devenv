#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
mod args;
mod context;
mod internal;
mod operations;

use clap::Parser;
use nix::unistd::Uid;
use std::env;

use crate::args::{Args, Operation};
use crate::args::{OperationBuild, OperationWatch};
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

    let context = Context::new(env::current_dir().unwrap());
    context.platform.move_to();

    let args: Args = Args::parse();

    let verbose = args.verbose;
    log!(verbose, "{context:#?}");

    match args.subcommand {
        Operation::Up => up::main(verbose),
        Operation::Down => down::main(verbose),
        Operation::Init => init::main(verbose),
        Operation::Watch { watchable } => match watchable {
            OperationWatch::Admin => watch::admin(verbose),
            OperationWatch::Storefront => watch::storefront(verbose),
            OperationWatch::StorefrontJest => watch::storefront_jest(verbose),
            OperationWatch::AdminJest => watch::admin_jest(verbose),
        },
        Operation::Build { buildable } => match buildable {
            OperationBuild::Storefront => build::storefront(verbose),
            OperationBuild::Admin => build::admin(verbose),
            OperationBuild::Platform {
                demodata,
                skip_test_db,
            } => build::platform(verbose, demodata, !skip_test_db),
            OperationBuild::TestDB => build::test_db(verbose),
        },
        Operation::Check {
            paths,
            no_ecs,
            no_phpstan,
        } => check::main(verbose, &context, paths, no_ecs, no_phpstan),
        Operation::Log => log::main(verbose),
    }
}
