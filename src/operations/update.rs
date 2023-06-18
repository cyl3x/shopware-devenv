use std::process::Stdio;

use crate::{devenv, AppCommand, direnv, spinner};


pub fn main() {
    spinner!("Checking platform branch for updates...");
    direnv!["git", "fetch", "--quiet"].start_await_success();

    let output = direnv!["git", "branch", "--show-current"]
        .output()
        .expect("runtime error");

    if let Ok(branch) = String::from_utf8(output.stdout).map(|s| s.trim().to_string()) {
        log::info!("Current platform branch: {}", branch);

        if branch == "trunk" {
            spinner!("Update platform brach 'trunk'...");
            direnv!["git", "pull", "--quiet"].start_await_success();
        }
    }

    spinner!("Update composer dependencies...");
    direnv!["composer", "update", "--quiet"].start_await_success();

    spinner!("Migrate database...");
    direnv!["bin/console", "database:migrate", "--all", "--quiet"].start_await_success();
    direnv!["bin/console", "database:migrate-destructive", "--all", "--quiet"].start_await_success();
    
    spinner!("Clear cache for env=dev");
    direnv!["bin/console", "cache:clear", "--env=dev", "--quiet"].start_await_success();

    spinner!("Clear cache for env=prod");
    direnv!["bin/console", "cache:clear", "--env=prod", "--quiet"].start_await_success();

    spinner!("Garbage collect devenv...");
    devenv!["gc"].stdout(Stdio::null()).start_await_success();
}