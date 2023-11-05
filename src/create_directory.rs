use std::fs;

pub fn create_directory(base_path: &str, dir_name: &str) {
    let full_path = format!("{}/{}", base_path, dir_name);
    fs::create_dir(full_path).unwrap();
}
