#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    InvalidDatabaseFile(serde_yml::Error),
    MissingDatabaseUrl(std::env::VarError),
    Connection(sqlx::Error),
    Query(sqlx::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(error) => write!(f, "IO error: {error}"),
            Error::InvalidDatabaseFile(error) => write!(f, "Invalid database file: {error}"),
            Error::MissingDatabaseUrl(error) => write!(f, "Missing database URL: {error}"),
            Error::Connection(error) => write!(f, "Connection error: {error}"),
            Error::Query(error) => write!(f, "Query error: {error}"),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::Query(value)
    }
}
