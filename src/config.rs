//! Configuration management
//!
//! This module handles application configuration including:
//! - Window state (size, position, maximized)
//! - Gallery settings (thumbnail size, grid layout)
//! - Viewer settings (background color, zoom behavior, info panel)
//!
//! Configuration is stored in platform-specific directories:
//! - Linux: ~/.config/image-viewer/config.toml
//! - macOS: ~/Library/Application Support/com.imageviewer.image-viewer/config.toml
//! - Windows: %APPDATA%\image-viewer\config.toml

use std::path::PathBuf;

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

/// Application configuration root
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// Window settings
    pub window: WindowConfig,
    /// Gallery settings
    pub gallery: GalleryConfig,
    /// Viewer settings
    pub viewer: ViewerConfig,
}

/// Window configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Window width in pixels
    pub width: f32,
    /// Window height in pixels
    pub height: f32,
    /// Window X position (None for default center)
    pub x: Option<f32>,
    /// Window Y position (None for default center)
    pub y: Option<f32>,
    /// Whether window is maximized
    pub maximized: bool,
}

/// Gallery view configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GalleryConfig {
    /// Thumbnail size in pixels (range: 80-200)
    pub thumbnail_size: u32,
    /// Items per row (0 = auto-calculate based on window width)
    pub items_per_row: usize,
    /// Grid spacing in pixels
    pub grid_spacing: f32,
    /// Show file names under thumbnails
    pub show_filenames: bool,
}

/// Viewer view configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerConfig {
    /// Background color as [R, G, B]
    pub background_color: [u8; 3],
    /// Default fit mode: fit to window on open
    pub fit_to_window: bool,
    /// Show info panel by default
    pub show_info_panel: bool,
    /// Minimum zoom scale (10%)
    pub min_scale: f32,
    /// Maximum zoom scale (2000% = 20x)
    pub max_scale: f32,
    /// Zoom step multiplier (1.25 = 25% per step)
    pub zoom_step: f32,
    /// Enable smooth scrolling
    pub smooth_scroll: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            gallery: GalleryConfig::default(),
            viewer: ViewerConfig::default(),
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 1200.0,
            height: 800.0,
            x: None,
            y: None,
            maximized: false,
        }
    }
}

impl Default for GalleryConfig {
    fn default() -> Self {
        Self {
            thumbnail_size: 120,
            items_per_row: 0, // Auto-calculate
            grid_spacing: 12.0,
            show_filenames: true,
        }
    }
}

impl Default for ViewerConfig {
    fn default() -> Self {
        Self {
            background_color: [30, 30, 30],
            fit_to_window: true,
            show_info_panel: false,
            min_scale: 0.1,
            max_scale: 20.0,
            zoom_step: 1.25,
            smooth_scroll: true,
        }
    }
}

impl Config {
    /// Load configuration from the platform-specific config directory.
    /// 
    /// If the config file doesn't exist, creates a default config and saves it.
    /// If the config file is corrupted or invalid, logs a warning and returns default config.
    ///
    /// # Returns
    /// - `Ok(Config)` - Loaded or default configuration
    /// - `Err(anyhow::Error)` - Only for critical filesystem errors
    ///
    /// # Platform Paths
    /// - Linux: `~/.config/image-viewer/config.toml`
    /// - macOS: `~/Library/Application Support/com.imageviewer.image-viewer/config.toml`
    /// - Windows: `%APPDATA%\image-viewer\config.toml`
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        debug!("Loading config from: {:?}", config_path);

        if !config_path.exists() {
            info!("Config file not found at {:?}, creating default", config_path);
            let config = Self::default();
            if let Err(e) = config.save() {
                warn!("Failed to save default config: {}. Using defaults without saving.", e);
            }
            return Ok(config);
        }

        let content = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config from {:?}", config_path))?;

