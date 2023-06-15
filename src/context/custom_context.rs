use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

use serde::Deserialize;

use crate::{fail, log_info, log_verbose, AppExitCode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CustomType {
    App,
    Plugin,
}

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
            .map(std::borrow::ToOwned::to_owned) else {
                log_info!("Malformed path: {}", path.display());
                return None;
            };

        if custom_type == CustomType::Plugin {
            if let Some(plugin_class) = composer.extra.and_then(|i| i.plugin_class) {
                if let Some(plugin_name) = plugin_class.split('\\').last() {
                    name = plugin_name.to_owned();
                };
            }

            if composer.plugin_type != "shopware-platform-plugin" {
                log_verbose!(
                    "Found malformed Plugin: composer.json::type is not 'shopware-platform-plugin': {path}",
                    path = path.display(),
                );

                return None;
            }
        }

        log_verbose!(
            "Found custom context: {name} ({custom_type:?}) ({length} deps)",
            length = require.len()
        );

        Some(Self {
            path: path.to_owned(),
            name,
            custom_type,
            require,
        })
    }

    #[allow(dead_code)]
    pub fn move_to(&self) {
        if env::set_current_dir(&self.path).is_err() {
            fail!(
                AppExitCode::Runtime,
                "Failed to move to custom context: {p}",
                p = self.path.display()
            );
        }
    }
}

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
        let file_result = fs::read_to_string(custom_path.join("composer.json"));

        if let Ok(file) = file_result {
            return serde_json::from_str(&file).ok();
        }

        None
    }
}
