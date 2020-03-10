// use serde_json::*;
use json::*;

pub mod terminal;

mod exercise;
mod fs;
pub mod rest;

/**
 * This is the main method for the program
 */
fn main() {
    // Create a variable containing the directory-structure to generate
    let fs_dirs: Vec<&str> = [
        "bin", "boot", "dev", "etc", "home", "lib", "lib64", "mnt", "opt", "proc", "root", "run",
        "sbin", "srv", "sys", "tmp", "usr", "var",
    ]
    .to_vec();

    fs::create_dir_structure(String::from(""), fs_dirs).unwrap_or(()); // Create the filesystem

    let json = json::parse(&rest::fetch_text_sync("http://localhost:5000/exercise.json").unwrap_or(String::new())).unwrap_or(json::JsonValue::Null);

    //exercise::build_exercise(json).unwrap_or(());
    terminal::terminal().unwrap_or(()); // Spawn the terminal
}
