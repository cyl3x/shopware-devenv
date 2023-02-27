use crate::internal::AppExitCode;
use crate::{devenv, fail, success};

pub fn admin() {
    if let Err(error) = devenv!("composer watch:admin")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        fail!(
            AppExitCode::DevenvExec,
            "Non zero exit from watcher: {error}"
        );
    } else {
        success!("Watcher stopped");
    }
}

pub fn storefront() {
    if let Err(error) = devenv!("composer watch:storefront")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        fail!(
            AppExitCode::DevenvExec,
            "Non zero exit from watcher: {error}"
        );
    } else {
        success!("Watcher stopped");
    }
}

pub fn admin_jest() {
    if let Err(error) = devenv!("composer admin:unit:watch")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        fail!(
            AppExitCode::DevenvExec,
            "Non zero exit from watcher: {error}"
        );
    } else {
        success!("Watcher stopped");
    }
}

pub fn storefront_jest() {
    if let Err(error) = devenv!("composer storefront:unit:watch")
        .spawn()
        .expect("Cannot spawn cmd, is devenv ok?")
        .wait()
    {
        fail!(
            AppExitCode::DevenvExec,
            "Non zero exit from watcher: {error}"
        );
    } else {
        success!("Watcher stopped");
    }
}
