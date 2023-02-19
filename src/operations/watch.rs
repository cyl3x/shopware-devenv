use crate::internal::AppExitCode;
use crate::{crash, devenv, finish};

pub fn admin() {
    if let Err(error) = devenv!("composer watch:admin")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(
            AppExitCode::DevenvExec,
            "Non zero exit from watcher: {error}"
        );
    } else {
        finish!("Watcher stopped");
    }
}

pub fn storefront() {
    if let Err(error) = devenv!("composer watch:storefront")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(
            AppExitCode::DevenvExec,
            "Non zero exit from watcher: {error}"
        );
    } else {
        finish!("Watcher stopped");
    }
}

pub fn admin_jest() {
    if let Err(error) = devenv!("composer admin:unit:watch")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(
            AppExitCode::DevenvExec,
            "Non zero exit from watcher: {error}"
        );
    } else {
        finish!("Watcher stopped");
    }
}

pub fn storefront_jest() {
    if let Err(error) = devenv!("composer storefront:unit:watch")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        crash!(
            AppExitCode::DevenvExec,
            "Non zero exit from watcher: {error}"
        );
    } else {
        finish!("Watcher stopped");
    }
}
