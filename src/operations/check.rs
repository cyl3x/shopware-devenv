use std::path::PathBuf;
use std::process::Command;

use crate::context::Context;
use crate::internal::AppExitCode;
use crate::{devenv, fail, log_verbose};

pub fn main(arg_paths: Option<Vec<PathBuf>>, no_ecs: bool, no_phpstan: bool) {
    if no_ecs && no_phpstan {
        fail!(
            AppExitCode::InvalidArgs,
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

        log_verbose!("{} {:?}", "Resolved paths:", absolute_paths);

        check_path_ecs = absolute_paths.clone();
        check_path_phpstan = absolute_paths;
    }

    if let Err(error) = ecs(context, &check_path_ecs)
        .spawn()
        .unwrap_or_else(|_| fail!(AppExitCode::Runtime, "Failed to start ECS"))
        .wait()
    {
        fail!(AppExitCode::DevenvExec, "Non zero exit for ECS: {error}");
    }

    if let Err(error) = phpstan(context, &check_path_phpstan)
        .spawn()
        .unwrap_or_else(|_| fail!(AppExitCode::Runtime, "Failed to start PHPStan"))
        .wait()
    {
        fail!(
            AppExitCode::DevenvExec,
            "Non zero exit for PHPStan: {error}"
        );
    }
}

fn phpstan(context: &Context, to_check: &[String]) -> Command {
    let mut curr_dir = String::from(".");

    if let Some(custom_context) = &context.custom {
        curr_dir = custom_context.path.display().to_string();
    }

    devenv!(
        "cd {}; {} analyze --memory-limit=2G {}",
        curr_dir,
        context.platform.join("vendor/bin/phpstan").display(),
        to_check.join(" ")
    )
}

fn ecs(context: &Context, to_check: &[String]) -> Command {
    let mut curr_dir = String::from(".");

    if let Some(custom_context) = &context.custom {
        curr_dir = custom_context.path.display().to_string();
    }

    devenv!(
        "cd {}; {} check --fix {}",
        curr_dir,
        context.platform.join("vendor/bin/ecs").display(),
        to_check.join(" ")
    )
}
