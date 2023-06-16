use colored::Colorize;

use crate::internal::AppExitCode;
use crate::{direnv, fail, success};

pub fn platform(gen_demodata: bool, build_test_db: bool) {
    if let Err(error) = direnv!["composer", "update"]
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        fail!(AppExitCode::DevenvExec, "Non zero exit for update: {error}");
    }

    if let Err(error) = direnv!["composer", "setup"]
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        fail!(AppExitCode::DevenvExec, "Non zero exit for setup: {error}");
    }

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
    if let Err(error) = direnv!["composer", "init:testdb"]
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        fail!(
            AppExitCode::DevenvExec,
            "Non zero exit: {error}\n{}",
            "If you're using platform 6.4 you probably have to do it manually".red()
        );
    } else {
        success!("Build successfull");
    }
}

pub fn admin() {
    if let Err(error) = direnv!["composer", "build:js:admin"]
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        fail!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    } else {
        success!("Build successfull");
    }
}

pub fn storefront() {
    if let Err(error) = direnv!["composer", "build:js:storefront"]
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        fail!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    } else {
        success!("Build successfull");
    }
}

pub fn demodata(args: &[String]) {
    if let Err(error) = direnv!["bin/console", "framework:demodata"]
        .args(args)
        .env("APP_ENV", "prod")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        fail!(
            AppExitCode::DevenvExec,
            "Non zero exit from demodata: {error}"
        );
    }

    if let Err(error) = direnv!["bin/console", "dal:refresh:index"]
        .env("APP_ENV", "prod")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        fail!(
            AppExitCode::DevenvExec,
            "Non zero exit from demodata: {error}"
        );
    }
}
