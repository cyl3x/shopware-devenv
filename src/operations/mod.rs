use once_cell::sync::Lazy;

use crate::context::Context;

pub mod build;
pub mod check;
pub mod console;
pub mod down;
pub mod init;
pub mod log;
pub mod up;
pub mod watch;

static DEVENV_CONFIG: &str = include_str!("../../devenv.local.nix");
static DEVENV_LOG: Lazy<String> = Lazy::new(|| {
    format!(
        "/tmp/devenv-{}.log",
        &Context::get().platform.path_hash[..8]
    )
});
static DEVENV_PID: Lazy<String> = Lazy::new(|| {
    Context::get()
        .platform
        .path
        .join(".devenv/state/devenv.pid")
        .display()
        .to_string()
});
