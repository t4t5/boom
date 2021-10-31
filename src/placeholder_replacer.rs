// list placeholders to look for
// find all files where they occur
// replace in each file

use std::fs::{self, OpenOptions};
use std::io::{self, prelude::*};

use crate::utils::error;
use walkdir::WalkDir;

pub fn replace_placeholders(dest_path: &str, proj_name: &str) {
    let placeholder_replacements = [["__name__", &proj_name]];

    for [placeholder, replacement] in placeholder_replacements {
        check_dir(&dest_path, &placeholder, &replacement);
    }
}

fn check_dir(path: &str, placeholder: &str, replacement: &str) {
    for (_, file) in WalkDir::new(path)
        .into_iter()
        .filter_map(|file| file.ok())
        .enumerate()
    {
        if file.metadata().unwrap().is_file() {
            match fstream::contains(file.path(), &placeholder) {
                Some(b) => {
                    if b {
                        check_file(file.path().to_str().unwrap(), &placeholder, &replacement);
                    }
                }
                None => println!("Error in walking Dir"),
            }
        }
    }
}

fn check_file(file_path: &str, placeholder: &str, replacement: &str) {
    match fstream::read_lines(file_path) {
        Some(lines) => {
            for (_pos, line) in &mut lines.iter().enumerate() {
                if line.contains(placeholder) {
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
        None => println!("Error in reading File"),
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
