mod fs;
mod rest;
mod terminal;

/**
 * This is the main method for the program
 * It runs asynchronisly
 * For this it uses the Tokio framework
 */
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut exercise_path = String::from("http://localhost:5000/");                                     // Create a variable containing the url from which to fetch the exercise JSON
    exercise_path.push_str("exercise.json");

    let fs_dirs: Vec<&str> = [                                                                          // Create a variable containing the directory-structure to generate
        "bin", "boot", "dev", "etc", "home", "lib", "lib64", "mnt", "opt", "proc", "root", "run",
        "sbin", "srv", "sys", "tmp", "usr", "var",
    ]
    .to_vec();

    fs::create_fs(String::from(""), fs_dirs).unwrap_or(());                                             // Create the filesystem

    let _content = &rest::fetch_text(&exercise_path).await?;                                            // Fetch the exercise JSON-file as text
    //let json_content = json::parse(&fetch_text(&exercise_path).await?);

    //println!("{}", content);

    terminal::terminal().unwrap_or(());                                                                 // Initialize the terminal

    Ok(())
}
