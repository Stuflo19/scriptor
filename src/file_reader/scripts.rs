use std::collections::HashMap;

use crate::file_reader::read_json::read_scripts;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Scripts {
    pub scripts: HashMap<String, String>,
}

impl Scripts {
    pub fn new() -> Self {
        let scripts = read_scripts("package.json").unwrap();

        Self { scripts }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_scripts_instantiates_correctly() {
        let scripts = Scripts::new();

        let expected = HashMap::from([
            ("android".to_string(), "yarn android".to_string()),
            ("run".to_string(), "expo start".to_string()),
            ("start".to_string(), "yarn start".to_string()),
            ("ios".to_string(), "yarn ios".to_string()),
            ("test".to_string(), "jest".to_string()),
        ]);

        assert_eq!(scripts.scripts, expected);
    }
}
