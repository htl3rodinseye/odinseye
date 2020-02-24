use std::io;
use std::io::{Error, Write};
use std::process::Command;

mod cmd_lib;
mod rest;

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

    let mut prompt; // This is the prompt that will be displayed

    let mut history: Vec<String> = Vec::new();

    loop {

        prompt = crate::fs::env::get_working_dir();
        prompt.push_str("> ");

        let mut input = String::new(); // Instantiate a new String to contain the input from the user

        write!(io::stdout().lock(), "{}", prompt)?; // Writes the prompt to stdout
        io::stdout().flush()?; // Flushes stdout so the prompt is displayed before stdin is read

        // Matches the Result of readin from stdin
        match io::stdin().read_line(&mut input) {
            // In case the Result returns Ok() execute the corresponding action
            Ok(_n) => {
                let mut cmd = input.clone();
                cmd.pop();

                match cmd.as_str() {
                    "exit" => {
                        break;
                    },
                    "fetch" => {
                        let text = rest::fetch_text_sync("http://localhost:5000/exercise.json").unwrap_or(String::new());
                        println!("{}", text);
                    },
                    "history" => { println!("{:?}", history); },
                    _ => { cmd_lib::run_cmd(&cmd)?; }
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
