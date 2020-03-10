// use serde_json::*;
use json::*;

//use crate::fs;

struct Exercise {
    
}

pub fn build_exercise(json: json::JsonValue) -> Result<()> {
    let mut exercise_path = String::from("http://localhost:5000/"); // Create a variable containing the url from which to fetch the exercise JSON
    exercise_path.push_str("exercise.json");

    let structure = &json["structure"];

    if &json["done"] == false {
        let _home_dir = &json["root"];

        //fs::create_dir_structure(home_dir, )
    }

    println!("{}", structure);

    Ok(())
}
