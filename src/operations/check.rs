use std::{path::PathBuf, process::Command};

use crate::{context::Context, crash, devenv, internal::AppExitCode, log};

pub fn main(
    verbose: bool,
    context: &Context,
    arg_paths: Option<Vec<PathBuf>>,
    no_ecs: bool,
    no_phpstan: bool,
) {
    if no_ecs && no_phpstan {
        crash!(
            AppExitCode::InvalidArgs,
            "There aren't any checks left to run..."
        );
    }

    let mut check_path_ecs: Vec<String> = vec!["src".to_owned(), "tests".to_owned()];
    let mut check_path_phpstan: Vec<String> = vec![];

    if let Some(paths) = arg_paths {
        let absolute_paths: Vec<String> = paths
            .into_iter()
            .map(|mut p| {
                if p.is_symlink() {
                    p = p.read_link().unwrap();
                }

                if p.is_relative() {
                    p = context.origin.join(p);
                }

                p.canonicalize()
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap()
            })
            .collect();

        log!(verbose, "{} {:?}", "Resolved paths:", absolute_paths);

        check_path_ecs = absolute_paths.clone();
        check_path_phpstan = absolute_paths;
    }

    let status_ecs = ecs(verbose, context, &check_path_ecs)
        .spawn()
        .unwrap()
        .wait();

    log!(verbose, "{} {:?}", "ECS exit Status:", status_ecs);

    let status_phpstan = phpstan(verbose, context, &check_path_phpstan)
        .spawn()
        .unwrap()
        .wait();

    log!(verbose, "{} {:?}", "PHPStan exit Status:", status_phpstan);
}

fn phpstan(verbose: bool, context: &Context, to_check: &[String]) -> Command {
    let mut curr_dir = String::from(".");

    if let Some(custom_context) = &context.custom {
        curr_dir = custom_context.path.display().to_string();
    }

    devenv!(
        verbose,
        "php src/Core/DevOps/StaticAnalyze/PHPStan/phpstan-bootstrap.php; cd {}; {} analyze --memory-limit=2G {}",
        curr_dir,
        context
            .platform
            .join("vendor/bin/phpstan")
            .display(),
        to_check.join(" ")
    )
}

fn ecs(verbose: bool, context: &Context, to_check: &[String]) -> Command {
    let mut curr_dir = String::from(".");

    if let Some(custom_context) = &context.custom {
        curr_dir = custom_context.path.display().to_string();
    }

    devenv!(
        verbose,
        "cd {}; {} check --fix {}",
        curr_dir,
        context.platform.join("vendor/bin/ecs").display(),
        to_check.join(" ")
    )
}
