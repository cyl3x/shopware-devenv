mod custom_context;
mod platform_context;

use std::path::{Path, PathBuf};

pub use custom_context::*;
pub use platform_context::*;

#[derive(Clone, Debug)]
pub struct Context {
    pub origin: PathBuf,
    pub platform: PlatformContext,
    pub custom: Option<CustomContext>,
}

impl Context {
    pub fn new(verbose: bool, origin_path: &Path) -> Option<Self> {
        let mut custom: Option<CustomContext> = None;

        let mut curr_dir = origin_path.to_owned();

        while !curr_dir.ends_with("/") {
            if let Some(custom_context) = CustomContext::new(verbose, &curr_dir) {
                custom = Some(custom_context);
            }

            if let Some(platform_context) = PlatformContext::new(verbose, &curr_dir) {
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
