use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

use serde::Deserialize;

use crate::{verbose, warn, OrFail};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CustomType {
    App,
    Plugin,
}

/// `CustomContext` is the context created for `custom/apps` & `custom/plugins`.
#[derive(Clone, Debug)]
pub struct CustomContext {
    pub path: PathBuf,
    pub name: String,
    pub custom_type: CustomType,
    pub require: HashMap<String, String>,
}

impl CustomContext {
    fn get_type(path: &Path) -> Option<CustomType> {
        if path.display().to_string().contains("custom/plugins/") {
            return Some(CustomType::Plugin);
        }

        if path.display().to_string().contains("custom/apps/") {
            return Some(CustomType::App);
        }

        None
    }

    pub fn new(path: &Path) -> Option<Self> {
        let Some(custom_type) = Self::get_type(path) else { return None; };
        let Some(composer) = Composer::new(path) else { return None; };

        let mut require = composer.require;

        if let Some(require_dev) = composer.require_dev {
            require.extend(require_dev);
        }

        let Some(mut name) = path
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .map(std::borrow::ToOwned::to_owned)
        else {
            warn!("Malformed path: {}", path.display());
            return None;
        };

        if custom_type == CustomType::Plugin {
            if let Some(p_class) = composer.extra.and_then(|i| i.plugin_class) {
                if let Some(p_name) = p_class.split('\\').last() {
                    name = p_name.to_string();
                }
            }

            if composer.plugin_type != "shopware-platform-plugin" {
                warn!(
                    "Found malformed Plugin: composer.json::type is not 'shopware-platform-plugin': {}",
                    path.display(),
                );

                return None;
            }
        }

        verbose!(
            "Found custom context: {name} ({custom_type:?}) ({} deps)",
            require.len()
        );

        Some(Self {
            path: path.to_owned(),
            name,
            custom_type,
            require,
        })
    }

    /// Moves the current working directory to the custom context path.
    #[allow(dead_code)]
    pub fn move_cwd(&self) {
        env::set_current_dir(&self.path).or_panic("Could not move to platform context".into());
    }
}

/// Minimal `composer.json` structure
#[derive(Clone, Deserialize, Debug)]
struct Composer {
    pub extra: Option<ComposerExtra>,

    #[serde(rename = "type")]
    pub plugin_type: String,

    pub require: HashMap<String, String>,

    #[serde(rename = "require-dev")]
    pub require_dev: Option<HashMap<String, String>>,
}

#[derive(Clone, Deserialize, Debug)]
struct ComposerExtra {
    #[serde(rename = "shopware-plugin-class")]
    pub plugin_class: Option<String>,
}

impl Composer {
    pub fn new(custom_path: &Path) -> Option<Self> {
        fs::read_to_string(custom_path.join("composer.json"))
            .ok()
            .and_then(|file| serde_json::from_str(&file).ok())
    }
}
