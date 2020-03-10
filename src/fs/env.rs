use std::env::var_os;
use std::process::Command;
use std::str::from_utf8;

pub fn get_env_var(var_name: String) -> String {
    let mut env_var: String = String::new(); // The path to the users home-directory will be stored here
    match var_os(&var_name) {
        // Match the environment variable $HOME
        Some(var_os) => {
            // If the match-statement returns a $HOME variable as OsString
            env_var = var_os.into_string().unwrap_or(String::new()); // Try to convert the OsString to a normal String and assign it to the home_dir variable
        }
        None => println!("{} is not defined in the environment.", env_var), // If the $HOME variable is not set print an error message
    }
    return env_var;
}

/**
 * Returns a String Object containing the current users home-directory path
 */
#[allow(dead_code)]
pub fn get_home_dir() -> String {
    let mut home_dir: String = String::new(); // The path to the users home-directory will be stored here
    match var_os("HOME") {
        // Match the environment variable $HOME
        Some(home_dir_os) => {
            // If the match-statement returns a $HOME variable as OsString
            home_dir = home_dir_os.into_string().unwrap_or(String::new()); // Try to convert the OsString to a normal String and assign it to the home_dir variable
        }
        None => println!("{} is not defined in the environment.", home_dir), // If the $HOME variable is not set print an error message
    }
    return home_dir; // Either return an empty String or the path behind the $Home environment variable
}

pub fn get_working_dir() -> String {
    let output = Command::new("pwd")
        .output()
        .expect("failed to execute process");

    let mut working_dir = String::from(from_utf8(&output.stdout).unwrap_or(""));
    working_dir.pop();

    //println!("{}", working_dir);

    return working_dir;
}
