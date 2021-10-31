// list placeholders to look for
// find all files where they occur
// replace in each file

use std::fs::{self, OpenOptions};
use std::io::{self, prelude::*};

use crate::utils::error;

pub fn replace_placeholders(dest_path: &str, proj_name: &str) {
    let placeholder_replacements = [["__name__", &proj_name]];

    let file_paths = [
        dest_path.to_owned() + "/package.json",
        dest_path.to_owned() + "/public/CNAME",
    ];

    for [placeholder, replacement] in placeholder_replacements {
        for file_path in &file_paths {
            if let Err(_) = replace_in_file(&file_path, &placeholder, &replacement) {
                error(
                    format!(
                        "Couldn't replace placeholder '{}' in file '{}'",
                        &placeholder, &file_path
                    )
                    .as_str(),
                );
            }
        }
    }
}

fn replace_in_file(file_path: &str, placeholder: &str, replacement: &str) -> Result<(), io::Error> {
    let content = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(_) => {
            error(format!("Couldn't open the file {}", &file_path).as_str());
            panic!()
        }
    };

    let content = content.replace(&placeholder, &replacement);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path)?;

    file.write(content.as_bytes())?;

    Ok(())
}
