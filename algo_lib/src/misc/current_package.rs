use std::env;

pub fn get_current_package_name() -> String {
    env::var("CARGO_PKG_NAME").expect("Can't determine package name")
}
