use std::fs;
use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::context::Context;
use crate::internal::ExitCode;
use crate::{fail, project_dirs};

pub static DEVENV_DEFAULT_CONFIG: &str = include_str!("../../devenv.local.nix");

pub static CONFIG_FILE: Lazy<PathBuf> = Lazy::new(|| {
    let path = project_dirs!().config_dir().join("config.toml");

    path.parent()
        .and_then(|p| fs::create_dir_all(p).ok())
        .unwrap_or_else(|| {
            fail!(
                ExitCode::AppDirsCreation,
                "Failed to create config directory: {}",
                path.display(),
            )
        });

    path
});

pub static LOG_FILE: Lazy<PathBuf> = Lazy::new(|| {
    let path = project_dirs!().cache_dir().join(format!(
        "devenv-{}.log",
        &Context::get().platform.path_hash[..8]
    ));

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
    Context::get()
        .platform
        .path
        .join(".devenv/state/devenv.pid")
});
