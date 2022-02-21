fn main() {
    // TODO: write my build utils to create a zip archive.
    rust_competitive_helper_util::build::build_several_libraries(&vec![
        "algo_lib".to_owned(),
        "marathon_utils".to_owned(),
    ]);
}
