use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let boilerplate = &args[1];
    let proj_name = &args[2];

    boom::start(&boilerplate, &proj_name)
}
