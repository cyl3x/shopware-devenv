use std::path::PathBuf;

use crate::context::Context;
use crate::{direnv, fail, topic, verbose, Command};

pub fn main(arg_paths: Option<Vec<PathBuf>>, no_ecs: bool, no_phpstan: bool) {
    if no_ecs && no_phpstan {
        fail!("There aren't any checks left to run...");
    }

    let context = Context::get();

    let mut check_path_ecs: Vec<String> = vec!["src".to_owned(), "tests".to_owned()];
    let mut check_path_phpstan: Vec<String> = vec![];

    topic!("Processing paths...");
    if let Some(paths) = arg_paths {
        let absolute_paths: Vec<String> = paths
            .into_iter()
            .filter_map(|p| p.canonicalize().ok().map(|c| c.display().to_string()))
            .collect();

        verbose!("Resolved paths: {:?}", absolute_paths);

        check_path_ecs = absolute_paths.clone();
        check_path_phpstan = absolute_paths;
    }

    topic!("Running ecs...");
    direnv![&context.platform.join_str("vendor/bin/ecs")]
        .args(&check_path_ecs)
        .await_success();

    topic!("Running phpstan...");
    direnv![
        &context.platform.join_str("vendor/bin/phpstan"),
        "analyze",
        "--memory-limit=2G"
    ]
    .args(&check_path_phpstan)
    .await_success();
}
