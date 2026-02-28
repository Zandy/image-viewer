//! Image decoder module

use std::path::Path;

use image::DynamicImage;
use tracing::{debug, error, instrument};

use crate::utils::errors::DecoderError;

/// Supported image formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Gif,
    Webp,
    Tiff,
    Bmp,
}

/// Image decoder for loading images
pub struct ImageDecoder;

impl ImageDecoder {
    /// Create a new image decoder
    pub fn new() -> Self {
        Self
    }

    /// Decode an image from a file path
    #[instrument(skip(self, path))]
    pub fn decode_from_file(&self,
        path: &Path,
    ) -> Result<DynamicImage, DecoderError> {
        debug!("Decoding image from: {:?}", path);

        let format = self.detect_format(path)?;
        debug!("Detected format: {:?}", format);

        let img = image::open(path).map_err(|e| {
            error!("Failed to decode image: {}", e);
            DecoderError::DecodeFailed(e.to_string())
        })?;

        Ok(img)
    }

    /// Decode an image from memory
    #[instrument(skip(self, data))]
    pub fn decode_from_memory(
        &self,
        data: &[u8],
    ) -> Result<DynamicImage, DecoderError> {
        debug!("Decoding image from memory, size: {} bytes", data.len());

        let img = image::load_from_memory(data).map_err(|e| {
            error!("Failed to decode image from memory: {}", e);
            DecoderError::DecodeFailed(e.to_string())
        })?;

        Ok(img)
    }

    /// Detect image format from file extension
    fn detect_format(&self,
        path: &Path,
    ) -> Result<ImageFormat, DecoderError> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| DecoderError::UnsupportedFormat)?;

        match ext.to_lowercase().as_str() {
            "png" => Ok(ImageFormat::Png),
            "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
            "gif" => Ok(ImageFormat::Gif),
            "webp" => Ok(ImageFormat::Webp),
            "tiff" | "tif" => Ok(ImageFormat::Tiff),
            "bmp" => Ok(ImageFormat::Bmp),
            _ => Err(DecoderError::UnsupportedFormat),
        }
    }

    /// Check if a file is a supported image
    pub fn is_supported(path: &Path) -> bool {
        matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("png" | "jpg" | "jpeg" | "gif" | "webp" | "tiff" | "tif" | "bmp")
        )
    }
}

impl Default for ImageDecoder {
    fn default() -> Self {
        Self::new()
    }
}
