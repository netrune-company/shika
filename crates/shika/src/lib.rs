pub mod commands;
pub mod error;

pub const DATABASE_FILE_PATH: &str = "database.yaml";

pub type Result<T> = std::result::Result<T, error::Error>;
