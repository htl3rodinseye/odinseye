use std::fs::create_dir;
use std::io::{Error};
use std::path::Path;

pub mod env;

/**
 * Returns and creates the path to the directory the application will use
 * This path is always relative to the users HOME-directory
 */
fn generate_program_path(root: String) -> String {
    let mut program_dir: String = super::fs::env::get_env_var(String::from("HOME"));                               // Save the current users $HOME env-variable in program_dir
    program_dir.push_str(&root);                                                // Append the provided path behind the root variable to the users home-directory path
    program_dir.push_str("/OdinsEye/");                                         // Append /OdinsEye/ to the previously created path. This will be used as the root dir of the simulated filestructure

    if !Path::new(&program_dir).exists() {                                      // If the Program path does not already exist create it
        create_dir(&program_dir).unwrap();
    //println!("{}", program_dir);
    } else {                                                                    // Else print an error message
        println!("Directory already exists!");
    }

    return program_dir.clone();                                           // Return the created path
}

/**
 * Creates the directory-structure provided by the parameter "fs"
 */
pub fn create_fs(root: String, fs: Vec<&str>) -> Result<(), Error> {            
    let env_dir: String = generate_program_path(root);                          // Generate the program path and return it as String

    if env_dir != "" {                                                          // If the directory creation succeeded 
        for dir in fs {                                                         // Create the directory structure provided as vector provided by the second parameter
            let mut path = String::new();
            path.push_str(&env_dir.clone());
            path.push_str(dir);
            create_dir(path)?;
        }
    } else {                                                                    // Else print an error message
        println!("Filesystem generation not possible!");                        
    }

    Ok(())
}