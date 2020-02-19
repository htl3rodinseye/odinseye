use std::env::var_os;
use std::fs::create_dir;
use std::path::Path;

fn create_program_env(root: &str) {
    let mut program_dir = String::from(root);
    let home_dir = String::from(root);

    match var_os(program_dir) {
        Some(program_dir_os) => {
            program_dir = program_dir_os.into_string().unwrap();
            program_dir.push_str("/OdinsEye/");
            if !Path::new(&program_dir).exists() {
                create_dir(&program_dir).unwrap();
            } else {
                println!("Directory already exists!");
            }
        }
        None => println!("{} is not defined in the environment.", home_dir),
    }
}

fn main() {
    create_program_env("HOME");
}
