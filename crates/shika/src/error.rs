pub enum Error {
    Database(shika_database::Error),
    Workspace(shika_workspace::Error),
    Renderer(shika_renderer::Error),
    RuntimeInitializationFailed,
    DatabaseNotPulled,
    TemplateNotFound(String),
}

impl From<shika_database::Error> for Error {
    fn from(err: shika_database::Error) -> Self {
        Error::Database(err)
    }
}
impl From<shika_workspace::Error> for Error {
    fn from(err: shika_workspace::Error) -> Self {
        Error::Workspace(err)
    }
}
impl From<shika_renderer::Error> for Error {
    fn from(err: shika_renderer::Error) -> Self {
        Error::Renderer(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DatabaseNotPulled => write!(f, "Database Not Pulled"),
            Error::Database(err) => write!(f, "Database error: {err}"),
            Error::Workspace(err) => write!(f, "Workspace error: {err}"),
            Error::Renderer(err) => write!(f, "Renderer error: {err}"),
            Error::RuntimeInitializationFailed => write!(f, "Failed to initialize runtime"),
            Error::TemplateNotFound(template) => write!(f, "Template not found: {template}"),
        }
    }
}
