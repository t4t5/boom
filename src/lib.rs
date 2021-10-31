use colour::green_ln;
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::process::Command;
use std::{env, fs};

mod init_script;
use init_script::run_init_commands;

mod placeholder_replacer;
use placeholder_replacer::replace_placeholders;

mod utils;
use utils::{error, get_boom_dir, TEMPLATES_DIR};

mod file_copier;
use file_copier::create_project_from_template;

pub fn start() {
    create_boom_folder_if_no_exist();

    let args: Vec<String> = env::args().collect();

    let command = match args.get(1) {
        Some(template_name) => template_name,
        None => {
            error("You need to specify the name of your template or a valid command (new|add) as the first argument!");
            panic!()
        }
    };

    if command == "new" {
        create_new_boilerplate(&args);
    } else if command == "add" {
        add_new_remote_boilerplate(&args);
    } else {
        let template_name = command;
        generate_new_project(template_name, &args);
    }
}

fn create_boom_folder_if_no_exist() {
    if let Err(message) = fs::create_dir_all(get_boom_dir() + TEMPLATES_DIR) {
        error(format!("Error creating .boom directory: {}", &message).as_str());
    }
}

fn create_new_boilerplate(args: &Vec<String>) {
    let template_name = match args.get(2) {
        Some(template_name) => template_name,
        None => {
            error("You need to specify the name of your new template as the second argument!");
            panic!()
        }
    };

    let template_path = get_boom_dir() + TEMPLATES_DIR + template_name;
    let boilerplate_path = template_path.to_owned() + "/boilerplate";

    if let Err(message) = fs::create_dir_all(&boilerplate_path) {
        error(format!("Error creating your new boom template: {}", &message).as_str());
    }

    if let Err(message) = File::create(template_path.to_owned() + "/init.sh") {
        error(
            format!(
                "Error creating init.sh file for your template: {}",
                &message
            )
            .as_str(),
        );
    }

    green_ln!("\n✓ Created new template at {}!", &template_path);
}

fn add_new_remote_boilerplate(args: &Vec<String>) {
    // Expecting a URL like:
    // "https://github.com/t4t5/dotfiles/tree/master/boom/templates/eleventy"
    let github_url = match args.get(2) {
        Some(github_url) => github_url,
        None => {
            error("You need to specify a GitHub URL as the second argument!");
            panic!()
        }
    };

    let last_slash_index = match github_url.rfind('/') {
        Some(last_slash_index) => last_slash_index,
        None => {
            error(format!("Invalid GitHub URL: {}", &github_url).as_str());
            panic!()
        }
    };

    let template_name: String = github_url
        .to_owned()
        .chars()
        .skip(&last_slash_index.to_owned() + 1)
        .take(&github_url.chars().count() - last_slash_index)
        .collect();

    let loading_msg = "Fetching template from ".to_owned() + &github_url.to_owned();
    let sp = Spinner::new(&Spinners::BouncingBar, loading_msg);

    let templates_path = get_boom_dir() + TEMPLATES_DIR;

    assert!(env::set_current_dir(&templates_path).is_ok());

    if let Err(message) = Command::new("ghclone").args([github_url]).output() {
        error(format!("{}", &message).as_str());
        panic!()
    }

    sp.stop();
    green_ln!("\n✓ Added new boom template: {}", &template_name);
}

const SKIP_INIT_ARG: &str = "--skip-init";

fn generate_new_project(template_name: &String, args: &Vec<String>) {
    let proj_name = match args.get(2) {
        Some(proj_name) => proj_name,
        None => {
            error("You need to specify the name of your project as the second argument!");
            panic!()
        }
    };

    let options: Vec<String> = args[3..].to_vec();

    let (template_path, dest_path) = create_project_from_template(template_name, &proj_name);

    replace_placeholders(&dest_path, &proj_name);

    let skip_init = String::from(SKIP_INIT_ARG);

    if !options.contains(&skip_init) {
        run_init_commands(&template_path, &dest_path);
    }
}
