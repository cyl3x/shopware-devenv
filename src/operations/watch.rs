use crate::{AppCommand, ExitCode, direnv, fail, success};

pub fn admin() {
    if let Err(error) = direnv!["composer", "watch:admin"].start().wait() {
        fail!(ExitCode::DevenvExec, "Non zero exit from watcher: {error}");
    } else {
        success!("Watcher stopped");
    }
}

pub fn storefront() {
    if let Err(error) = direnv!["composer", "watch:storefront"].start().wait() {
        fail!(ExitCode::DevenvExec, "Non zero exit from watcher: {error}");
    } else {
        success!("Watcher stopped");
    }
}

pub fn admin_jest() {
    if let Err(error) = direnv!["composer", "admin:unit:watch"].start().wait() {
        fail!(ExitCode::DevenvExec, "Non zero exit from watcher: {error}");
    } else {
        success!("Watcher stopped");
    }
}

pub fn storefront_jest() {
    if let Err(error) = direnv!["composer", "storefront:unit:watch"].start().wait() {
        fail!(ExitCode::DevenvExec, "Non zero exit from watcher: {error}");
    } else {
        success!("Watcher stopped");
    }
}
