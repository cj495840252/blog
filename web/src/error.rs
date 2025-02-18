use std::io::Error as IOError;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum WebError {
    // #[cfg(feature="server")]
    // #[error("Sqlx error: {}")]
    // SqlxError(#[from] sqlx::Error),
    #[error("load config failed: {}", 0)]
    LoadConfigError(#[from] IOError),

    #[error("DeserializeYamlError: {}", 0)]
    DeserializeYamlError(#[from] serde_yaml::Error),
}
