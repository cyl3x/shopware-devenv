use std::{fs, path::PathBuf};

use regex::Regex;

use crate::{
    crash,
    internal::{AppExitCode, DEVENV_CONFIG},
    log, sha256,
};

pub fn main(verbose: bool) {
    let config = fs::read_to_string("./devenv.local.nix");

    if config.is_err() {
        create_config(verbose);
        return;
    }

    let config = config.unwrap();

    let regex = Regex::new(r"^(# sha256<)([a-zA-Z0-9]{64})(>)$").unwrap();
    let mut lines = config.lines();

    if config.lines().count() < 10 {
        log!(verbose, "Found personal devenv.local.nix, backing up...");
        backup_create(verbose);
        return;
    }

    let first_line = lines.next().unwrap();

    if !regex.is_match(first_line) {
        log!(verbose, "Found personal devenv.local.nix, backing up...");
        backup_create(verbose);
        return;
    }

    let stored_hash = &regex.captures(first_line).unwrap()[2];
    let file_hash = sha256!("{}", lines.skip(1).collect::<String>());
    let internal_hash = sha256!("{}", DEVENV_CONFIG);

    if internal_hash == stored_hash {
        log!(
            verbose,
            "Found swde devenv.local.nix, but no update is needed"
        );
        return;
    }

    if stored_hash != file_hash {
        log!(
            verbose,
            "Found modified swde devenv.local.nix, backing up and updating..."
        );
        backup_create(verbose);
        return;
    }

    log!(verbose, "Found swde devenv.local.nix, updating...");
    create_config(verbose);
}

fn create_config(verbose: bool) {
    let hash = sha256!("{}", DEVENV_CONFIG);
    let config = format!("# sha256<{hash}>\n{DEVENV_CONFIG}");

    let result = fs::write("devenv.local.nix", config);

    if let Err(error) = result {
        crash!(
            AppExitCode::ConfigWrite,
            "An error occured while writing to devenv.local.nix: {}",
            error
        );
    }

    log!(verbose, "Wrote devenv.local.nix");
}

fn backup_create(verbose: bool) {
    let mut i = 1;

    while PathBuf::from(format!("devenv.local.nix.{i}.bak")).is_file() {
        i += 1;

        if i > 10 {
            crash!(
                AppExitCode::ConfigBak,
                "Please clean up your devenv.local.nix.bak files. Who needs more than 10.."
            );
        }
    }

    let result = fs::rename("devenv.local.nix", format!("devenv.local.nix.{i}.bak"));

    if let Err(error) = result {
        crash!(
            AppExitCode::ConfigBak,
            "An error occured while backing up your devenv.local.nix: {}",
            error
        );
    } else {
        log!(
            verbose,
            "Backed up devenv.local.nix to devenv.local.nix.{}.bak",
            i
        );
    }

    create_config(verbose);
}
