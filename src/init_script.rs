use colour::{green_ln, yellow_ln};
use spinner::SpinnerBuilder;
use std::process::Command;
use std::{env, fs};

const INIT_COMMAND_FILE: &str = "/init.sh";

pub fn run_init_commands(template_path: &str, dest_path: &str) {
    assert!(env::set_current_dir(&dest_path).is_ok());

    let commands_str = fs::read_to_string(template_path.to_owned() + INIT_COMMAND_FILE)
        .expect("Unable to read file");

    let commands = commands_str.split(";").filter(|&x| !x.trim().is_empty());

    green_ln!("Running initialisation scripts...");

    let sp = SpinnerBuilder::new("Starting...".into()).start();

    for command in commands {
        let command = str::replace(command, "\n", "");
        sp.update(command.to_owned().into());

        let index = &command.find(" ").unwrap();
        let base_cmd = &command[(0 as usize)..index.to_owned()];

        let length = &command.len();
        let args: Vec<&str> = command[index.to_owned()..length.to_owned()]
            .split(" ")
            .filter(|&x| !x.trim().is_empty())
            .collect();

        if let Err(message) = Command::new(base_cmd).args(args).output() {
            yellow_ln!("Error running command {}: {}", &base_cmd, &message);
        }
    }

    green_ln!("\n✓ Done!");
}
