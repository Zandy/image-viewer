//! Image Viewer - A modern image viewer built with Rust and egui
//!
//! Features:
//! - Cross-platform image viewing (PNG, JPEG, GIF, WebP, BMP, TIFF)
//! - Gallery mode with thumbnail grid
//! - Viewer mode with zoom and pan
//! - Persistent configuration

use std::sync::Arc;

use anyhow::Result;
use eframe::NativeOptions;
use tracing::{info, warn};

use crate::app::ImageViewerApp;
use crate::config::Config;

mod app;
mod config;
mod decoder;
mod gallery;
mod utils;
mod viewer;

fn main() -> Result<()> {
    // Initialize logging with tracing
    tracing_subscriber::fmt()
        .with_env_filter("info,image_viewer=debug")
        .with_target(true)
        .with_thread_ids(true)
        .init();

    info!("Starting Image Viewer v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration from platform-specific config directory
    let config = match Config::load() {
        Ok(cfg) => {
            info!("Configuration loaded successfully");
            cfg
        }
        Err(e) => {
            warn!("Failed to load config: {}. Using defaults.", e);
            Config::default()
        }
    };

    // Build native window options from configuration
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([config.window.width, config.window.height])
        .with_min_inner_size([400.0, 300.0]);

    // Restore window position if available
    if let Some([x, y]) = config.window.position() {
        info!("Restoring window position to ({}, {})", x, y);
        viewport = viewport.with_position([x, y]);
    }

    // Set maximized state
    if config.window.maximized {
        viewport = viewport.with_maximized(true);
    }

    let native_options = NativeOptions {
        viewport,
        ..Default::default()
    };

    info!(
        "Window config: size={}x{}, maximized={}",
        config.window.width, config.window.height, config.window.maximized
    );

    // Run the application
    eframe::run_native(
        "Image Viewer",
        native_options,
        Box::new(|cc| Box::new(ImageViewerApp::new(cc, config))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run application: {}", e))?;

    Ok(())
}
