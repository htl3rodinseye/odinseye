// use serde_json::*;
use json::*;

//use crate::fs;

enum FileType {
    Directory((String, Vec<FileType>)),
    File((String, String)),
}

struct Exercise {
    id: i32,
    group_name: String,
    name: String,
    description: String,
    command_list: Vec<String>,
    status: bool,
    root: String,
    structure: Vec<(String, JsonValue)>,
}

pub fn build_exercise(mut json: json::JsonValue) -> Result<()> {

    let mut cmds: Vec<String> = Vec::new();
    for cmd in json["command_list"].members() {
        cmds.push(String::from(cmd.as_str().unwrap_or("")));
    }

    // let exercise: Exercise = Exercise {
    //     id: json["id"].as_i32().unwrap_or(0),
    //     group_name: String::from(json["group_name"].as_str().unwrap_or("")),
    //     name: String::from(json["name"].as_str().unwrap_or("")),
    //     description: String::from(json["description"].as_str().unwrap_or("")),
    //     command_list: cmds,
    //     status: json["status"].as_bool().unwrap_or(false),
    //     root: String::from(json["root"].as_str().unwrap_or("")),
    //     structure: structure,
    // };

    if &json["status"] == false {
        let _home_dir = &json["root"];

        //fs::create_dir_structure(home_dir, )
    }

    println!("{}", json);

    Ok(())
}
