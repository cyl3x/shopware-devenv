use std::process::Stdio;

use crate::{devenv, direnv, direnv_git, topic, Command, Context};

pub fn main() {
    Context::get().platform.move_cwd();

    topic!("Fetching newest branches...");
    direnv_git!["fetch"].await_success();

    topic!("Checking platform branch for updates...");
    let output = direnv_git!["branch", "--show-current"].await_output();

    if let Ok(branch) = String::from_utf8(output.stdout).map(|s| s.trim().to_string()) {
        topic!("Skipping branch update, brach is '{branch}'...");

        if branch == "trunk" {
            topic!("Updating platform brach 'trunk'...");
            direnv_git!["pull"].await_success();
        }
    }

    topic!("Update composer dependencies...");
    direnv!["composer", "update"].await_success();

    topic!("Migrate database...");
    direnv!["bin/console", "database:migrate", "--all", "--quiet"].await_success();
    direnv![
        "bin/console",
        "database:migrate-destructive",
        "--all",
        "--quiet"
    ]
    .await_success();

    topic!("Clearing cache for env=dev...");
    direnv!["bin/console", "cache:clear", "--env=dev", "--quiet"].await_success();

    topic!("Clearing cache for env=prod...");
    direnv!["bin/console", "cache:clear", "--env=prod", "--quiet"].await_success();

    topic!("Garbage collecting devenv...");
    devenv!["gc"].stdout(Stdio::null()).await_success();
}
