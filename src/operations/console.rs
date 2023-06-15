use crate::{devenv, fail, AppExitCode};

pub fn main(args: &[String]) {
    if let Err(error) = devenv!("bin/console")
        .args(args)
        .spawn()
        .unwrap_or_else(|_| fail!(AppExitCode::Runtime, "Failed to start bin/console"))
        .wait()
    {
        fail!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    }
}
