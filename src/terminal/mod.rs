use ansi_term::Colour::*;
use std::io;
use std::io::{Error, Write};
use std::process::Command;

use crate::rest;
//use crate::exercise;

mod cmd_lib;

/**
 * Clears stdout
 */
fn clear_stdout() -> Result<(), Error> {
    let output = Command::new("clear") // Create a new Command-object containing the clear-statement
        .output() // Get the output of executing the defined command
        .expect("failed to execute process"); // In case of an error catch the message "failed to execute process"

    io::stdout().write_all(&output.stdout)?; // Write the data that was written to stdout by the execution of the command previously to stdout
    io::stderr().write_all(&output.stderr)?; // Write the data that was written to stderr by the execution of the command previously to stderr

    io::stdout().flush()?; // Flush stdout

    Ok(())
}

/**
 * This is the main function to run the terminal
 * It will return an IO Error in case of a problem writing to stdout or reading from stdin
 */
pub fn terminal() -> Result<(), Error> {
    clear_stdout()?;

    let mut history: Vec<String> = Vec::new();

    loop {
        let mut prompt = String::new(); // This is the prompt that will be displayed

        let pwd = crate::fs::env::get_working_dir();
        let dirs = pwd.split("/").collect::<Vec<&str>>();

        prompt.push_str(&Blue.bold().paint(dirs[dirs.len() - 1]).to_string());

        prompt.push_str(&Green.bold().paint(" $ ").to_string());

        write!(io::stdout().lock(), "{}", prompt)?; // Writes the prompt to stdout
        io::stdout().flush()?; // Flushes stdout so the prompt is displayed before stdin is read

        let mut input = String::new(); // Instantiate a new String to contain the input from the user

        // Matches the Result of read_line from stdin
        match io::stdin().read_line(&mut input) {
            // In case the Result returns Ok() execute the corresponding action
            Ok(_n) => {
                let mut cmd = input.clone();
                cmd.pop();

                match cmd.as_str() {
                    "exit" => {
                        break;
                    }
                    "fetch" => {
                        let text = rest::fetch_text_sync("http://localhost:5000/exercise.json")
                            .unwrap_or(String::new());
                        println!("{}", text);
                    }
                    "history" => {
                        println!("{:?}", history);
                    }
                    "odin info" => {
                        // =================== HEADER ========================
                        let mut header = String::new();
                        header.push_str("   ____      ___      _          ______         \n");
                        header.push_str("  / __ \\____/ (_)___ ( )_____   / ____/_  _____ \n");
                        header.push_str(" / / / / __  / / __ \\|// ___/  / __/ / / / / _ \\\n");
                        header.push_str("/ /_/ / /_/ / / / / / (__  )  / /___/ /_/ /  __/\n");
                        header.push_str("\\____/\\__,_/_/_/ /_/ /____/  /_____/\\__, /\\___/ \n");
                        header.push_str("                                   /____/       ");
                        println!("{}", Yellow.bold().paint(header));
                    }
                    _ => {
                        cmd_lib::run_cmd(&cmd)?;
                    }
                }

                if cmd != "" {
                    history.push(cmd.clone());
                }
            }
            Err(error) => println!("error: {}", error), // In case the Result returns Err() print the error
        }
    }

    Ok(())
}
