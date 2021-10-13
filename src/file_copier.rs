use colour::green_ln;
use std::path::Path;
use std::{env, fs, io};

use crate::utils::{error, get_boom_dir, BOILERPLATE_DIR, TEMPLATES_DIR};

pub fn create_project_from_template(
    template_name: &String,
    proj_name: &String,
) -> (String, String) {
    let template_path = get_boom_dir() + TEMPLATES_DIR + template_name;
    let boilerplate_path = template_path.to_owned() + BOILERPLATE_DIR;
    let dest_path = env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + proj_name;

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

    green_ln!("✓ Created project: {}", &dest_path);

    (template_path, dest_path)
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
