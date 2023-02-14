pub mod build;
pub mod check;
pub mod down;
pub mod init;
pub mod log;
pub mod up;
pub mod watch;

pub static DEVENV_CONFIG: &str = include_str!("../../devenv.local.nix");
pub static DEVENV_LOG: &str = "/tmp/devenv.log";
