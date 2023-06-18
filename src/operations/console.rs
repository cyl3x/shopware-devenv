use crate::{direnv, AppCommand};

pub fn main(args: &[String]) {
    direnv!["bin/console"].args(args).start_await_success();
}
