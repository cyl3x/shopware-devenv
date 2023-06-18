mod custom_context;
mod platform_context;

use std::path::PathBuf;

pub use custom_context::*;
use once_cell::sync::OnceCell;
pub use platform_context::*;

use crate::{fail, ExitCode};

static CONTEXT: OnceCell<Context> = OnceCell::new();

#[derive(Clone, Debug)]
pub struct Context {
    pub origin: PathBuf,
    pub platform: PlatformContext,
    pub custom: Option<CustomContext>,
}

impl Context {
    pub fn get() -> &'static Self {
        CONTEXT.get_or_init(|| {
            let Ok(current_dir) = std::env::current_dir() else {
                fail!(
                    ExitCode::InvalidContext,
                    "Could not get current directory"
                );
            };

            Self::new(current_dir).unwrap_or_else(|| {
                fail!(
                    ExitCode::InvalidContext,
                    "Current directory has no valid context"
                );
            })
        })
    }

    fn new(origin: PathBuf) -> Option<Self> {
        let mut custom: Option<CustomContext> = None;
        let mut origin = origin;

        // TODO - Does not work always (windows for example, symlinks)
        while {
            log::debug!("Checking directory for context: {}", origin.display());

            if let Some(custom_context) = CustomContext::new(&origin) {
                custom = Some(custom_context);
            }

            if let Some(platform_context) = PlatformContext::new(&origin) {
                return Some(Self {
                    origin,
                    platform: platform_context,
                    custom,
                });
            }

            origin.pop()
        } {}

        None
    }
}
