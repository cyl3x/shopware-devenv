use std::path::PathBuf;
use std::process::Command;

use crate::config::Config;
use crate::context::Context;
use crate::internal::AppExitCode;
use crate::{crash, devenv, log};

pub fn main(
    config: &Config,
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
                    p = p.read_link().expect("Cannot resolve symlink");
                }

                if p.is_relative() {
                    p = context.origin.join(p);
                }

                p.canonicalize()
                    .expect("Cannot resolve path")
                    .into_os_string()
                    .into_string()
                    .expect("Cannot convert path to string")
            })
            .collect();

        log!(config, "{} {:?}", "Resolved paths:", absolute_paths);

        check_path_ecs = absolute_paths.clone();
        check_path_phpstan = absolute_paths;
    }

    let status_ecs = ecs(config, context, &check_path_ecs)
        .spawn()
        .expect("Cannot start ECS")
        .wait();

    log!(config, "{} {:?}", "ECS exit Status:", status_ecs);

    let status_phpstan = phpstan(config, context, &check_path_phpstan)
        .spawn()
        .expect("Cannot start PHPStan")
        .wait();

    log!(config, "{} {:?}", "PHPStan exit Status:", status_phpstan);
}

fn phpstan(config: &Config, context: &Context, to_check: &[String]) -> Command {
    let mut curr_dir = String::from(".");

    if let Some(custom_context) = &context.custom {
        curr_dir = custom_context.path.display().to_string();
    }

    devenv!(
        config,
        "php src/Core/DevOps/StaticAnalyze/PHPStan/phpstan-bootstrap.php; cd {}; {} analyze --memory-limit=2G {}",
        curr_dir,
        context
            .platform
            .join("vendor/bin/phpstan")
            .display(),
        to_check.join(" ")
    )
}

fn ecs(config: &Config, context: &Context, to_check: &[String]) -> Command {
    let mut curr_dir = String::from(".");

    if let Some(custom_context) = &context.custom {
        curr_dir = custom_context.path.display().to_string();
    }

    devenv!(
        config,
        "cd {}; {} check --fix {}",
        curr_dir,
        context.platform.join("vendor/bin/ecs").display(),
        to_check.join(" ")
    )
}
