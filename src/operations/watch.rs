use crate::{direnv, Command};

pub fn admin() -> anyhow::Result<String> {
    direnv!["composer", "watch:admin"]
        .await_success()
        .map(|_| "Watcher stopped".into())
}

pub fn storefront() -> anyhow::Result<String> {
    let _ = direnv!["fix-storefront-proxy"]
        .spawn()
        .and_then(|mut s| s.wait());
    
    direnv!["composer", "watch:storefront"]
        .await_success()
        .map(|_| "Watcher stopped".into())
}

pub fn admin_jest() -> anyhow::Result<String> {
    direnv!["composer", "admin:unit:watch"]
        .await_success()
        .map(|_| "Watcher stopped".into())
}

pub fn storefront_jest() -> anyhow::Result<String> {
    direnv!["composer", "storefront:unit:watch"]
        .await_success()
        .map(|_| "Watcher stopped".into())
}
