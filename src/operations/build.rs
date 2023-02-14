use crate::config::Config;
use crate::internal::AppExitCode;
use crate::{crash, devenv};

pub fn platform(config: &Config, demodata: bool, build_test_db: bool) {
    if let Err(error) = devenv!(config, "composer setup")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(AppExitCode::DevenvExec, "Non zero exit for setup: {error}");
    }

    if build_test_db {
        test_db(config);
    }

    if demodata {
        if let Err(error) = devenv!(config, "bin/console framework:demodata")
            .env("APP_ENV", "prod")
            .spawn()
            .expect("Cannot spawn cmd, is devenv ok?")
            .wait()
        {
            crash!(
                AppExitCode::DevenvExec,
                "Non zero exit from demodata: {error}"
            );
        }

        if let Err(error) = devenv!(config, "bin/console dal:refresh:index")
            .env("APP_ENV", "prod")
            .spawn()
            .expect("Cannot spawn cmd, is devenv ok?")
            .wait()
        {
            crash!(
                AppExitCode::DevenvExec,
                "Non zero exit from demodata: {error}"
            );
        }
    }

    // TODO - Add additional URL https://platform.dev.localhost:4000
}

pub fn test_db(config: &Config) {
    // TODO-6.4 FORCE_INSTALL=true vendor/bin/phpunit --group=none
    if let Err(error) = devenv!(config, "composer init:testdb")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    }
}

pub fn admin(config: &Config) {
    if let Err(error) = devenv!(config, "composer build:js:admin")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    }
}

pub fn storefront(config: &Config) {
    if let Err(error) = devenv!(config, "composer build:js:storefront")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    }
}
