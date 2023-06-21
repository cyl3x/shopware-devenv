mod custom_context;
mod platform_context;

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
    pub fn get() -> &'static Self {
        CONTEXT.get_or_init(|| {
            let mut current_dir =
                std::env::current_dir().or_fail("Could not get current directory");

            Self::new(&mut current_dir).or_fail("Current directory has no valid context")
        })
    }

    fn new(origin: &mut PathBuf) -> Option<Self> {
        let mut custom: Option<CustomContext> = None;

        while {
            verbose!("Checking directory for context: {}", origin.display());

            if let Some(custom_context) = CustomContext::new(origin) {
                custom = Some(custom_context);
            }

            if let Some(platform_context) = PlatformContext::new(origin) {
                return Some(Self {
                    origin: origin.clone(),
                    platform: platform_context,
                    custom,
                });
            }

            origin.pop()
        } {}

        None
    }
}
