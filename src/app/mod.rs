//! Main application module
//!
//! This module contains the main application state and egui integration.
//! It handles view switching between Gallery and Viewer modes, and manages
//! configuration persistence.

use eframe::Frame;
use egui::Context;
use tracing::{debug, trace};

use crate::config::Config;
use crate::gallery::Gallery;
use crate::viewer::Viewer;

/// Main application state
pub struct ImageViewerApp {
    config: Config,
    gallery: Gallery,
    viewer: Viewer,
    current_view: View,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum View {
    Gallery,
    Viewer,
}

impl ImageViewerApp {
    /// Create a new application instance with the given configuration.
    ///
    /// This is called by eframe during application startup.
    pub fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        debug!("Initializing ImageViewerApp");

        // Configure fonts and styles
        Self::configure_fonts(&cc.egui_ctx);
        Self::configure_styles(&cc.egui_ctx);

        // Restore window position if available
        if let Some(position) = config.window.position() {
            debug!("Restoring window position to {:?}", position);
            // Note: Window position is handled by the native window manager
            // through eframe::NativeOptions in main.rs
        }

        Self {
            gallery: Gallery::new(config.gallery.clone()),
            viewer: Viewer::new(config.viewer.clone()),
            current_view: View::Gallery,
            config,
        }
    }

    /// Configure custom fonts for the application.
    fn configure_fonts(ctx: &Context) {
        let mut fonts = egui::FontDefinitions::default();
        
        // Use system default fonts (Segoe UI on Windows, SF Pro on macOS, etc.)
        // Custom fonts can be added here if needed
        
        ctx.set_fonts(fonts);
    }

    /// Configure visual styles for the application.
    fn configure_styles(ctx: &Context) {
        let mut style = (*ctx.style()).clone();
        
        // Configure spacing based on UI spec
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.window_margin = egui::Margin::same(10.0);
        style.spacing.button_padding = egui::vec2(12.0, 8.0);
        
        // Rounded corners for modern look
        style.visuals.widgets.inactive.rounding = egui::Rounding::same(4.0);
        style.visuals.widgets.hovered.rounding = egui::Rounding::same(4.0);
        style.visuals.widgets.active.rounding = egui::Rounding::same(4.0);
        
        ctx.set_style(style);
    }

    /// Save the current window state to configuration.
    ///
    /// This captures window size, position, and maximized state for persistence.
    fn save_window_state(&mut self, frame: &Frame) {
        // Get window info from frame
        let inner_size = frame.info().window_info.size;
        let position = frame.info().window_info.position.map(|p| [p.x, p.y]);
        let maximized = frame.info().window_info.maximized;

        debug!(
            "Saving window state: size={:?}, position={:?}, maximized={}",
            inner_size, position, maximized
        );

        self.config.update_from_window(
            [inner_size.x, inner_size.y],
            position,
            maximized,
        );
    }
}

impl eframe::App for ImageViewerApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        trace!("App update cycle");

        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open...").clicked() {
                        // TODO: Implement file dialog
                        ui.close_menu();
                    }
                    if ui.button("Exit").clicked() {
                        // Save window state before exiting
                        self.save_window_state(frame);
                        frame.close();
                    }
                });
                ui.menu_button("View", |ui| {
                    if ui.button("Gallery").clicked() {
                        self.current_view = View::Gallery;
                        ui.close_menu();
                    }
                    if ui.button("Viewer").clicked() {
                        self.current_view = View::Viewer;
                        ui.close_menu();
                    }
                });
            });
        });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                View::Gallery => self.gallery.ui(ui),
                View::Viewer => self.viewer.ui(ui),
            }
        });
    }

    /// Called when the application is about to close.
    ///
    /// This is the primary hook for persisting window state and configuration.
    fn on_close_event(&mut self) -> bool {
        debug!("Close event received, saving configuration");
        
        // Try to save config, but don't block close on error
        if let Err(e) = self.config.save() {
            tracing::error!("Failed to save config on close: {}", e);
        }
        
        true // Allow the window to close
    }

    /// Legacy save hook (kept for compatibility).
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        debug!("Legacy save hook called");
        // Configuration is now saved in on_close_event
    }
}
