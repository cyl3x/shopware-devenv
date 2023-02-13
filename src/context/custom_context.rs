use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

use serde_derive::Deserialize;

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
    pub fn check(path: &Path) -> Option<CustomType> {
        if path.display().to_string().contains("custom/plugins/") {
            return Some(CustomType::Plugin);
        }

        if path.display().to_string().contains("custom/apps/") {
            return Some(CustomType::App);
        }

        None
    }

    pub fn new(path: &Path, custom_type: CustomType) -> Self {
        let composer = Composer::new(path);
        let mut require = composer.require;

        if let Some(require_dev) = composer.require_dev {
            require.extend(require_dev);
        }

        let mut name = path
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap()
            .to_owned();

        if custom_type == CustomType::Plugin {
            if let Some(extra) = composer.extra {
                if let Some(plugin_class) = extra.plugin_class {
                    name = plugin_class.split('\\').last().unwrap().to_owned();
                }
            }
        }

        Self {
            path: path.to_path_buf(),
            name,
            custom_type,
            require,
        }
    }

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
    pub fn new(custom_path: &Path) -> Self {
        let json = fs::read_to_string(custom_path.join("composer.json")).unwrap();
        serde_json::from_str(&json).unwrap()
    }
}
