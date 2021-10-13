use colour::red;
use dirs::home_dir;
use std::process::exit;

pub const TEMPLATES_DIR: &str = "/templates/";
pub const BOILERPLATE_DIR: &str = "/boilerplate/";

pub fn get_boom_dir() -> String {
    home_dir().unwrap().into_os_string().into_string().unwrap() + "/.boom"
}

pub fn error(msg: &str) {
    red!(msg);
    exit(1);
}
