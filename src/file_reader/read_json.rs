use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{collections::HashMap, fs};

#[derive(Serialize, Deserialize)]
struct ScriptList {
    scripts: HashMap<String, String>,
}

pub fn read_json() -> std::io::Result<HashMap<String, String>> {
    let current_dir = std::env::current_dir()?;

    let data = fs::read_to_string(current_dir.join("package.json"))?;

    let json_result: Result<ScriptList> = serde_json::from_str(&data);

    let json = json_result.unwrap_or_else(|_error| ScriptList {
        scripts: HashMap::from([(
            "Error parsing json".to_owned(),
            "There was an error parsing your package.json".to_owned(),
        )]),
    });

    Ok(json.scripts)
}
