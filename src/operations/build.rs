use crate::internal::AppExitCode;
use crate::{crash, devenv, finish};

pub fn platform(gen_demodata: bool, build_test_db: bool) {
    if let Err(error) = devenv!("composer setup")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(AppExitCode::DevenvExec, "Non zero exit for setup: {error}");
    }

    if build_test_db {
        test_db();
    }

    if gen_demodata {
        demodata();
    }

    finish!("Build successfull");
    // TODO - Add additional URL https://platform.dev.localhost:4000
}

pub fn test_db() {
    // TODO-6.4 FORCE_INSTALL=true vendor/bin/phpunit --group=none
    if let Err(error) = devenv!("composer init:testdb")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    } else {
        finish!("Build successfull");
    }
}

pub fn admin() {
    if let Err(error) = devenv!("composer build:js:admin")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    } else {
        finish!("Build successfull");
    }
}

pub fn storefront() {
    if let Err(error) = devenv!("composer build:js:storefront")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    } else {
        finish!("Build successfull");
    }
}

pub fn demodata() {
    if let Err(error) = devenv!("bin/console framework:demodata")
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

    if let Err(error) = devenv!("bin/console dal:refresh:index")
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
