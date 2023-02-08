mod custom_context;
mod platform_context;

pub use custom_context::*;
pub use platform_context::*;

use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Context {
    pub origin: PathBuf,
    pub platform: PlatformContext,
    pub custom: Option<CustomContext>,
}

impl Context {
    pub fn new(origin_path: PathBuf) -> Self {
        let mut custom: Option<CustomContext> = None;
        let mut platform: Option<PlatformContext> = None;

        let mut curr_dir = origin_path.clone();

        while !curr_dir.ends_with("/") {
            if let Some(custom_type) = CustomContext::check(&curr_dir) {
                custom = Some(CustomContext::new(&curr_dir, custom_type));
            }

            if PlatformContext::check(&curr_dir) {
                platform = Some(PlatformContext::new(&curr_dir));
                break;
            }

            curr_dir.pop();
        }

        Self {
            origin: origin_path,
            platform: platform.expect("Could not find platform context"),
            custom,
        }
    }
}
