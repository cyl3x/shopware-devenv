use std::env;
use std::path::{Path, PathBuf};

use crate::{fail, sha256, verbose};

/// `PlatformContext` is the context created for the main platform directory.
#[derive(Clone, Debug)]
pub struct PlatformContext {
    pub path: PathBuf,
    pub path_hash: String,
}

impl PlatformContext {
    pub fn new(path: &Path) -> Option<Self> {
        let Ok(path_content) = path.read_dir() else { return None; };

        let mut has_devenv_file = false;
        let mut has_devenv_dir = false;

        for p in path_content
            .into_iter()
            .filter_map(Result::ok)
            .map(|e| e.path())
        {
            if p.is_file() && p.ends_with("devenv.nix") {
                has_devenv_file = true;
            }

            if p.is_dir() && p.ends_with(".devenv") {
                has_devenv_dir = true;
            }

            if has_devenv_dir && has_devenv_file {
                verbose!("Found platform context: {}", path.display());

                return Some(Self {
                    path: path.to_path_buf(),
                    path_hash: sha256!("{}", path.display()),
                });
            }
        }

        None
    }

    /// Moves the current working directory to the platform context path.
    pub fn move_cwd(&self) {
        if let Err(error) = env::set_current_dir(&self.path) {
            fail!("Failed to move to custom context: {error}");
        }
    }

    /// Joins a path to the platform context path.
    pub fn join(&self, path: &str) -> PathBuf {
        self.path.join(path)
    }

    /// Joins a path to the platform context path and returns it as a string.
    pub fn join_str(&self, path: &str) -> String {
        self.join(path).display().to_string()
    }
}
