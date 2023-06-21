use colored::Colorize;

use crate::{direnv, success, topic, Command};

pub fn install(name: &str, no_activation: bool) {
    topic!("Installing plugin matching {}...", name.green());

    let mut cmd = direnv!["bin/console", "plugin:install", "-rc", name];

    if !no_activation {
        cmd.arg("-a");
    }

    cmd.await_success();

    success!("Plugin matching {name} installed");
}

pub fn activate(name: &str) {
    topic!("Activating plugin matching {}...", name.green());

    direnv!["bin/console", "plugin:activate", name].await_success();

    success!("Plugin matching {name} activated");
}

pub fn uninstall(name: &str) {
    topic!("Uninstalling plugin matching {}...", name.green());

    direnv!["bin/console", "plugin:uninstall", "-c", name].await_success();

    success!("Plugin matching {name} uninstalled");
}

pub fn reinstall(name: &str) {
    uninstall(name);
    install(name, false);
}

pub fn refresh() {
    topic!("Refreshing plugins...");

    direnv!["bin/console", "plugin:refresh"].await_success();

    success!("Plugins refreshed");
}

pub fn list() {
    direnv!["bin/console", "plugin:list"].await_success();
}