        match toml::from_str::<Self>(&content) {
            Ok(config) => {
                // Validate config values
                let validated = config.validate();
                if validated != config {
                    info!("Config values were adjusted to valid ranges");
                    // Save the corrected config
                    if let Err(e) = validated.save() {
                        warn!("Failed to save corrected config: {}", e);
                    }
                }
                Ok(validated)
            }
            Err(e) => {
                warn!("Failed to parse config file: {}. Using defaults.", e);
                let default = Self::default();
                // Try to backup the corrupted config
                let backup_path = config_path.with_extension("toml.bak");
                if let Err(backup_err) = std::fs::copy(&config_path, &backup_path) {
                    warn!("Failed to backup corrupted config: {}", backup_err);
                } else {
                    info!("Corrupted config backed up to {:?}", backup_path);
                }
                // Save default config
                if let Err(save_err) = default.save() {
                    warn!("Failed to save default config: {}", save_err);
                }
                Ok(default)
            }
        }
    }

    /// Save configuration to the platform-specific config directory.
    ///
    /// Automatically creates parent directories if they don't exist.
    ///
    /// # Returns
    /// - `Ok(())` - Config saved successfully
    /// - `Err(anyhow::Error)` - Failed to create directories or write file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
        }

        let content = toml::to_string_pretty(self)
            .with_context(|| "Failed to serialize config to TOML")?;

        std::fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config to {:?}", config_path))?;

        debug!("Config saved to {:?}", config_path);
        Ok(())
    }

    /// Get the configuration file path for the current platform.
    ///
    /// # Returns
    /// - `Ok(PathBuf)` - Full path to config.toml
    /// - `Err(anyhow::Error)` - Failed to determine config directory
    ///
    /// # Examples
    ///
    /// ```rust
    /// use image_viewer::config::Config;
    ///
    /// let path = Config::config_path().unwrap();
    /// assert!(path.file_name().unwrap() == "config.toml");
    /// ```
    pub fn config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "imageviewer", "image-viewer")
            .context("Failed to determine config directory: home directory not found")?;
        
        Ok(proj_dirs.config_dir().join("config.toml"))
    }

    /// Get the configuration directory path.
    ///
    /// Useful for storing additional config files (themes, presets, etc.)
    pub fn config_dir() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "imageviewer", "image-viewer")
            .context("Failed to determine config directory: home directory not found")?;
        
        Ok(proj_dirs.config_dir().to_path_buf())
    }

    /// Validate and normalize configuration values.
    ///
    /// Ensures all values are within acceptable ranges:
    /// - Window size: minimum 400x300
    /// - Thumbnail size: 80-200 pixels
    /// - Zoom scales: min < max, both > 0
    /// - Zoom step: 1.01-2.0
    fn validate(fn validate(self) -> Selfself) -> Self {
        Self {
            window: self.window.validate(),
            gallery: self.gallery.validate(),
            viewer: self.viewer.validate(),
        }
    }

    /// Update window state from eframe window info.
    ///
    /// Call this when the window is closed or when you want to save current state.
    pub fn update_from_window(&mut self, inner_size: [f32; 2], position: Option<[f32; 2]>, maximized: bool) {
        self.window.width = inner_size[0];
        self.window.height = inner_size[1];
        self.window.maximized = maximized;
        
        if let Some([x, y]) = position {
            self.window.x = Some(x);
            self.window.y = Some(y);
        }
    }
}

impl WindowConfig {
    /// Validate window configuration.
    fn validate(fn validate(self) -> Selfself) -> Self {
        Self {
            width: self.width.max(400.0),
            height: self.height.max(300.0),
            x: self.x,
            y: self.y,
            maximized: self.maximized,
        }
    }

    /// Get the window position as an array, or None if not set.
    pub fn position(&self) -> Option<[f32; 2]> {
        match (self.x, self.y) {
            (Some(x), Some(y)) => Some([x, y]),
            _ => None,
        }
    }

    /// Get the window size as an array.
    pub fn size(&self) -> [f32; 2] {
        [self.width, self.height]
    }
}

impl GalleryConfig {
    /// Validate gallery configuration.
    fn validate(fn validate(self) -> Selfself) -> Self {
        const MIN_THUMBNAIL: u32 = 80;
        const MAX_THUMBNAIL: u32 = 200;
        
        Self {
            thumbnail_size: self.thumbnail_size.clamp(MIN_THUMBNAIL, MAX_THUMBNAIL),
            items_per_row: self.items_per_row,
            grid_spacing: self.grid_spacing.max(0.0),
            show_filenames: self.show_filenames,
        }
    }
}

