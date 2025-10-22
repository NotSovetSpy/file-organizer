use std::path::PathBuf;

pub fn get_test_path() -> PathBuf {
    let project_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(project_dir).join("tests/")
}

pub fn get_path_under_tests(path: &str) -> PathBuf {
    get_test_path().join(path)
}
