use std::env;
use std::path::{Path, PathBuf};

use crate::log;

#[derive(Clone, Debug)]
pub struct PlatformContext {
    pub path: PathBuf,
}

impl PlatformContext {
    pub fn new(verbose: bool, path: &Path) -> Option<Self> {
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
                log!(
                    verbose,
                    "Found platform context: {path}",
                    path = path.display()
                );

                return Some(Self {
                    path: path.to_path_buf(),
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
