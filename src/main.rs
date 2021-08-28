use dirs::home_dir;
use std::path::Path;
use std::{env, fs, io};

static TEMPLATES_DIR: &'static str = "/templates/";

fn main() {
    let args: Vec<String> = env::args().collect();
    let boilerplate = &args[1];

    create_boom_folder_if_no_exist();
    create_project_from_boilerplate(&boilerplate);
}

fn create_boom_folder_if_no_exist() {
    if let Err(message) = fs::create_dir_all(get_boom_dir() + TEMPLATES_DIR) {
        println!("Error creating .boom directory: {}", &message);
    }
}

fn get_boom_dir() -> String {
    home_dir().unwrap().into_os_string().into_string().unwrap() + "/.boom"
}

fn create_project_from_boilerplate(boilerplate: &String) {
    let boilerplate_path = get_boom_dir() + TEMPLATES_DIR + boilerplate;
    let dest_path = env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + boilerplate;

    if let Err(message) = copy_dir_all(boilerplate_path, dest_path) {
        println!("Error creating boilerplate: {}", message);
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
