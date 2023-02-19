use std::fs;

use merge_struct::merge;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Data {
    config: Config,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    pub verbose: bool,
}

impl Config {
    pub fn get() -> &'static Self {
        CONFIG.get_or_init(Self::new)
    }

    fn new() -> Self {
        let mut config = Self { verbose: false };

        let content = dirs::config_dir().and_then(|mut cd| {
            cd.push("swde/config.toml");
            fs::read_to_string(cd).ok()
        });

        if let Some(content) = content {
            let file_config: Self = match toml::from_str::<Data>(&content) {
                Ok(d) => d.config,
                Err(error) => {
                    println!("Invalid syntax in config.toml: {error}");
                    return config;
                },
            };

            config = match merge::<Self>(&config, &file_config) {
                Ok(config) => config,
                Err(error) => {
                    println!("Internal error while processing config.toml: {error}");
                    config
                },
            };
        }

        config
    }
}
