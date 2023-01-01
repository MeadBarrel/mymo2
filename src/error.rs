#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to start the application: {0}")]
    StartupFailed(String),
}

pub type Result<T> = std::result::Result<T, Error>;