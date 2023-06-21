use colored::Colorize;

use crate::{direnv, fail, success, topic, Command};

pub fn platform(gen_demodata: bool, build_test_db: bool) {
    topic!("Updating composer dependencies...");
    direnv!["composer", "update"].await_success();

    topic!("Setting up platform...");
    direnv!["composer", "setup"].await_success();

    if gen_demodata {
        demodata(&[]);
    }

    if build_test_db {
        test_db();
    }

    success!("Build successfull");
}

pub fn test_db() {
    topic!("Setting test database...");
    let exit_code = direnv!["composer", "init:testdb"].r#await();

    if exit_code.success() {
        success!("Build successfull");
    }

    fail!(
        "Non zero exit {exit_code}\n{}",
        "If you're using platform 6.4 you probably have to do it manually".red()
    );
}

pub fn admin() {
    topic!("Building administration...");
    direnv!["composer", "build:js:admin"].await_success();
    success!("Build successfull");
}

pub fn storefront() {
    topic!("Dumping theme configuration...");
    direnv!["bin/console", "theme:dump", "--quiet"].await_success();

    topic!("Compiling theme...");
    direnv!["bin/console", "theme:compile", "--quiet"].await_success();

    topic!("Building storefront...");
    direnv!["composer", "build:js:storefront", "--quiet"].await_success();
    success!("Building successfull...");
}

pub fn demodata(args: &[String]) {
    topic!("Generating demodata");
    direnv!["bin/console", "framework:demodata"]
        .args(args)
        .env("APP_ENV", "prod")
        .await_success();

    topic!("Refreshing search index");
    direnv!["bin/console", "dal:refresh:index"]
        .env("APP_ENV", "prod")
        .await_success();
}
