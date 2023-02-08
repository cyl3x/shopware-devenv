use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[clap(name = "swde", version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Operation,

    /// Sets the level of verbosity
    #[clap(long, short, global(true), action = ArgAction::SetTrue)]
    pub verbose: bool,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Operation {
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

    /// Build storefront/admin
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
        #[clap(name = "path")]
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
}

#[derive(Debug, Clone, Subcommand)]
pub enum OperationWatch {
    /// Enable hot module reloading for Storefront
    #[clap(name = "storefront")]
    Storefront,

    /// Hot module reloading for Administration
    #[clap(name = "admin")]
    Admin,

    /// Launch the interactive jest unit test-suite watcher for Storefront
    #[clap(name = "storefront-jest")]
    StorefrontJest,

    /// Launch the interactive jest unit test-suite watcher for Administration
    #[clap(name = "admin-jest")]
    AdminJest,
}

#[derive(Debug, Clone, Subcommand)]
pub enum OperationBuild {
    // Rebuild Storefront
    #[clap(name = "storefront")]
    Storefront,

    // Rebuild Administration
    #[clap(name = "admin")]
    Admin,

    // Build test db
    #[clap(name = "test-db")]
    TestDB,

    #[clap(name = "platform")]
    Platform {
        /// Additionally fill database with demodata
        #[clap(long)]
        demodata: bool,

        #[clap(long, name = "skip-test-db")]
        skip_test_db: bool,
    },
}
