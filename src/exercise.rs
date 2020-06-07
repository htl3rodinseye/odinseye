pub struct Exercise {
    pub id: i32,
    pub group_name: String,
    pub name: String,
    pub description: String,
    pub command_list: Vec<String>,
    pub status: bool,
    pub root: String,
    pub structure: String,
}

pub fn build_exercise(json: json::JsonValue) -> Exercise {

    let mut cmds: Vec<String> = Vec::new();
    for cmd in json["command_list"].members() {
        cmds.push(String::from(cmd.as_str().unwrap_or("")));
    }

    let exercise: Exercise = Exercise {
        id: json["id"].as_i32().unwrap_or(0),
        group_name: String::from(json["group_name"].as_str().unwrap_or("")),
        name: String::from(json["name"].as_str().unwrap_or("")),
        description: String::from(json["description"].as_str().unwrap_or("")),
        command_list: cmds,
        status: json["status"].as_bool().unwrap_or(false),
        root: String::from(json["root"].as_str().unwrap_or("")),
        structure: String::from(json["structure"].as_str().unwrap_or("")),
    };

    // if &json["status"] == "0" {
    //     let home_dir = &json["root"];

    //     //fs::create_dir_structure(home_dir, );
    // }

    println!("{}", json);

    return exercise;
}
