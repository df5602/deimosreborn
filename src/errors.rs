use thiserror::Error;

#[allow(clippy::enum_variant_names, dead_code)]
#[derive(Error, Debug)]
pub enum SdlError {
    #[error("Initialization error: {0}")]
    InitError(String),
    #[error("Failed to load sprite: {0}")]
    SpriteLoadError(String),
    #[error("Failed to create placeholder texture: {0}")]
    PlaceHolderCreateError(String),
    #[error("Failed to load sound sample: {0}")]
    SoundLoadError(String),
    #[error("Failed to play audio sample: {0}")]
    AudioPlayError(String),
}
