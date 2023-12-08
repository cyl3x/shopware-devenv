use clap::{ArgAction, Parser, Subcommand, ValueHint};
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
    #[clap(name = "completion", visible_alias = "comp")]
    Completions {
        /// The shell to generate completions for. Inferred from $SHELL if not provided
        #[clap(name = "shell", action = ArgAction::Set)]
        shell: Option<Shell>,
    },

    /// Update to the included devenv.local.nix
    #[clap(name = "config")]
    Config,

    /// Start devenv in background
    #[clap(name = "up", visible_alias = "u")]
    Up,

    /// Stop devenv in background
    #[clap(name = "down", visible_alias = "d")]
    Down,

    /// Show devenv logs
    #[clap(name = "log", visible_alias = "l")]
    Log,

    /// Build storefront/admin/platform
    #[clap(name = "build", visible_alias = "b")]
    Build {
        /// Can be storefront/admin
        #[clap(subcommand)]
        buildable: OperationBuild,
    },

    /// Watch storefront/admin/unit/jest
    #[clap(name = "watch", visible_alias = "w")]
    Watch {
        /// Can be storefront/admin/unit/jest
        #[clap(subcommand)]
        watchable: OperationWatch,
    },

    /// bin/console from platform
    #[clap(name = "console", visible_alias = "c")]
    Console {
        /// Arguments for bin/console
        #[clap(trailing_var_arg = true, value_hint = ValueHint::Other)]
        arguments: Vec<String>,
    },

    /// Install/Activate/Uninstall/Reinstall plugins
    #[clap(name = "plugin", visible_alias = "p")]
    Plugin {
        /// Can be Install/Activate/Uninstall/Reinstall
        #[clap(subcommand)]
        action: OperationPlugin,
    },

    /// Update platform and garbage collect devenv
    #[clap(name = "update")]
    Update,

    /// Start the symfony dump server
    #[clap(name = "dump-server", visible_alias = "ds")]
    DumpServer {
        /// Server port (1024-65535)
        #[clap(name = "port", long, short, value_parser=port_check, value_hint = ValueHint::Other)]
        port: Option<u16>,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum OperationWatch {
    /// Enable hot module reloading for Storefront
    #[clap(name = "store", alias = "storefront", visible_alias = "s")]
    Storefront,

    /// Hot module reloading for Administration
    #[clap(name = "admin", alias = "administration", visible_alias = "a")]
    Admin,

    /// Launch the interactive jest unit test-suite watcher for Storefront
    #[clap(name = "store-jest", alias = "storefront-jest", visible_alias = "sj")]
    StorefrontJest,

    /// Launch the interactive jest unit test-suite watcher for Administration
    #[clap(
        name = "admin-jest",
        alias = "administration-jest",
        visible_alias = "aj"
    )]
    AdminJest,
}

#[derive(Debug, Clone, Subcommand)]
pub enum OperationBuild {
    /// Rebuild Storefront
    #[clap(name = "store", alias = "storefront", visible_alias = "s")]
    Storefront,

    /// Rebuild Administration
    #[clap(name = "admin", alias = "administration", visible_alias = "a")]
    Admin,

    /// Build test db
    #[clap(name = "test-db", visible_alias = "tdb")]
    TestDB,

    /// Build platform, use --demodata to generate with demodata
    #[clap(name = "platform", visible_alias = "p")]
    Platform {
        /// Additionally fill database with demodata
        #[clap(long)]
        demodata: bool,

        #[clap(long, name = "skip-test-db")]
        skip_test_db: bool,
    },

    /// Generate demodata
    #[clap(name = "demodata", visible_alias = "dd")]
    Demodata {
        /// Arguments for demo data generator
        #[clap(trailing_var_arg = true)]
        arguments: Vec<String>,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum OperationPlugin {
    /// Install and activate a plugin
    #[clap(name = "install", visible_alias = "i")]
    Install {
        /// Fuzzy matched plugin name
        #[clap(name = "plugin-name", value_hint = ValueHint::Other)]
        name: String,

        /// Do not activate plugin after installation
        #[clap(name = "no-activation", long, short)]
        no_activation: bool,
    },

    /// Activate an installed plugin
    #[clap(name = "activate", visible_alias = "a")]
    Activate {
        /// Fuzzy matched plugin name
        #[clap(name = "plugin-name", value_hint = ValueHint::Other)]
        name: String,
    },

    /// Uninstall a plugin
    #[clap(name = "uninstall", visible_alias = "u", alias = "remove")]
    Uninstall {
        /// Fuzzy matched plugin name
        #[clap(name = "plugin-name", value_hint = ValueHint::Other)]
        name: String,
    },

    /// Reinstall a plugin
    #[clap(name = "reinstall", visible_alias = "r")]
    Reinstall {
        /// Fuzzy matched plugin name
        #[clap(name = "plugin-name", value_hint = ValueHint::Other)]
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
