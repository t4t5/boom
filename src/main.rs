use dirs::home_dir;
use std::path::Path;
use std::process::Command;
use std::{env, fs, io};

static TEMPLATES_DIR: &'static str = "/templates/";
static BOILERPLATE_DIR: &'static str = "/boilerplate/";

fn main() {
    let args: Vec<String> = env::args().collect();
    let boilerplate = &args[1];

    create_boom_folder_if_no_exist();
    create_project_from_template(&boilerplate);
}

fn create_boom_folder_if_no_exist() {
    if let Err(message) = fs::create_dir_all(get_boom_dir() + TEMPLATES_DIR) {
        println!("Error creating .boom directory: {}", &message);
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
        panic!(
            "There's no template called {} in your .boom directory!",
            template_name
        );
    }

    if Path::new(&boilerplate_path).exists() {
        if let Err(message) = copy_dir_all(&boilerplate_path, &dest_path) {
            println!("Error creating boilerplate: {}", message);
        }
    }

    println!("✓ Created project: {}", &dest_path);

    assert!(env::set_current_dir(&dest_path).is_ok());

    println!(
        "changed directory {:?}",
        env::current_dir().unwrap().to_str().unwrap()
    );

    let output = Command::new("npm.cmd")
        .arg("install")
        .output()
        .expect("Failed to run npm install");

    println!("Output {:?}", &output.stdout.as_slice());
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
