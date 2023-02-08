use crate::devenv;

pub fn admin(verbose: bool) {
    let _result = devenv!(verbose, "composer watch:admin")
        .spawn()
        .unwrap()
        .wait();
}

pub fn storefront(verbose: bool) {
    let _result = devenv!(verbose, "composer watch:storefront")
        .spawn()
        .unwrap()
        .wait();
}

pub fn admin_jest(verbose: bool) {
    let _result = devenv!(verbose, "composer admin:unit:watch")
        .spawn()
        .unwrap()
        .wait();
}

pub fn storefront_jest(verbose: bool) {
    let _result = devenv!(verbose, "composer storefront:unit:watch")
        .spawn()
        .unwrap()
        .wait();
}
