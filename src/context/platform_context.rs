use std::{
    env,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug)]
pub struct PlatformContext {
    pub path: PathBuf,
}

impl PlatformContext {
    pub fn check(path: &Path) -> bool {
        let paths = path.read_dir().unwrap();

        let mut has_devenv_file = false;
        let mut has_devenv_dir = false;

        for p in paths.into_iter().filter_map(Result::ok).map(|e| e.path()) {
            if p.is_file() && p.ends_with("devenv.nix") {
                has_devenv_file = true;
            }

            if p.is_dir() && p.ends_with(".devenv") {
                has_devenv_dir = true;
            }

            if has_devenv_dir && has_devenv_file {
                return true;
            }
        }

        false
    }

    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    pub fn move_to(&self) {
        env::set_current_dir(&self.path).expect("Cannot change context");
    }

    pub fn join(&self, path: &str) -> PathBuf {
        self.path.join(path)
    }
}
