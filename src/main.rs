use colour::{red, yellow};
use dirs::home_dir;
use spinners::{Spinner, Spinners};
use std::path::Path;
use std::process::{exit, Command};
use std::{env, fs, io};
use substring::Substring;

const TEMPLATES_DIR: &str = "/templates/";
const BOILERPLATE_DIR: &str = "/boilerplate/";
const INIT_COMMAND_FILE: &str = "/init.sh";

fn main() {
    let args: Vec<String> = env::args().collect();
    let boilerplate = &args[1];

    create_boom_folder_if_no_exist();
    create_project_from_template(&boilerplate);
}

fn create_boom_folder_if_no_exist() {
    if let Err(message) = fs::create_dir_all(get_boom_dir() + TEMPLATES_DIR) {
        error(format!("Error creating .boom directory: {}", &message).as_str());
    }
}

fn get_boom_dir() -> String {
    home_dir().unwrap().into_os_string().into_string().unwrap() + "/.boom"
}

fn create_project_from_template(template_name: &String) {
    let template_path = get_boom_dir() + TEMPLATES_DIR + template_name;
    let boilerplate_path = template_path.to_owned() + BOILERPLATE_DIR;
    let dest_path = env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + template_name;

    if !Path::new(&template_path).exists() {
        error(
            format!(
                "There's no template called \"{}\" in your .boom directory!",
                template_name
            )
            .as_str(),
        );
    }

    if Path::new(&boilerplate_path).exists() {
        if let Err(message) = copy_dir_all(&boilerplate_path, &dest_path) {
            error(format!("Error creating boilerplate: {}", message).as_str())
        }
    }

    println!("✓ Created project: {}", &dest_path);

    run_init_commands(&template_path, &dest_path);
}

fn run_init_commands(template_path: &str, dest_path: &str) {
    assert!(env::set_current_dir(&dest_path).is_ok());

    println!(
        "changed directory {:?}",
        env::current_dir().unwrap().to_str().unwrap()
    );

    let commands_str = fs::read_to_string(template_path.to_owned() + INIT_COMMAND_FILE)
        .expect("Unable to read file");

    let commands = commands_str.split(";").filter(|&x| !x.trim().is_empty());

    for command in commands {
        let sp = Spinner::new(&Spinners::BouncingBar, command.to_owned().into());

        let index = command.find(" ").unwrap();
        let base_cmd = &command.substring(0, index);
        let args: Vec<&str> = command
            .substring(index, &command.len() + 0)
            .split(" ")
            .filter(|&x| !x.trim().is_empty())
            .collect();

        println!("{}", base_cmd);
        println!("{:?}", args);

        if let Err(message) = Command::new(base_cmd).args(args).output() {
            yellow!("Error running command {}", &message);
        }

        sp.stop();
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

fn error(msg: &str) {
    red!(msg);
    exit(1);
}
