use crate::{devenv, fail, AppExitCode};

pub fn main(args: &[String]) {
    if let Err(error) = devenv!("bin/console")
        .args(args)
        .spawn()
        .expect("Cannot start bin/console")
        .wait()
    {
        fail!(AppExitCode::DevenvExec, "Non zero exit: {error}");
    }
}
