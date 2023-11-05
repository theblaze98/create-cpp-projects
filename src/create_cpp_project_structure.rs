use crate::create_directory::create_directory;
use crate::generate_launch_json_content::generate_launch_json_content;
use crate::generate_task_json_content::generate_task_json_content;
use crate::generate_main_cpp_content::generate_main_cpp_content;
use crate::create_file_with_content::create_file_with_content;
use std::fs;
use std::path::Path;


pub fn create_cpp_project_structure(project_name: &str) {
    let project_dir = format!("{}", project_name);

    if Path::new(&project_dir).exists() {
        println!("El directorio {} ya existe.", project_dir);
    } else {
        // Crear el directorio del proyecto
        fs::create_dir(&project_dir).unwrap();

        // Crear directorio src
        create_directory(&project_dir, "src");
        create_directory(&project_dir, ".vscode");

        create_file_with_content(
            format!("{}/src", project_dir).as_str(),
            "main.cpp",
            &generate_main_cpp_content(project_name),
        );
        create_file_with_content(
            format!("{}/.vscode", project_dir).as_str(),
            "launch.json",
            &generate_launch_json_content(),
        );
        create_file_with_content(
            format!("{}/.vscode", project_dir).as_str(),
            "tasks.json",
            &generate_task_json_content(),
        );

        println!("El proyecto de C++ '{}' ha sido creado en el directorio '{}'.", project_name, project_dir);
    }
}
