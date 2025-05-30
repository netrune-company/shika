use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    InvalidConfig(serde_yml::Error),
    Deserialization(serde_yml::Error),
    NoWorkspaceFound,
    Database(shika_database::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(error) => f.write_str(error.to_string().as_str()),
            Error::NoWorkspaceFound => f.write_str("No workspace found"),
            Error::Deserialization(error) => f.write_str(error.to_string().as_str()),
            Error::InvalidConfig(error) => f.write_str(error.to_string().as_str()),
            Error::Database(error) => f.write_str(error.to_string().as_str()),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<serde_yml::Error> for Error {
    fn from(value: serde_yml::Error) -> Self {
        Error::InvalidConfig(value)
    }
}

impl From<shika_database::Error> for Error {
    fn from(value: shika_database::Error) -> Self {
        Error::Database(value)
    }
}

impl std::error::Error for Error {}
