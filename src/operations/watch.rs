use crate::{direnv, success, Command};

pub fn admin() {
    direnv!["composer", "watch:admin"].await_success();
    success!("Watcher stopped");
}

pub fn storefront() {
    direnv!["composer", "watch:storefront"].await_success();
    success!("Watcher stopped");
}

pub fn admin_jest() {
    direnv!["composer", "admin:unit:watch"].await_success();
    success!("Watcher stopped");
}

pub fn storefront_jest() {
    direnv!["composer", "storefront:unit:watch"].await_success();
    success!("Watcher stopped");
}
