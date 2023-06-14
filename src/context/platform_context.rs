use std::env;
use std::path::{Path, PathBuf};

use crate::{log, sha256};

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

            let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

            if has_devenv_dir && has_devenv_file {
                log!("Found platform context: {}", path.display());

                return Some(Self {
                    path: path.clone(),
                    path_hash: sha256!("{}", path.display()),
                });
            }
        }

        None
    }

    pub fn move_to(&self) {
        env::set_current_dir(&self.path).expect("Cannot change context");
    }

    pub fn join(&self, path: &str) -> PathBuf {
        self.path.join(path)
    }
}
