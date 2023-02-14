use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

use serde_derive::Deserialize;

use crate::config::Config;
use crate::log;

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

    pub fn new(config: &Config, path: &Path) -> Option<Self> {
        let Some(custom_type) = Self::get_type(path) else { return None; };

        let Some(composer) = Composer::new(path) else { return None; };

        let mut require = composer.require;

        if let Some(require_dev) = composer.require_dev {
            require.extend(require_dev);
        }

        let mut name: String = path
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .expect("Cannot get custom directory name")
            .to_owned();

        if custom_type == CustomType::Plugin {
            if let Some(plugin_class) = composer.extra.and_then(|i| i.plugin_class) {
                name = plugin_class.split('\\').last().unwrap_or(&name).to_owned();
            }

            if composer.plugin_type != "shopware-platform-plugin" {
                log!(
                    config,
                    "Found malformed Plugin: composer.json::type is not 'shopware-platform-plugin': {path}",
                    path = path.display(),
                );

                return None;
            }
        }

        log!(
            config,
            "Found custom context: {name} ({custom_type:?}) ({length} deps)",
            length = require.len()
        );

        Some(Self {
            path: path.to_path_buf(),
            name,
            custom_type,
            require,
        })
    }

    #[allow(dead_code)]
    pub fn move_to(&self) {
        env::set_current_dir(&self.path).expect("Cannot change context");
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
