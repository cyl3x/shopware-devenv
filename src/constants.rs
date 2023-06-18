use std::fs;
use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::{Context, ExitCode, fail, project_dirs};

pub static DEVENV_DEFAULT_CONFIG: &str = include_str!("../devenv.local.nix");

pub static LOG_FILE: Lazy<PathBuf> = Lazy::new(|| {
    let path = project_dirs!().cache_dir().join(format!(
        "devenv-{}.log",
        &Context::get().platform.path_hash[..8]
    ));

    log::info!("Comiled log file path: {}", path.display());

    path.parent()
        .and_then(|p| fs::create_dir_all(p).ok())
        .unwrap_or_else(|| {
            fail!(
                ExitCode::AppDirsCreation,
                "Failed to create log directory: {}",
                path.display(),
            )
        });

    path
});

pub static DEVENV_PID: Lazy<PathBuf> = Lazy::new(|| {
    let path = Context::get()
        .platform
        .path
        .join(".devenv/state/devenv.pid");

    log::info!("Comiled devenv pid file path: {}", path.display());

    path
});
