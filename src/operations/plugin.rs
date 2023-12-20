use colored::Colorize;

use crate::{direnv, topic, Command};

pub fn install(name: &str, no_activation: bool) -> anyhow::Result<String> {
    topic!("Installing plugin matching {}...", name.green());

    let mut cmd = direnv!["bin/console", "plugin:install", "-c", name];

    if !no_activation {
        cmd.arg("-a");
    }

    cmd.await_success()?;

    Ok(format!("Plugin matching {name} installed"))
}

pub fn activate(name: &str) -> anyhow::Result<String> {
    topic!("Activating plugin matching {}...", name.green());

    direnv!["bin/console", "plugin:activate", name].await_success()?;

    Ok(format!("Plugin matching {name} activated"))
}

pub fn uninstall(name: &str) -> anyhow::Result<String> {
    topic!("Uninstalling plugin matching {}...", name.green());

    direnv!["bin/console", "plugin:uninstall", "-c", name].await_success()?;

    Ok(format!("Plugin matching {name} uninstalled"))
}

pub fn reinstall(name: &str) -> anyhow::Result<String> {
    uninstall(name)?;
    install(name, false)?;

    Ok(format!("Plugin matching {name} reinstalled"))
}

pub fn refresh() -> anyhow::Result<String> {
    topic!("Refreshing plugins...");

    direnv!["bin/console", "plugin:refresh"].await_success()?;

    Ok("Plugins refreshed".into())
}

pub fn list() -> anyhow::Result<String> {
    direnv!["bin/console", "plugin:list"].await_success()?;

    Ok(String::new())
}
