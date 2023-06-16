use colored::Colorize;

use crate::internal::AppExitCode;
use crate::{direnv, fail, spinner, success};

pub fn install(name: &str, no_activation: bool) {
    let mut cmd = direnv!["bin/console", "plugin:install", "-rc", name];

    if !no_activation {
        cmd.arg("-a");
    }

    if let Err(error) = cmd.spawn().expect("Cannot start bin/console").wait() {
        fail!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    } else {
        success!("Plugin matching {} installed", name.green());
    }
}

pub fn uninstall(name: &str) {
    let mut cmd = direnv!["bin/console", "plugin:uninstall", "-c", name];

    if let Err(error) = cmd.spawn().expect("Cannot start bin/console").wait() {
        fail!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    } else {
        success!("Plugin matching {} uninstalled", name.green());
    }
}

pub fn reinstall(name: &str) {
    uninstall(name);
    install(name, false);
}

pub fn refresh() {
    spinner!("Refreshing plugins...");

    if let Err(error) = direnv!["bin/console", "plugin:refresh", "-sq"]
        .spawn()
        .expect("Cannot start bin/console")
        .wait_with_output()
    {
        fail!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    } else {
        success!("Plugins refreshed");
    }
}

pub fn list() {
    if let Err(error) = direnv!["bin/console", "plugin:list"]
        .spawn()
        .expect("Cannot start bin/console")
        .wait()
    {
        fail!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    }
}
