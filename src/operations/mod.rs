use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::context::Context;
use crate::project_dirs;

pub mod build;
pub mod check;
pub mod console;
pub mod down;
pub mod init;
pub mod log;
pub mod plugin;
pub mod up;
pub mod watch;

static DEVENV_CONFIG: &str = include_str!("../../devenv.local.nix");

static DEVENV_LOG: Lazy<PathBuf> = Lazy::new(|| {
    project_dirs!().cache_dir().join(format!(
        "devenv-{}.log",
        &Context::get().platform.path_hash[..8]
    ))
});

static DEVENV_PID: Lazy<String> = Lazy::new(|| {
    Context::get()
        .platform
        .path
        .join(".devenv/state/devenv.pid")
        .display()
        .to_string()
});
