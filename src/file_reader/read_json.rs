use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{collections::HashMap, fs};

#[derive(Serialize, Deserialize)]
struct ScriptList {
    scripts: HashMap<String, String>,
}

pub fn read_scripts(filename: &str) -> std::io::Result<HashMap<String, String>> {
    let current_dir = std::env::current_dir()?;

    let data = fs::read_to_string(current_dir.join(filename));

    let result = data.unwrap_or_else(|_d| {
        return r#"
        { 
            "scripts": {
                "file_not_found": "package.json could not be found in this directory"
            }
        }"#
        .to_string();
    });

    let json_result: Result<ScriptList> = serde_json::from_str(&result);

    let json = json_result.unwrap_or_else(|_error| ScriptList {
        scripts: HashMap::from([(
            "Error parsing json".to_owned(),
            "There was an error parsing your package.json".to_owned(),
        )]),
    });

    Ok(json.scripts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_no_scripts_returns_error() {
        let scripts = read_scripts("src/file_reader/test_data/no_scripts.json").unwrap();

        let expected = HashMap::from([(
            "Error parsing json".to_string(),
            "There was an error parsing your package.json".to_string(),
        )]);

        assert_eq!(scripts, expected);
    }

    #[test]
    fn it_bad_json_format_returns_error() {
        let scripts = read_scripts("src/file_reader/test_data/bad_formatting.json").unwrap();

        let expected = HashMap::from([(
            "Error parsing json".to_string(),
            "There was an error parsing your package.json".to_string(),
        )]);

        assert_eq!(scripts, expected);
    }

    #[test]
    fn it_no_file_returns_error() {
        let scripts = read_scripts("src/file_reader/test_data/no_file.json").unwrap();

        let expected = HashMap::from([(
            "file_not_found".to_string(),
            "package.json could not be found in this directory".to_string(),
        )]);

        assert_eq!(scripts, expected);
    }

    #[test]
    fn it_read_scripts_returns_scripts() {
        let scripts = read_scripts("src/file_reader/test_data/scripts.json").unwrap();

        let expected = HashMap::from([
            ("android".to_string(), "yarn android".to_string()),
            ("run".to_string(), "expo start".to_string()),
            ("start".to_string(), "yarn start".to_string()),
            ("ios".to_string(), "yarn ios".to_string()),
            ("test".to_string(), "jest".to_string()),
        ]);

        assert_eq!(scripts, expected);
    }
}
