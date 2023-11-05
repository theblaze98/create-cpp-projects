pub fn generate_main_cpp_content(project_name: &str) -> String {
    format!(r#"
    #include <iostream>

        int main() {{
            std::cout << "Hello, {}!" << std::endl;
            return 0;
        }}
    "#, project_name)
}
