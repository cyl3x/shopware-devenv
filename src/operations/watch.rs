use crate::config::Config;
use crate::internal::AppExitCode;
use crate::{crash, devenv, finish};

pub fn admin(config: &Config) {
    if let Err(error) = devenv!(config, "composer watch:admin")
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

pub fn storefront(config: &Config) {
    if let Err(error) = devenv!(config, "composer watch:storefront")
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

pub fn admin_jest(config: &Config) {
    if let Err(error) = devenv!(config, "composer admin:unit:watch")
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

pub fn storefront_jest(config: &Config) {
    if let Err(error) = devenv!(config, "composer storefront:unit:watch")
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
