mod database;
mod error;

pub(crate) type Result<T> = std::result::Result<T, error::Error>;

pub use database::Database;
pub use error::Error;
