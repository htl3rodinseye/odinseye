use std::io;
use std::io::{Error, Write};
use std::process::Command;

/**
 * Clears stdout
 */
fn clear_stdout() -> Result<(), Error> {
    let output = Command::new("clear")                              // Create a new Command-object containing the clear-statement
        .output()                                                   // Get the output of executing the defined command
        .expect("failed to execute process");                       // In case of an error catch the message "failed to execute process"

    io::stdout().write_all(&output.stdout)?;                        // Write the data that was written to stdout by the execution of the command previously to stdout
    io::stderr().write_all(&output.stderr)?;                        // Write the data that was written to stderr by the execution of the command previously to stderr

    io::stdout().flush()?;                                          // Flush stdout

    Ok(())
}

/**
 * This is the main function to run the terminal
 * It will return an IO Error in case of a problem writing to stdout or reading from stdin
 */
pub fn terminal() -> Result<(), Error> {
    let prompt = "odin> ";                                          // This is the prompt that will be displayed 

    let commands: Vec<&str> = ["exit\n", "clear\n"].to_vec();       // This vector contains all available commands

    loop {                                                          // Loop 
        let mut input = String::new();                              // Instantiate a new String to contain the input from the user

        write!(io::stdout().lock(), "{}", prompt)?;                 // Writes the prompt to stdout
        io::stdout().flush()?;                                      // Flushes stdout so the prompt is displayed before stdin is read

        match io::stdin().read_line(&mut input) {                   // Matches the Result of readin from stdin 
            Ok(_n) => {                                             // In case the Result returns Ok() execute the corresponding action
                if &input == commands[0] { break; }
                else if &input == commands[1] { clear_stdout()?; }
                else { print!("{}", &input); }
            }
            Err(error) => println!("error: {}", error),             // In case the Result returns Err() print the error
        }
    }

    Ok(())
}