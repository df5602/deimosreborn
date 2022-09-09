use thiserror::Error;

#[derive(Error, Debug)]
pub enum SdlError {
    #[error("Initialization error: {0}")]
    InitError(String),
    #[error("Failed to load sprite: {0}")]
    SpriteLoadError(String),
}
