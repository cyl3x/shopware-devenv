use std::path::PathBuf;

use crate::context::Context;
use crate::{direnv, fail, AppCommand, ExitCode};

pub fn main(arg_paths: Option<Vec<PathBuf>>, no_ecs: bool, no_phpstan: bool) {
    if no_ecs && no_phpstan {
        fail!(
            ExitCode::InvalidArgs,
            "There aren't any checks left to run..."
        );
    }

    let context = Context::get();

    let mut check_path_ecs: Vec<String> = vec!["src".to_owned(), "tests".to_owned()];
    let mut check_path_phpstan: Vec<String> = vec![];

    if let Some(paths) = arg_paths {
        let absolute_paths: Vec<String> = paths
            .into_iter()
            .filter_map(|p| p.canonicalize().ok().map(|c| c.display().to_string()))
            .collect();

        log::info!("Resolved paths: {:?}", absolute_paths);

        check_path_ecs = absolute_paths.clone();
        check_path_phpstan = absolute_paths;
    }

    direnv![
        path = context.custom.clone().map(|c| c.path),
        &context.platform.join_str("vendor/bin/ecs")
    ]
    .args(&check_path_ecs)
    .start_await_success();

    direnv![
        path = context.custom.clone().map(|c| c.path),
        &context.platform.join_str("vendor/bin/phpstan"),
        "analyze",
        "--memory-limit=2G"
    ]
    .args(&check_path_phpstan)
    .start_await_success();
}
