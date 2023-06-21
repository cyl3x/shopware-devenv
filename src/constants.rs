use std::fs;
use std::path::PathBuf;

use once_cell::sync::{Lazy, OnceCell};

use crate::{project_dirs, OrFail, Context};

pub static VERBOSE: OnceCell<bool> = OnceCell::new();

pub static DEVENV_DEFAULT_CONFIG: &str = include_str!("../devenv.local.nix");

pub static LOG_FILE: Lazy<PathBuf> = Lazy::new(|| {
    let path = project_dirs!().cache_dir().join(format!(
        "devenv-{}.log",
        &Context::get().platform.path_hash[..8]
    ));

    path.parent()
        .and_then(|p| fs::create_dir_all(p).ok())
        .or_fail(&format!(
            "Failed to create log directory: {}",
            path.display()
        ));

    path
});

pub static DEVENV_PID: Lazy<PathBuf> = Lazy::new(|| {
    Context::get()
        .platform
        .path
        .join(".devenv/state/devenv.pid")
});
