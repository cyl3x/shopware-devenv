use crate::{direnv, Command};

pub fn main(args: &[String]) {
    direnv!["bin/console"].args(args).await_success();
}
