use colored::Colorize;

use crate::{direnv, spinner, success, AppCommand};

pub fn install(name: &str, no_activation: bool) {
    let mut cmd = direnv!["bin/console", "plugin:install", "-rc", name];

    if !no_activation {
        cmd.arg("-a");
    }

    cmd.start_await_success();

    success!("Plugin matching {} installed", name.green());
}

pub fn activate(name: &str) {
    direnv!["bin/console", "plugin:activate", name].start_await_success();

    success!("Plugin matching {} installed", name.green());
}

pub fn uninstall(name: &str) {
    direnv!["bin/console", "plugin:uninstall", "-c", name].start_await_success();

    success!("Plugin matching {} uninstalled", name.green());
}

pub fn reinstall(name: &str) {
    uninstall(name);
    install(name, false);
}

pub fn refresh() {
    spinner!("Refreshing plugins...");

    direnv!["bin/console", "plugin:refresh", "-sq"].start_await_success();

    success!("Plugins refreshed");
}

pub fn list() {
    direnv!["bin/console", "plugin:list"].start_await_success();
}
