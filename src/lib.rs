use std::fs;

mod init_script;
use init_script::run_init_commands;

mod utils;
use utils::{error, get_boom_dir, TEMPLATES_DIR};

mod file_copier;
use file_copier::create_project_from_template;

pub fn start(boilerplate: &String, proj_name: &String) {
    create_boom_folder_if_no_exist();
    let (template_path, dest_path) = create_project_from_template(&boilerplate, &proj_name);
    run_init_commands(&template_path, &dest_path);
}

fn create_boom_folder_if_no_exist() {
    if let Err(message) = fs::create_dir_all(get_boom_dir() + TEMPLATES_DIR) {
        error(format!("Error creating .boom directory: {}", &message).as_str());
    }
}
