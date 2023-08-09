use std::fs;
use std::path::PathBuf;

use regex::Regex;

use crate::context::Context;
use crate::{devenv, sha256, topic, Command, DEVENV_DEFAULT_CONFIG};

pub fn main() -> anyhow::Result<String> {
    Context::get()?.platform.move_cwd();

    fs::read_to_string("./devenv.local.nix")
        .map_or_else(|_| create_config(), |config| update_config(&config))?;

    devenv!["ci"].await_success()?;

    Ok("Init successfully".into())
}

fn update_config(config: &str) -> anyhow::Result<()> {
    let regex = Regex::new(r"^(# sha256<)([a-zA-Z0-9]{64})(>)$").expect("Invalid regex");
    let mut lines = config.lines();

    let first_line = lines.next().unwrap_or_default();
    let Some(stored_file_hash) = regex.captures(first_line) else {
        topic!("Found personal devenv.local.nix, backing up...");
        return backup_create();
    };

    let file_hash = sha256!("{}", lines.skip(1).collect::<String>());
    let internal_hash = sha256!("{}", DEVENV_DEFAULT_CONFIG);

    if stored_file_hash[2] == internal_hash {
        topic!("Found swde devenv.local.nix and no update is needed");
        return Ok(());
    }

    if stored_file_hash[2] != file_hash {
        topic!("Found modified swde devenv.local.nix, backing up and updating...");
        return backup_create();
    }

    topic!("Found swde devenv.local.nix, updating...");
    create_config()
}

fn create_config() -> anyhow::Result<()> {
    let hash = sha256!("{}", DEVENV_DEFAULT_CONFIG);
    let config = format!("# sha256<{hash}>\n{DEVENV_DEFAULT_CONFIG}");

    topic!("Write devenv.local.nix");

    if let Err(e) = fs::write("devenv.local.nix", config) {
        anyhow::bail!("An error occured while writing to devenv.local.nix: {e}")
    }

    Ok(())
}

fn backup_create() -> anyhow::Result<()> {
    let mut i = 1;

    while PathBuf::from(format!("devenv.local.nix.{i}.bak")).is_file() {
        i += 1;

        if i > 5 {
            anyhow::bail!(
                "Please clean up your devenv.local.nix.bak files.\n  Who needs more than 5.."
            );
        }
    }

    if let Err(e) = fs::rename("devenv.local.nix", format!("devenv.local.nix.{i}.bak")) {
        anyhow::bail!("An error occured while backing up your devenv.local.nix: {e}")
    }

    topic!("Backed up devenv.local.nix to devenv.local.nix.{i}.bak");

    create_config()
}
