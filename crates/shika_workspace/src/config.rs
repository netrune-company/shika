use std::{collections::HashMap, fs::File, path::Path};

use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub exclude_tables: Option<Vec<String>>,
    pub templates: HashMap<String, Template>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {
    pub path: String,
    pub output: String,
    pub language: String,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path)?;
        let config = serde_yml::from_reader(file)?;

        Ok(config)
    }
}
