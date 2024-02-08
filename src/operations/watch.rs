use crate::{direnv, Command};

pub fn admin() -> anyhow::Result<String> {
    direnv!["composer", "watch:admin"]
        .await_success()
        .map(|()| "Watcher stopped".into())
}

pub fn storefront() -> anyhow::Result<String> {
    direnv!["composer", "watch:storefront"]
        .await_success()
        .map(|()| "Watcher stopped".into())
}

pub fn admin_jest() -> anyhow::Result<String> {
    direnv!["composer", "admin:unit:watch"]
        .await_success()
        .map(|()| "Watcher stopped".into())
}

pub fn storefront_jest() -> anyhow::Result<String> {
    direnv!["composer", "storefront:unit:watch"]
        .await_success()
        .map(|()| "Watcher stopped".into())
}
