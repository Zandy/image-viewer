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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_decoder_error_variants() {
        let err1 = DecoderError::UnsupportedFormat;
        assert!(err1.to_string().contains("Unsupported"));

        let err2 = DecoderError::DecodeFailed("test error".to_string());
        assert!(err2.to_string().contains("test error"));

        let err3 = DecoderError::FileNotFound("/path/to/file".to_string());
        assert!(err3.to_string().contains("/path/to/file"));

        let err4 = DecoderError::InvalidData;
        assert!(err4.to_string().contains("Invalid"));
    }

    #[test]
    fn test_decoder_error_debug() {
        let err = DecoderError::UnsupportedFormat;
        let debug = format!("{:?}", err);
        assert!(debug.contains("UnsupportedFormat"));
    }

    #[test]
    fn test_gallery_error_variants() {
        let err1 = GalleryError::LoadFailed("image.png".to_string());
        assert!(err1.to_string().contains("image.png"));

        let err2 = GalleryError::ThumbnailFailed("resize error".to_string());
        assert!(err2.to_string().contains("resize error"));

        let err3 = GalleryError::DirectoryNotFound("/path/to/dir".to_string());
        assert!(err3.to_string().contains("/path/to/dir"));
    }

    #[test]
    fn test_gallery_error_debug() {
        let err = GalleryError::LoadFailed("test".to_string());
        let debug = format!("{:?}", err);
        assert!(debug.contains("LoadFailed"));
    }

    #[test]
    fn test_app_error_from_io() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let app_err = AppError::from(io_err);
        
        match app_err {
            AppError::Io(_) => {},
            _ => panic!("Expected Io error"),
        }
    }

    #[test]
    fn test_app_error_from_decoder() {
        let decoder_err = DecoderError::UnsupportedFormat;
        let app_err = AppError::from(decoder_err);
        
        match app_err {
            AppError::Decode(_) => {},
            _ => panic!("Expected Decode error"),
        }
    }

    #[test]
    fn test_app_error_variants() {
        let err1 = AppError::Io(io::Error::new(io::ErrorKind::Other, "io error"));
        assert!(err1.to_string().contains("IO error"));

        let err2 = AppError::Config("invalid config".to_string());
        assert!(err2.to_string().contains("Configuration"));
        assert!(err2.to_string().contains("invalid config"));

        let err3 = AppError::Ui("ui error".to_string());
        assert!(err3.to_string().contains("UI error"));

        let err4 = AppError::Unknown("unknown".to_string());
        assert!(err4.to_string().contains("Unknown"));
    }

    #[test]
    fn test_error_display_messages() {
        let decoder_err = DecoderError::DecodeFailed("corrupt data".to_string());
        assert_eq!(
            decoder_err.to_string(),
            "Failed to decode image: corrupt data"
        );

        let gallery_err = GalleryError::ThumbnailFailed("timeout".to_string());
        assert_eq!(
            gallery_err.to_string(),
            "Thumbnail generation failed: timeout"
        );
    }
}