impl ViewerConfig {
    /// Validate viewer configuration.
    fn validate(fn validate(self) -> Selfself) -> Self {
        let min_scale = self.min_scale.max(0.01);
        let max_scale = self.max_scale.max(min_scale * 2.0);
        let zoom_step = self.zoom_step.clamp(1.01, 2.0);
        
        Self {
            background_color: [
                self.background_color[0],
                self.background_color[1],
                self.background_color[2],
            ],
            fit_to_window: self.fit_to_window,
            show_info_panel: self.show_info_panel,
            min_scale,
            max_scale,
            zoom_step,
            smooth_scroll: self.smooth_scroll,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // =========================================================================
    // Default Config Tests
    // =========================================================================

    #[test]
    fn test_default_config() {
        let config = Config::default();
        
        // Window defaults
        assert_eq!(config.window.width, 1200.0);
        assert_eq!(config.window.height, 800.0);
        assert_eq!(config.window.x, None);
        assert_eq!(config.window.y, None);
        assert!(!config.window.maximized);
        
        // Gallery defaults
        assert_eq!(config.gallery.thumbnail_size, 120);
        assert_eq!(config.gallery.items_per_row, 0);
        assert_eq!(config.gallery.grid_spacing, 12.0);
        assert!(config.gallery.show_filenames);
        
        // Viewer defaults
        assert_eq!(config.viewer.background_color, [30, 30, 30]);
        assert!(config.viewer.fit_to_window);
        assert!(!config.viewer.show_info_panel);
        assert_eq!(config.viewer.min_scale, 0.1);
        assert_eq!(config.viewer.max_scale, 20.0);
        assert_eq!(config.viewer.zoom_step, 1.25);
        assert!(config.viewer.smooth_scroll);
    }

    // =========================================================================
    // Serialization Tests
    // =========================================================================

    #[test]
    fn test_toml_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).expect("Failed to serialize");
        
        // Verify TOML contains expected sections
        assert!(toml_str.contains("[window]"));
        assert!(toml_str.contains("[gallery]"));
        assert!(toml_str.contains("[viewer]"));
        
        // Verify some values are present
        assert!(toml_str.contains("width = 1200"));
        assert!(toml_str.contains("thumbnail_size = 120"));
        assert!(toml_str.contains("background_color = [30, 30, 30]"));
    }

    #[test]
    fn test_toml_deserialization() {
        let toml_str = r#"
[window]
width = 1920.0
height = 1080.0
x = 100.0
y = 50.0
maximized = true

[gallery]
thumbnail_size = 150
items_per_row = 5
grid_spacing = 16.0
show_filenames = false

[viewer]
background_color = [50, 50, 50]
fit_to_window = false
show_info_panel = true
min_scale = 0.05
max_scale = 50.0
zoom_step = 1.5
smooth_scroll = false
"#;

        let config: Config = toml::from_str(toml_str).expect("Failed to deserialize");
        
        assert_eq!(config.window.width, 1920.0);
        assert_eq!(config.window.height, 1080.0);
        assert_eq!(config.window.x, Some(100.0));
        assert_eq!(config.window.y, Some(50.0));
        assert!(config.window.maximized);
        
        assert_eq!(config.gallery.thumbnail_size, 150);
        assert_eq!(config.gallery.items_per_row, 5);
        assert_eq!(config.gallery.grid_spacing, 16.0);
        assert!(!config.gallery.show_filenames);
        
        assert_eq!(config.viewer.background_color, [50, 50, 50]);
        assert!(!config.viewer.fit_to_window);
        assert!(config.viewer.show_info_panel);
        assert_eq!(config.viewer.min_scale, 0.05);
        assert_eq!(config.viewer.max_scale, 50.0);
        assert_eq!(config.viewer.zoom_step, 1.5);
        assert!(!config.viewer.smooth_scroll);
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = Config {
            window: WindowConfig {
                width: 1600.0,
                height: 900.0,
                x: Some(200.0),
                y: Some(100.0),
                maximized: true,
            },
            gallery: GalleryConfig {
                thumbnail_size: 100,
                items_per_row: 4,
                grid_spacing: 8.0,
                show_filenames: false,
            },
            viewer: ViewerConfig {
                background_color: [20, 20, 20],
                fit_to_window: false,
                show_info_panel: true,
                min_scale: 0.2,
                max_scale: 10.0,
                zoom_step: 1.1,
                smooth_scroll: false,
            },
        };

        let toml_str = toml::to_string_pretty(&original).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();
        
        assert_eq!(original, deserialized);
    }

    // =========================================================================
    // Invalid Config Handling Tests
    // =========================================================================

