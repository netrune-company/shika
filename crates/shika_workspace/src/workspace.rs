use serde::{Serialize, de::DeserializeOwned};
use shika_database::Database;

use crate::{config::Config, error::Error};
use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub struct Workspace {
    pub path: PathBuf,
    pub config: Config,
    pub database: Option<Database>,
}

impl Workspace {
    pub fn load() -> Result<Self, Error> {
        let current_dir = std::env::current_dir()?;
        Self::load_recursive(current_dir)
    }

    /// Recursively look for a ".shika" directory to mark the root directory of the workspace.
    ///
    fn load_recursive<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        let full_path = path.join(".shika");

        if !full_path.exists() {
            let Some(parent) = path.parent() else {
                return Err(Error::NoWorkspaceFound);
            };

            Self::load_recursive(parent)
        } else {
            let path = path.to_path_buf();
            let config = Config::load(full_path.join("config.yaml"))?;
            let database = Database::load(full_path.join("database.yaml"))?;

            Ok(Workspace {
                path,
                config,
                database,
            })
        }
    }

    pub fn write<P: AsRef<Path>, S: Serialize>(&self, path: P, data: S) -> Result<(), Error> {
        let path = self.path.join(".shika").join(path);
        serde_yml::to_writer(File::create(path)?, &data).map_err(Error::from)?;

        Ok(())
    }

    pub fn read<D: DeserializeOwned, P: AsRef<Path>>(&self, path: P) -> Result<Option<D>, Error> {
        let path = self.path.join(".shika").join(path);
        let Ok(file) = File::open(path).map_err(Error::IO) else {
            return Ok(None);
        };

        let data = serde_yml::from_reader(file).map_err(Error::Deserialization)?;

        Ok(data)
    }

    pub fn write_file<P: AsRef<Path>>(&self, path: P, data: &str) -> Result<(), Error> {
        let path = self.path.join(path);

        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }

        let mut file = File::create(path)?;
        file.write_all(data.as_bytes()).map_err(Error::from)?;

        Ok(())
    }
}
