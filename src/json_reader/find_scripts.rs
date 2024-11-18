use serde_json::Result;
use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub scripts: HashMap<String, String>
}

pub fn read_values() -> Result<Package> {
    let current_dir = std::env::current_dir().unwrap();

    let data = fs::read_to_string(current_dir.join("package.json")).unwrap();
    
    let json_result: Result<Package> = serde_json::from_str(&data);

    let json = match json_result {
        Ok(content) => content,
        Err(_error) => Package {
            scripts: HashMap::from([
            ("Error parsing json".to_owned(), "There was an error parsing your package.json".to_owned())
        ]),
        }
    };

    Ok(json)
}
