use crate::{direnv, Command};

pub fn main(args: &[String]) -> anyhow::Result<String> {
    direnv!["bin/console"].args(args).await_success()?;

    Ok(String::new())
}
