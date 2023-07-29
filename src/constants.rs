use once_cell::sync::OnceCell;

pub static VERBOSE: OnceCell<bool> = OnceCell::new();

pub static DEVENV_DEFAULT_CONFIG: &str = include_str!("../devenv.local.nix");
