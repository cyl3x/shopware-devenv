use colored::Colorize;

use crate::internal::{AppCommand, ExitCode};
use crate::{direnv, fail, success};

pub fn platform(gen_demodata: bool, build_test_db: bool) {
    direnv!["composer", "update"].start_await_success();

    direnv!["composer", "setup"].start_await_success();

    if gen_demodata {
        demodata(&[]);
    }

    if build_test_db {
        test_db();
    }

    success!("Build successfull");
    // TODO - Add additional URL https://platform.dev.localhost:4000
}

pub fn test_db() {
    if let Err(error) = direnv!["composer", "init:testdb"].start().wait() {
        fail!(
            ExitCode::DevenvExec,
            "Non zero exit: {error}\n{}",
            "If you're using platform 6.4 you probably have to do it manually".red()
        );
    } else {
        success!("Build successfull");
    }
}

pub fn admin() {
    direnv!["composer", "build:js:admin"].start_await_success();
    success!("Build successfull");
}

pub fn storefront() {
    direnv!["composer", "build:js:storefront"].start_await_success();
    success!("Build successfull");
}

pub fn demodata(args: &[String]) {
    direnv!["bin/console", "framework:demodata"]
        .args(args)
        .env("APP_ENV", "prod")
        .start_await_success();

    direnv!["bin/console", "dal:refresh:index"]
        .env("APP_ENV", "prod")
        .start_await_success();
}
