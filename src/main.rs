mod fs;
mod terminal;

/**
 * This is the main method for the program
 */
fn main() {
    let mut exercise_path = String::from("http://localhost:5000/"); // Create a variable containing the url from which to fetch the exercise JSON
    exercise_path.push_str("exercise.json");

    // Create a variable containing the directory-structure to generate
    let fs_dirs: Vec<&str> = [
        "bin", "boot", "dev", "etc", "home", "lib", "lib64", "mnt", "opt", "proc", "root", "run",
        "sbin", "srv", "sys", "tmp", "usr", "var",
    ]
    .to_vec();

    fs::create_fs(String::from(""), fs_dirs).unwrap_or(()); // Create the filesystem
    //println!("{}", content);

    terminal::terminal().unwrap_or(()); // Initialize the terminal
}
