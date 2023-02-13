use crate::internal::AppExitCode;
use crate::{crash, devenv};

pub fn platform(verbose: bool, demodata: bool, build_test_db: bool) {
    if let Err(error) = devenv!(verbose, "composer setup")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(AppExitCode::DevenvExec, "Non zero exit for setup: {error}");
    }

    if build_test_db {
        test_db(verbose);
    }

    if demodata {
        if let Err(error) = devenv!(verbose, "bin/console framework:demodata")
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
}

pub fn test_db(verbose: bool) {
    if let Err(error) = devenv!(verbose, "composer init:testdb")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    }
}

pub fn admin(verbose: bool) {
    if let Err(error) = devenv!(verbose, "composer build:js:admin")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    }
}

pub fn storefront(verbose: bool) {
    if let Err(error) = devenv!(verbose, "composer build:js:storefront")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    }
}
