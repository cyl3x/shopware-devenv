use crate::direnv;
use crate::internal::AppCommand;

pub fn main(args: &[String]) {
    direnv!["bin/console"].args(args).start_await_success();
}
