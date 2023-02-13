use crate::internal::AppExitCode;
use crate::{crash, devenv};

pub fn admin(verbose: bool) {
    if let Err(error) = devenv!(verbose, "composer watch:admin")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(
            AppExitCode::DevenvExec,
            "Non zero exit from watcher: {error}"
        );
    }
}

pub fn storefront(verbose: bool) {
    if let Err(error) = devenv!(verbose, "composer watch:storefront")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(
            AppExitCode::DevenvExec,
            "Non zero exit from watcher: {error}"
        );
    }
}

pub fn admin_jest(verbose: bool) {
    if let Err(error) = devenv!(verbose, "composer admin:unit:watch")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(
            AppExitCode::DevenvExec,
            "Non zero exit from watcher: {error}"
        );
    }
}

pub fn storefront_jest(verbose: bool) {
    if let Err(error) = devenv!(verbose, "composer storefront:unit:watch")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(
            AppExitCode::DevenvExec,
            "Non zero exit from watcher: {error}"
        );
    }
}
