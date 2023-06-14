use std::fs;
use std::path::PathBuf;
use std::process::Command;

use regex::Regex;

use crate::context::Context;
use crate::internal::AppExitCode;
use crate::operations::DEVENV_CONFIG;
use crate::{fail, sha256, spinner, success};

pub fn main() {
    Context::get().platform.move_to();

    let spinner = spinner!("Initialize...");

    config();

    if let Err(error) = Command::new("devenv")
        .arg("ci")
        .spawn()
        .expect("Cannot spawn cmd")
        .wait()
    {
        spinner.clear();
        fail!(
            AppExitCode::DevenvExec,
            "Non zero exit from devenv: {error}"
        );
    }

    spinner.clear();
    success!("Init successfully");
}

fn config() {
    let Ok(config) = fs::read_to_string("./devenv.local.nix") else {
        create_config();
        return;
    };

    let config = config;

    let regex = Regex::new(r"^(# sha256<)([a-zA-Z0-9]{64})(>)$").expect("Invalid regex");
    let mut lines = config.lines();

    if config.lines().count() < 10 {
        println!("Found personal devenv.local.nix, backing up...");
        backup_create();
        return;
    }

    let first_line = lines.next().expect("No first line?");

    if !regex.is_match(first_line) {
        println!("Found personal devenv.local.nix, backing up...");
        backup_create();
        return;
    }

    let stored_hash = &regex
        .captures(first_line)
        .expect("Invalid devenv.local.nix file header")[2];
    let file_hash = sha256!("{}", lines.skip(1).collect::<String>());
    let internal_hash = sha256!("{}", DEVENV_CONFIG);

    if internal_hash == stored_hash {
        println!("Found swde devenv.local.nix, but no update is needed");
        return;
    }

    if stored_hash != file_hash {
        println!("Found modified swde devenv.local.nix, backing up and updating...");
        backup_create();
        return;
    }

    println!("Found swde devenv.local.nix, updating...");
    create_config();
}

fn create_config() {
    let hash = sha256!("{}", DEVENV_CONFIG);
    let config = format!("# sha256<{hash}>\n{DEVENV_CONFIG}");

    let result = fs::write("devenv.local.nix", config);

    if let Err(error) = result {
        fail!(
            AppExitCode::ConfigWrite,
            "An error occured while writing to devenv.local.nix: {}",
            error
        );
    }

    println!("Wrote devenv.local.nix");
}

fn backup_create() {
    let mut i = 1;

    while PathBuf::from(format!("devenv.local.nix.{i}.bak")).is_file() {
        i += 1;

        if i > 10 {
            fail!(
                AppExitCode::ConfigBak,
                "Please clean up your devenv.local.nix.bak files. Who needs more than 10.."
            );
        }
    }

    let result = fs::rename("devenv.local.nix", format!("devenv.local.nix.{i}.bak"));

    if let Err(error) = result {
        fail!(
            AppExitCode::ConfigBak,
            "An error occured while backing up your devenv.local.nix: {}",
            error
        );
    } else {
        println!("Backed up devenv.local.nix to devenv.local.nix.{i}.bak");
    }

    create_config();
}
