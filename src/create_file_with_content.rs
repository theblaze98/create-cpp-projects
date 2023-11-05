use std::fs;

pub fn create_file_with_content(base_path: &str, file_name: &str, content: &str) {
    let full_path = format!("{}/{}", base_path, file_name);
    fs::write(full_path, content).unwrap();
}
