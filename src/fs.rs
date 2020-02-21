use std::env::var_os;
use std::fs::create_dir;
use std::io::{Error};
use std::path::Path;

/**
 * Returns a String Object containing the current users home-directory path
 */
fn get_home_dir() -> String {
    let mut home_dir: String = String::new();                                   // The path to the users home-directory will be stored here
    match var_os("HOME") {                                                      // Match the environment variable $HOME
        Some(home_dir_os) => {                                                  // If the match-statement returns a $HOME variable as OsString
            home_dir = home_dir_os.into_string().unwrap_or(String::new());      // Try to convert the OsString to a normal String and assign it to the home_dir variable
        }
        None => println!("{} is not defined in the environment.", home_dir),    // If the $HOME variable is not set print an error message
    }
    return home_dir;                                                            // Either return an empty String or the path behind the $Home environment variable
}

/**
 * Returns and creates the path to the directory the application will use
 * This path is always relative to the users HOME-directory
 */
fn generate_program_path(root: String) -> String {
    let mut program_dir: String = get_home_dir();                               // Save the current users $HOME env-variable in program_dir
    program_dir.push_str(&root);                                                // Append the provided path behind the root variable to the users home-directory path
    program_dir.push_str("/OdinsEye/");                                         // Append /OdinsEye/ to the previously created path. This will be used as the root dir of the simulated filestructure

    if !Path::new(&program_dir).exists() {                                      // If the Program path does not already exist create it
        create_dir(&program_dir).unwrap();
    //println!("{}", program_dir);
    } else {                                                                    // Else print an error message
        println!("Directory already exists!");
    }

    return String::from(program_dir);                                           // Return the created path
}

/**
 * Creates the directory-structure provided by the parameter "fs"
 */
pub fn create_fs(root: String, fs: Vec<&str>) -> Result<(), Error> {            
    let env_dir: String = generate_program_path(root);                          // Generate the program path and return it as String

    if env_dir != "" {                                                          // If the directory creation succeeded 
        for dir in fs {                                                         // Create the directory structure provided as vector provided by the second parameter
            let mut env_dir_cp = String::new();
            env_dir_cp.push_str(&env_dir);
            let mut path = String::new();
            path.push_str(&env_dir_cp);
            path.push_str(dir);
            create_dir(path)?;
        }
    } else {                                                                    // Else print an error message
        println!("Filesystem generation not possible!");                        
    }

    Ok(())
}