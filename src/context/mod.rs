mod custom_context;
mod platform_context;

use std::path::{Path, PathBuf};

pub use custom_context::*;
use once_cell::sync::OnceCell;
pub use platform_context::*;

use crate::crash;
use crate::internal::AppExitCode;

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
            let current_dir =
                std::env::current_dir().expect("Insufficient permissions or invalid current path");
            let Some(context) = Self::new(&current_dir) else {
                crash!(
                    AppExitCode::InvalidContext,
                    "Current directory has not a valid context"
                );
            };

            context
        })
    }

    fn new(origin_path: &Path) -> Option<Self> {
        let mut custom: Option<CustomContext> = None;

        let mut curr_dir = origin_path.to_owned();

        while !curr_dir.ends_with("/") {
            if let Some(custom_context) = CustomContext::new(&curr_dir) {
                custom = Some(custom_context);
            }

            if let Some(platform_context) = PlatformContext::new(&curr_dir) {
                return Some(Self {
                    origin: origin_path.to_path_buf(),
                    platform: platform_context,
                    custom,
                });
            }

            curr_dir.pop();
        }

        None
    }
}
