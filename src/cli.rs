use clap::{ArgAction, Parser, Subcommand};
use clap_complete::Shell;

#[derive(Debug, Clone, Parser)]
#[clap(bin_name = env!("CARGO_PKG_NAME"), name = "Shopware Devenv", version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Operation,

    /// Set the verbosity. Can also be set with SWDE_VERBOSE=1
    #[clap(long, short, global(true), action = ArgAction::SetTrue)]
    pub verbose: bool,
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

    /// Update to the included devenv.local.nix
    #[clap(name = "config")]
    Config,

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

    /// Install/Activate/Uninstall/Reinstall plugins
    #[clap(name = "plugin")]
    Plugin {
        /// Can be Install/Activate/Uninstall/Reinstall
        #[clap(subcommand)]
        action: OperationPlugin,
    },

    /// Update platform and garbage collect devenv
    #[clap(name = "update")]
    Update,

    /// Start the symfony dump server
    #[clap(name = "dump-server")]
    DumpServer {
        /// Server port (1024-65535)
        #[clap(name = "port", long, short, value_parser=port_check)]
        port: Option<u16>,
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

    /// Activate an installed plugin
    #[clap(name = "activate", alias = "a")]
    Activate {
        /// Fuzzy matched plugin name
        #[clap(name = "plugin-name")]
        name: String,
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

fn port_check(s: &str) -> Result<u16, String> {
    let port = s
        .parse::<u16>()
        .map_err(|_| "Port must be a number between 1024 and 65535")?;

    if !(1024..=65535).contains(&port) {
        return Err("Port must be a number between 1024 and 65535".to_string());
    }

    Ok(port)
}
