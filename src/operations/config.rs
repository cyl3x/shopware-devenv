use std::fs;
use std::path::PathBuf;

use regex::Regex;

use crate::context::Context;
use crate::{devenv, fail, sha256, success, topic, Command, OrFail, DEVENV_DEFAULT_CONFIG};

pub fn main() {
    Context::get().platform.move_cwd();

    fs::read_to_string("./devenv.local.nix")
        .map_or_else(|_| create_config(), |config| update_config(&config));

    devenv!["ci"].await_success();

    success!("Init successfully");
}

fn update_config(config: &str) {
    let regex = Regex::new(r"^(# sha256<)([a-zA-Z0-9]{64})(>)$").expect("Invalid regex");
    let mut lines = config.lines();

    if config.lines().count() < 10 {
        topic!("Found personal devenv.local.nix, backing up...");
        return backup_create();
    }

    let first_line = lines.next().or_fail("Malformed devenv.local.nix file");

    if !regex.is_match(first_line) {
        topic!("Found personal devenv.local.nix, backing up...");
        return backup_create();
    }

    let stored_hash = &regex
        .captures(first_line)
        .or_fail("Malformed devenv.local.nix file")[2];
    let file_hash = sha256!("{}", lines.skip(1).collect::<String>());
    let internal_hash = sha256!("{}", DEVENV_DEFAULT_CONFIG);

    if internal_hash == stored_hash {
        topic!("Found swde devenv.local.nix, but no update is needed");
        return;
    }

    if stored_hash != file_hash {
        topic!("Found modified swde devenv.local.nix, backing up and updating...");
        return backup_create();
    }

    topic!("Found swde devenv.local.nix, updating...");
    create_config();
}

fn create_config() {
    let hash = sha256!("{}", DEVENV_DEFAULT_CONFIG);
    let config = format!("# sha256<{hash}>\n{DEVENV_DEFAULT_CONFIG}");

    topic!("Write devenv.local.nix");

    if let Err(e) = fs::write("devenv.local.nix", config) {
        fail!("An error occured while writing to devenv.local.nix: {e}")
    }
}

fn backup_create() {
    let mut i = 1;

    while PathBuf::from(format!("devenv.local.nix.{i}.bak")).is_file() {
        i += 1;

        if i > 10 {
            fail!("Please clean up your devenv.local.nix.bak files. Who needs more than 10..");
        }
    }

    if let Err(e) = fs::rename("devenv.local.nix", format!("devenv.local.nix.{i}.bak")) {
        fail!("An error occured while backing up your devenv.local.nix: {e}")
    }

    topic!("Backed up devenv.local.nix to devenv.local.nix.{i}.bak");

    create_config();
}
