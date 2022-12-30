#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not load data: {0}")]
    LoadingFailed(String)
}