    #[test]
    fn test_invalid_toml_handling() {
        let invalid_toml = r#"
[window]
width = "not a number"
height = 800.0
"#;

        let result = toml::from_str::<Config>(invalid_toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_partial_config_loading() {
        // Partial config should fill in defaults for missing fields
        let partial_toml = r#"
[window]
width = 1400.0
maximized = true
"#;

        let config: Config = toml::from_str(partial_toml).expect("Should parse partial config");
        
        // Specified values
        assert_eq!(config.window.width, 1400.0);
        assert!(config.window.maximized);
        
        // Default values for missing fields
        assert_eq!(config.window.height, 800.0); // default
        assert_eq!(config.window.x, None);
        assert_eq!(config.window.y, None);
        
        // Other sections should use defaults
        assert_eq!(config.gallery.thumbnail_size, 120);
        assert_eq!(config.viewer.min_scale, 0.1);
    }

    #[test]
    fn test_corrupted_toml_fallback() {
        let corrupted = r#"
this is not valid toml {{{
[window
width = 100
"#;

        let result = toml::from_str::<Config>(corrupted);
        assert!(result.is_err());
    }

    // =========================================================================
    // Validation Tests
    // =========================================================================

    #[test]
    fn test_window_validation() {
        let config = Config {
            window: WindowConfig {
                width: 100.0,  // Too small
                height: 50.0,  // Too small
                ..Default::default()
            },
            ..Default::default()
        };

        let validated = config.validate();
        
        // Should be clamped to minimums
        assert_eq!(validated.window.width, 400.0);
        assert_eq!(validated.window.height, 300.0);
    }

    #[test]
    fn test_gallery_validation() {
        // Test thumbnail size clamping - too small
        let config = Config {
            gallery: GalleryConfig {
                thumbnail_size: 50,  // Below minimum
                ..Default::default()
            },
            ..Default::default()
        };
        let validated = config.validate();
        assert_eq!(validated.gallery.thumbnail_size, 80);

        // Test thumbnail size clamping - too large
        let config = Config {
            gallery: GalleryConfig {
                thumbnail_size: 300,  // Above maximum
                ..Default::default()
            },
            ..Default::default()
        };
        let validated = config.validate();
        assert_eq!(validated.gallery.thumbnail_size, 200);

        // Test negative spacing
        let config = Config {
            gallery: GalleryConfig {
                grid_spacing: -5.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let validated = config.validate();
        assert_eq!(validated.gallery.grid_spacing, 0.0);
    }

    #[test]
    fn test_viewer_validation() {
        // Test min/max scale relationship
        let config = Config {
            viewer: ViewerConfig {
                min_scale: 5.0,
                max_scale: 1.0,  // Smaller than min
                ..Default::default()
            },
            ..Default::default()
        };
        let validated = config.validate();
        assert!(validated.viewer.max_scale >= validated.viewer.min_scale * 2.0);

        // Test zoom step clamping - too small
        let config = Config {
            viewer: ViewerConfig {
                zoom_step: 1.005,  // Below minimum
                ..Default::default()
            },
            ..Default::default()
        };
        let validated = config.validate();
        assert_eq!(validated.viewer.zoom_step, 1.01);

        // Test zoom step clamping - too large
        let config = Config {
            viewer: ViewerConfig {
                zoom_step: 5.0,  // Above maximum
                ..Default::default()
            },
            ..Default::default()
        };
        let validated = config.validate();
        assert_eq!(validated.viewer.zoom_step, 2.0);
    }

    #[test]
    fn test_negative_scale_handling() {
        let config = Config {
            viewer: ViewerConfig {
                min_scale: -0.5,  // Invalid negative
                max_scale: -1.0,  // Invalid negative
                ..Default::default()
            },
            ..Default::default()
        };
        
        let validated = config.validate();
        assert!(validated.viewer.min_scale > 0.0);
        assert!(validated.viewer.max_scale > validated.viewer.min_scale);
    }

    // =========================================================================
    // File I/O Tests
    // =========================================================================

    #[test]
    fn test_config_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");

        // Create and save a config
        let original = Config {
            window: WindowConfig {
                width: 1400.0,
                height: 900.0,
                x: Some(100.0),
                y: Some(200.0),
                maximized: false,
            },
            gallery: GalleryConfig {
                thumbnail_size: 100,
                items_per_row: 6,
                grid_spacing: 10.0,
                show_filenames: true,
            },
            viewer: ViewerConfig {
                background_color: [40, 40, 40],
                fit_to_window: true,
                show_info_panel: false,
                min_scale: 0.15,
                max_scale: 15.0,
                zoom_step: 1.2,
                smooth_scroll: true,
            },
        };

        // Write directly to test file
        let content = toml::to_string_pretty(&original).unwrap();
        std::fs::write(&config_path, content).unwrap();

        // Read back and verify
        let loaded_content = std::fs::read_to_string(&config_path).unwrap();
        let loaded: Config = toml::from_str(&loaded_content).unwrap();

        assert_eq!(original, loaded);
    }

    #[test]
    fn test_config_directory_creation() {
        let temp_dir = TempDir::new().unwrap();
        let nested_dir = temp_dir.path().join("deep").join("nested").join("dir");
        let config_path = nested_dir.join("config.toml");

        let config = Config::default();
        let content = toml::to_string_pretty(&config).unwrap();
        
        // Create parent directories
        std::fs::create_dir_all(&nested_dir).unwrap();
        std::fs::write(&config_path, content).unwrap();

        assert!(config_path.exists());
    }

    // =========================================================================
    // Window State Tests
    // =========================================================================

    #[test]
    fn test_update_from_window() {
        let mut config = Config::default();
        
        config.update_from_window([1600.0, 900.0], Some([100.0, 50.0]), true);
        
        assert_eq!(config.window.width, 1600.0);
        assert_eq!(config.window.height, 900.0);
        assert_eq!(config.window.x, Some(100.0));
        assert_eq!(config.window.y, Some(50.0));
        assert!(config.window.maximized);
    }

    #[test]
    fn test_update_from_window_without_position() {
        let mut config = Config::default();
        
        config.update_from_window([1400.0, 800.0], None, false);
        
        assert_eq!(config.window.width, 1400.0);
        assert_eq!(config.window.height, 800.0);
        // Position should remain unchanged when None is passed
        assert_eq!(config.window.x, None);
        assert_eq!(config.window.y, None);
        assert!(!config.window.maximized);
    }

    #[test]
    fn test_window_position_helper() {
        let config_with_pos = WindowConfig {
            x: Some(100.0),
            y: Some(200.0),
            ..Default::default()
        };
        assert_eq!(config_with_pos.position(), Some([100.0, 200.0]));

        let config_without_pos = WindowConfig {
            x: None,
            y: Some(200.0),
            ..Default::default()
        };
        assert_eq!(config_without_pos.position(), None);

        let config_partial = WindowConfig {
            x: Some(100.0),
            y: None,
            ..Default::default()
        };
        assert_eq!(config_partial.position(), None);
    }

    #[test]
    fn test_window_size_helper() {
        let config = WindowConfig {
            width: 1920.0,
            height: 1080.0,
            ..Default::default()
        };
        assert_eq!(config.size(), [1920.0, 1080.0]);
    }

    // =========================================================================
    // Edge Case Tests
    // =========================================================================

    #[test]
    fn test_empty_toml_file() {
        let empty = "";
        let config: Config = toml::from_str(empty).expect("Empty TOML should parse to defaults");
        
        // All values should be defaults
        assert_eq!(config, Config::default());
    }

    #[test]
    fn test_very_large_values() {
        let toml_str = r#"
[window]
width = 1000000.0
height = 1000000.0

[gallery]
thumbnail_size = 4294967295
"#;

        let config: Config = toml::from_str(toml_str).expect("Should parse");
        let validated = config.validate();
        
        // Values should be clamped
        assert_eq!(validated.window.width, 1000000.0);  // No upper limit on window size
        assert_eq!(validated.gallery.thumbnail_size, 200);  // Clamped to max
    }

    #[test]
    fn test_special_characters_in_toml() {
        // Test that special characters don't break parsing
        let toml_str = r#"
[window]
width = 1200.0
# This is a comment with special chars: !@#$%^&*()
height = 800.0
"#;

        let config: Config = toml::from_str(toml_str).expect("Should handle comments");
        assert_eq!(config.window.width, 1200.0);
        assert_eq!(config.window.height, 800.0);
    }

    #[test]
    fn test_config_equality() {
        let config1 = Config::default();
        let config2 = Config::default();
        assert_eq!(config1, config2);

        let mut config3 = Config::default();
        config3.window.width = 999.0;
        assert_ne!(config1, config3);
    }

    #[test]
    fn test_clone_config() {
        let original = Config::default();
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_debug_format() {
        let config = Config::default();
        let debug_str = format!("{:?}", config);
        
        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("window"));
        assert!(debug_str.contains("gallery"));
        assert!(debug_str.contains("viewer"));
    }
}
