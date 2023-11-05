mod generate_launch_json_content;
mod generate_task_json_content;
mod create_cpp_project_structure;
mod create_directory;
mod create_file_with_content;
mod generate_main_cpp_content;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso: {} <nombre_del_proyecto>", args[0]);
        return;
    }

    if args[1] == "new" {
        let project_name = &args[2];
        create_cpp_project_structure::create_cpp_project_structure(project_name);
        return;
    }
}
