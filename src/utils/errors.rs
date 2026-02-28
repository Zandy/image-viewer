//! Error types for the image viewer

use std::io;

use thiserror::Error;

/// Main application error type
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Image decoding error: {0}")]
    Decode(#[from] DecoderError),

    #[error("UI error: {0}")]
    Ui(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Image decoder specific errors
#[derive(Error, Debug)]
pub enum DecoderError {
    #[error("Unsupported image format")]
    UnsupportedFormat,

    #[error("Failed to decode image: {0}")]
    DecodeFailed(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid image data")]
    InvalidData,
}

/// Gallery related errors
#[derive(Error, Debug)]
pub enum GalleryError {
    #[error("Failed to load image: {0}")]
    LoadFailed(String),

    #[error("Thumbnail generation failed: {0}")]
    ThumbnailFailed(String),

    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),
}
