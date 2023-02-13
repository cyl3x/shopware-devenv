use crate::internal::AppExitCode;
use crate::{crash, devenv};

pub fn platform(verbose: bool, demodata: bool, build_test_db: bool) {
    let status = devenv!(verbose, "composer setup").spawn().unwrap().wait();

    if status.is_err() || !status.unwrap().success() {
        crash!(AppExitCode::BuildPlatformError, "Setup failed");
    }

    if build_test_db {
        test_db(verbose);
    }

    if demodata {
        let _result = devenv!(verbose, "bin/console framework:demodata")
            .env("APP_ENV", "prod")
            .spawn()
            .expect("Cannot execute command to generate demodata")
            .wait();
    }
}

pub fn test_db(verbose: bool) {
    let _result = devenv!(verbose, "composer init:testdb")
        .spawn()
        .unwrap()
        .wait();
}

pub fn admin(verbose: bool) {
    let _result = devenv!(verbose, "composer build:js:admin")
        .spawn()
        .unwrap()
        .wait();
}

pub fn storefront(verbose: bool) {
    let _result = devenv!(verbose, "composer build:js:storefront")
        .spawn()
        .unwrap()
        .wait();
}
