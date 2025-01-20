mod custom_context;
mod platform_context;

use std::fs;
use std::path::PathBuf;

pub use custom_context::*;
use once_cell::sync::OnceCell;
pub use platform_context::*;

use crate::{verbose, OrFail};

static CONTEXT: OnceCell<Context> = OnceCell::new();

#[derive(Clone, Debug)]
pub struct Context {
    pub origin: PathBuf,
    pub platform: PlatformContext,
    pub custom: Option<CustomContext>,
}

impl Context {
    /// Init and returns the current context of the platform project.
    ///
    /// # Errors
    /// Fails if no valid context is found.
    pub fn get() -> anyhow::Result<&'static Self> {
        CONTEXT.get_or_try_init(|| {
            let mut current_dir =
                std::env::current_dir().or_panic("Could not get current directory".into());

            let context = Self::new(&mut current_dir)?;

            Ok(context)
        })
    }

    fn new(origin: &mut PathBuf) -> anyhow::Result<Self> {
        let mut custom: Option<CustomContext> = None;

        while {
            verbose!("Checking directory for context: {}", origin.display());

            if let Some(custom_context) = CustomContext::new(origin) {
                custom = Some(custom_context);
            }

            if let Some(platform_context) = PlatformContext::new(origin) {
                return Ok(Self {
                    origin: origin.clone(),
                    platform: platform_context,
                    custom,
                });
            }

            origin.pop()
        } {}

        anyhow::bail!("No valid context found")
    }

    /// Returns the pid of the devenv process.
    ///
    /// # Errors
    /// Fails if there is no pidfile
    /// Fails if the pid inside file is malformed (not <usize>).
    pub fn devenv_pid(&self) -> anyhow::Result<usize> {
        let path = self.platform.path.join(".devenv/processes.pid");

        let pid_string = fs::read_to_string(path).or_error("No pidfile found")?;

        let pid = pid_string
            .lines()
            .next()
            .and_then(|p| p.parse::<usize>().ok())
            .or_error("Malformed pid in pidfile")?;

        Ok(pid)
    }

    /// Returns the path to the logfile.
    #[must_use]
    pub fn log_file(&self) -> PathBuf {
        self.platform.path.join(".devenv/processes.log")
    }
}
