use ansi_term::Colour::*;
use json::JsonValue;
use std::collections::HashMap;
use std::io;
use std::io::{Error, Write};
use std::process::Command;
use std::str;

use crate::exercise;
use crate::rest;

use crate::cmd_lib;

/**
 * Clears stdout
 */
fn clear_stdout() -> Result<(), Error> {
    let output = Command::new("clear") // Create a new Command-object containing the clear-statement
        .output() // Get the output of executing the defined command
        .expect("failed to execute process"); // In case of an error catch the error with the message "failed to execute process"

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
    let output = Command::new("id")
        .arg("-u")
        .arg("-n")
        .output()
        .expect("failed to execute process");

    let mut user = String::from(str::from_utf8(&output.stdout).unwrap_or(""));
    user.pop();

    let mut root_dir: String = String::from("/home/");
    root_dir.push_str(&user);
    root_dir.push_str("/OdinsEye/root");

    let mut cmd_string = String::from("cd ");
    cmd_string.push_str(&root_dir);

    cmd_lib::run_cmd(&cmd_string).unwrap_or(());

    clear_stdout()?;

    // println!("{}", str::from_utf8(&user.stdout).unwrap_or(""));

    let mut history: Vec<String> = Vec::new();

    let mut startscreen: String = String::new();
    startscreen.push_str("Welcome to Odin's Eye!\n");
    println!("{}", &Green.bold().paint(&startscreen));
    print!("Get going right away and start a new exercise by typing ");
    print!("{}", &Green.bold().paint("odin exercise "));
    println!("and selecting the first one in the provided list.\n");
    print!("If you need help at any time you can run ");
    print!("{}", &Green.bold().paint("odin help "));
    println!("to get an overview!");

    let mut cl_map: HashMap<String, bool> = HashMap::new();
    // static mut cl_vec: Vec<bool> = Vec::new();
    let mut executed: usize;

    loop {
        let mut prompt = String::new(); // This is the prompt that will be displayed

        let pwd = crate::fs::env::get_working_dir();
        let dirs = pwd.split("/").collect::<Vec<&str>>();

        if dirs[dirs.len() - 1] == "root" && pwd == root_dir {
            prompt.push_str(&Blue.bold().paint("/").to_string());
        } else {
            prompt.push_str(&Blue.bold().paint(dirs[dirs.len() - 1]).to_string());
        }

        prompt.push_str(&Green.bold().paint(" $ ").to_string());

        write!(io::stdout().lock(), "{}", prompt)?; // Writes the prompt to stdout
        io::stdout().flush()?; // Flushes stdout so the prompt is displayed before stdin is read

        let mut input = String::new(); // Instantiate a new String to contain the input from the user

        // let json_cl: JsonValue = JsonValue::Null;

        // Matches the Result of read_line from stdin
        match io::stdin().read_line(&mut input) {
            // In case the Result returns Ok() execute the corresponding action
            Ok(_n) => {
                let mut input_command = input.clone();
                input_command.pop();

                match input_command.as_str() {
                    "exit" => {
                        break;
                    }
                    "history" => {
                        println!("{:?}", history);
                    }
                    "odin info" => {
                        // =================== HEADER ========================
                        let mut header = String::new();
                        header.push_str("Odin's Eye is a simple shell interface, designed to teach people the proper way to interact with a terminal.\n");
                        header.push_str("To get started with using our application you can type \"odin help\" to get an insight into the usage of this interface otherwise you can use \"odin list\" to get a list of all available commands.\n\n");
                        header.push_str("                        -+sso:                        \n");
                        header.push_str("                      -osssssso:                      \n");
                        header.push_str("                    -osssssssssso:                    \n");
                        header.push_str("                  -osssssssssssssso:                  \n");
                        header.push_str("                -osssssssssssssssssso:                \n");
                        header.push_str("              -osssssssssssssssssssssso:              \n");
                        header.push_str("            -osssssssssssssssssssssssssso-            \n");
                        header.push_str("            ossssssssssso/..:ossssssssssss            \n");
                        header.push_str("            ossssssss+:`      `:+sssssssss            \n");
                        header.push_str("            osssss+-`            `-/ssssss            \n");
                        header.push_str("      -+`   oso/.                    ./oss   `+:      \n");
                        header.push_str("    -oss.   ..           `..`           `-   `sso:    \n");
                        header.push_str("  -ossss.             `/ssssss/.             `sssso:  \n");
                        header.push_str("-osssss/             .ssssssssss.             /osssso-\n");
                        header.push_str("sssss/               :ssssssssss/               /sssss\n");
                        header.push_str("-osssss/             .ssssssssss.             /ssssso-\n");
                        header.push_str("  -ossss.             `/ssssss/`             `sssso:  \n");
                        header.push_str("    -oss.   ..           `..`           .-   `sso:    \n");
                        header.push_str("      -+`   oso/.                    ./oss   `+:      \n");
                        header.push_str("            osssss+-`            `-/ssssss            \n");
                        header.push_str("            ossssssss+:`      `:+sssssssss            \n");
                        header.push_str("            ossssssssssso/..:ossssssssssss            \n");
                        header.push_str("            .osssssssssssssssssssssssssso-            \n");
                        header.push_str("              -osssssssssssssssssssssso:              \n");
                        header.push_str("                -osssssssssssssssssso:                \n");
                        header.push_str("                  -osssssssssssssso:                  \n");
                        header.push_str("                    -osssssssssso:                    \n");
                        header.push_str("                      -osssssso:                      \n");
                        header
                            .push_str("                        -+sso:                        \n\n");
                        header.push_str("A project made possible by HTL Rennweg");
                        println!("{}", Yellow.bold().paint(header));
                    }
                    "odin help" => {
                        let mut help: String = String::new();
                        help.push_str("The easiest way to get started is to use the ");
                        help.push_str(&Green.bold().paint("exercise"));
                        help.push_str(" command to select the exercise you want to start with.\n");
                        help.push_str("We structured the exercises in a way that will guide you through the most important uses of terminal commands without requiring any previous knowledge.");
                        println!("{}", help);
                    }
                    "odin list" => {
                        let mut list: String = String::new();
                        list.push_str("====== List of commands ======\n");
                        list.push_str("exit . . . is used to exit the interface\n");
                        list.push_str(
                            "history . . . show your previously entered commands of the session\n",
                        );
                        list.push_str("odin info . . . prints information about the application to the screen\n");
                        list.push_str("odin help . . . provides the user with tips and tricks to aid with the usage of the application\n");
                        list.push_str("odin list . . . gives a list of all available commands provided by Odin's Eye\n");
                        list.push_str("odin exercise . . . will list all available exercises and provide an option to select which exercise to do next\n");
                        println!("{}", list);
                    }
                    "odin exercise" => {
                        let mut id: String = String::new();
                        println!("Which exercise would you like to select?");

                        let all_exercises = json::parse(
                            &rest::fetch_text_sync(
                                "http://caretaker.wurzer.cc:9040/exercises?exid=0",
                            )
                            .unwrap_or(String::new()),
                        )
                        .unwrap_or(json::JsonValue::Null);

                        for ex in all_exercises.members() {
                            let mut items: Vec<&JsonValue> = Vec::new();
                            for ex_item in ex.entries() {
                                items.push(ex_item.1);
                            }
                            writeln!(io::stdout(), "{} ({}): {}", items[0], items[2], items[1])?;
                            io::stdout().flush()?;
                        }

                        write!(io::stdout(), "Choose an exercise: ")?;
                        io::stdout().flush()?;
                        io::stdin().read_line(&mut id).unwrap_or(0);

                        if !id.eq(""){
                            let mut url_ex: String =
                                String::from("http://caretaker.wurzer.cc:9040/exercises?exid=");
                            url_ex.push_str(&id);
                            let json_ex = json::parse(
                                &rest::fetch_text_sync(&url_ex).unwrap_or(String::new()),
                            )
                            .unwrap_or(json::JsonValue::Null);

                            let mut url_cl: String =
                                String::from("http://caretaker.wurzer.cc:9040/exercises?clid=");
                            url_cl.push_str(&id);
                            let json_cl = json::parse(
                                &rest::fetch_text_sync(&url_cl).unwrap_or(String::new()),
                            )
                            .unwrap_or(json::JsonValue::Null);

                            cl_map = HashMap::new();

                            for cmd in json_cl.members() {
                                // let temp_vec = cmd.as_str().unwrap_or("").split(" ").collect::<Vec<&str>>();
                                // let mut copy_vec: Vec<String> = Vec::new();
                                // for e in temp_vec.iter() {
                                //     copy_vec.push(e.replace(" ", ""));
                                // }
                                cl_map.insert(cmd.as_str().unwrap_or("").replace(" ", ""), false);
                                // println!("{}", cmd);
                                // unsafe {
                                //     cl_vec.push(false);
                                // }
                            }

                            let mut url_st: String =
                                String::from("http://caretaker.wurzer.cc:9040/exercises?stid=");
                            url_st.push_str(&id);
                            let _st = &rest::fetch_text_sync(&url_st).unwrap_or(String::new());
                            // let json_st = json::parse(st).unwrap_or(json::JsonValue::Null);

                            print_exercise(exercise::build_exercise(json_ex)).unwrap_or(());
                            // clear_stdout()?;
                            // println!("No exercise was chosen!");

                            // println!("{}", id);
                            // println!("{}", json_cl);
                            // println!("{}", st);
                        } else {
                            clear_stdout()?;
                            println!("{}", &Red.bold().paint("No exercise was chosen!"));
                        }
                    }
                    _ => {
                        if pwd == root_dir && input_command.contains("..") {
                            println!("You are already in the root directory");
                        } else {
                            // let cmd_parts_input = input_command.split(" ").collect::<Vec<&str>>();
                            // let mut cmd_parts_trimmed: Vec<String> = Vec::new();
                            // for part in cmd_parts_input {
                            //     cmd_parts_trimmed.push(part.replace(" ", ""));
                            // }
                            // let mut cmd_parts_list: Vec<Vec<&str>> = Vec::new();

                            let cmd_trimmed = input_command.replace(" ", "");

                            // for cmd in json_cl.members() {
                            //     cmd_parts_list.push(
                            //         cmd.as_str().unwrap_or("").split(" ").collect::<Vec<&str>>(),
                            //     );
                            // }

                            if cl_map.contains_key(&cmd_trimmed) { 
                                cl_map.entry(cmd_trimmed).insert(true);
                            }

                            // for (i, e) in cmd_parts_list.iter().enumerate() {
                            //     if cmd_parts_trimmed.eq(e) {
                            //         unsafe {
                            //             cl_vec.get(i).replace(&true);
                            //         }
                            //     }
                            // }

                            cmd_lib::run_cmd(&input_command)?;
                            // println!("{}: {}", cmd_trimmed);

                            let mut temp = 0;
                            for (_k, v) in cl_map.iter() {
                                if v == &true {
                                    temp += 1;
                                }
                                // println!("{} -> {}", k, v);
                            }
                            // unsafe {
                            //     for b in &cl_vec {
                            //         if *b {
                            //             temp += 1
                            //         }
                            //     }
                            // }
                            executed = temp;
                            if cl_map.len() != 0 && executed == cl_map.len() {
                                // clear_stdout()?;
                                println!("{}", &Green.bold().paint("Exercise completed!"));
                                cl_map.clear();
                            }
                            // unsafe {
                            // if cl_map.len() != 0 && executed == cl_map.len() {
                            //     clear_stdout()?;
                            //     // println!("{}", &Green.bold().paint("Exercise completed!"));
                            //     executed = 0;
                            //     cl_map.clear();
                            // }
                            // }
                            // println!("{}", executed);
                        }
                    }
                }
                if input_command != "" {
                    history.push(input_command.clone());
                }
            }
            Err(error) => println!("error: {}", error), // In case the Result returns Err() print the error
        }
    }
    Ok(())
}

pub fn print_exercise(ex: exercise::Exercise) -> Result<(), Error> {
    clear_stdout()?;
    println!("{}", ex.name);
    println!("===========");
    println!("Category: {}\n", ex.group_name);
    println!("{}", ex.description);
    Ok(())
}
