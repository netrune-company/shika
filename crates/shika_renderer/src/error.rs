#[derive(Debug)]
pub enum Error {
    Render(tera::Error),
    FilterError(String),
}

impl From<tera::Error> for Error {
    fn from(value: tera::Error) -> Self {
        Error::Render(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Render(error) => write!(f, "Renderer error: {error}"),
            Error::FilterError(_) => write!(f, "FilterError"),
        }
    }
}
