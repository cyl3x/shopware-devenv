use crate::internal::AppExitCode;
use crate::{devenv, fail, success};

pub fn platform(gen_demodata: bool, build_test_db: bool) {
    if let Err(error) = devenv!("composer setup")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        fail!(AppExitCode::DevenvExec, "Non zero exit for setup: {error}");
    }

    if gen_demodata {
        demodata();
    }

    if build_test_db {
        test_db();
    }

    success!("Build successfull");
    // TODO - Add additional URL https://platform.dev.localhost:4000
}

pub fn test_db() {
    if let Err(error) = devenv!("composer init:testdb")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        fail!(AppExitCode::DevenvExec, "Non zero exit: {error}\nIf you're using platform 6.4 you probably have to do it manually");
    } else {
        success!("Build successfull");
    }
}

pub fn admin() {
    if let Err(error) = devenv!("composer build:js:admin")
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
    if let Err(error) = devenv!("composer build:js:storefront")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        fail!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    } else {
        success!("Build successfull");
    }
}

pub fn demodata() {
    if let Err(error) = devenv!("bin/console framework:demodata")
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

    if let Err(error) = devenv!("bin/console dal:refresh:index")
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
