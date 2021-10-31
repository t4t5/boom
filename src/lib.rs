use std::{env, fs};

mod init_script;
use init_script::run_init_commands;

mod placeholder_replacer;
use placeholder_replacer::replace_placeholders;

mod utils;
use utils::{error, get_boom_dir, TEMPLATES_DIR};

mod file_copier;
use file_copier::create_project_from_template;

const SKIP_INIT_ARG: &str = "--skip-init";

pub fn start() {
    let args: Vec<String> = env::args().collect();

    let template_name = match args.get(1) {
        Some(template_name) => template_name,
        None => {
            error("You need to specify the name of your boilerplate as the first argument!");
            panic!()
        }
    };

    let proj_name = match args.get(2) {
        Some(proj_name) => proj_name,
        None => {
            error("You need to specify the name of your project as the second argument!");
            panic!()
        }
    };

    let options: Vec<String> = args[3..].to_vec();

    create_boom_folder_if_no_exist();

    let (template_path, dest_path) = create_project_from_template(&template_name, &proj_name);

    replace_placeholders(&dest_path, &proj_name);

    let skip_init = String::from(SKIP_INIT_ARG);

    if !options.contains(&skip_init) {
        run_init_commands(&template_path, &dest_path);
    }
}

fn create_boom_folder_if_no_exist() {
    if let Err(message) = fs::create_dir_all(get_boom_dir() + TEMPLATES_DIR) {
        error(format!("Error creating .boom directory: {}", &message).as_str());
    }
}
