use std::collections::HashMap;

use crate::file_reader::read_json::read_json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Scripts {
    pub scripts: HashMap<String, String>,
}

impl Scripts {
    pub fn new() -> Self {
        let scripts = read_json().unwrap();

        Self { scripts }
    }

    // pub fn scripts(&self) -> &HashMap<String, String> {
    //     &self.scripts
    // }
}
