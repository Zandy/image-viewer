//! Image viewer module for displaying full-size images
//!
//! Supports zoom, pan, and various display modes.

use egui::{Color32, Rect, Sense, Ui, Vec2};
use tracing::debug;

use crate::config::ViewerConfig;

/// Image viewer state and rendering
pub struct Viewer {
    config: ViewerConfig,
    current_image: Option<ViewImage>,
    scale: f32,
    offset: Vec2,
    dragging: bool,
}

#[derive(Clone)]
pub struct ViewImage {
    pub path: std::path::PathBuf,
    pub texture: Option<egui::TextureHandle>,
    pub dimensions: Option<(u32, u32)>,
}

impl Viewer {
    /// Create a new Viewer with the given configuration.
    pub fn new(config: ViewerConfig) -> Self {
        debug!("Initializing Viewer with config: {:?}", config);
        
        Self {
            config,
            current_image: None,
            scale: 1.0,
            offset: Vec2::ZERO,
            dragging: false,
        }
    }

    /// Set the current image to display.
    pub fn set_image(&mut self, path: std::path::PathBuf) {
        self.current_image = Some(ViewImage {
            path,
            texture: None,
            dimensions: None,
        });
        self.scale = 1.0;
        self.offset = Vec2::ZERO;
    }

    /// Clear the current image.
    pub fn clear(&mut self) {
        self.current_image = None;
        self.scale = 1.0;
        self.offset = Vec2::ZERO;
    }

    /// Render the viewer UI.
    pub fn ui(&mut self, ui: &mut Ui) {
        let available_size = ui.available_size();
        let bg_color = Color32::from_rgb(
            self.config.background_color[0],
            self.config.background_color[1],
            self.config.background_color[2],
        );

        // Background
        let (rect, response) = ui.allocate_exact_size(available_size, Sense::drag());
        ui.painter().rect_filled(rect, 0.0, bg_color);

        let current_image = self.current_image.clone(); if let Some(ref image) = current_image {
            self.render_image(ui, image, rect, &response);
        } else {
            // No image loaded - show placeholder
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "No image selected\nOpen an image or select from gallery",
                egui::FontId::proportional(16.0),
                Color32::GRAY,
            );
        }

        // Info panel
        if self.config.show_info_panel {
            self.render_info_panel(ui);
        }

        // Zoom indicator
        self.render_zoom_indicator(ui, rect);
    }

    fn render_image(
        &mut self,
        ui: &mut Ui,
        image: &ViewImage,
        rect: Rect,
        response: &egui::Response,
    ) {
        if let Some(texture) = &image.texture {
            // Calculate display size based on scale and fit mode
            let texture_size = texture.size_vec2();
            let display_size = self.calculate_display_size(texture_size, rect.size());

            // Handle dragging (pan)
            if response.dragged() {
                self.offset += response.drag_delta();
                self.dragging = true;
            } else {
                self.dragging = false;
            }

            // Handle zoom with mouse wheel
            if response.hovered() && !self.dragging {
                let scroll_delta = ui.input(|i| i.scroll_delta.y);
                if scroll_delta != 0.0 && self.config.smooth_scroll {
                    let zoom_factor = 1.0 + scroll_delta * 0.001;
                    let new_scale = (self.scale * zoom_factor)
                        .clamp(self.config.min_scale, self.config.max_scale);
                    
                    // Zoom towards mouse position
                    if new_scale != self.scale {
                        let mouse_pos = ui.input(|i| i.pointer.hover_pos())
                            .unwrap_or(rect.center());
                        let zoom_center = mouse_pos - rect.center() - self.offset;
                        self.offset -= zoom_center * (new_scale / self.scale - 1.0);
                        self.scale = new_scale;
                    }
                }
            }

            // Draw image centered with offset
            let center = rect.center() + self.offset;
            let image_rect = Rect::from_center_size(center, display_size);
            
            ui.painter().image(
                texture.id(),
                image_rect,
                Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        } else {
            // Loading state
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Loading...",
                egui::FontId::proportional(14.0),
                Color32::GRAY,
            );
        }
    }

    /// Calculate display size based on current scale and fit mode.
    fn calculate_display_size(
        &self,
        image_size: Vec2,
        container_size: Vec2,
    ) -> Vec2 {
        let base_size = if self.config.fit_to_window && self.scale == 1.0 {
            self.fit_to_rect(image_size, container_size)
        } else {
            image_size
        };
        
        base_size * self.scale
    }

    /// Calculate size to fit image within container while maintaining aspect ratio.
    fn fit_to_rect(
        &self,
        image_size: Vec2,
        container_size: Vec2,
    ) -> Vec2 {
        let scale_x = container_size.x / image_size.x;
        let scale_y = container_size.y / image_size.y;
        let scale = scale_x.min(scale_y).min(1.0);
        
        image_size * scale
    }

    /// Render the info panel window.
    fn render_info_panel(
        &self,
        ui: &mut Ui,
    ) {
        let current_image = self.current_image.clone(); if let Some(ref image) = current_image {
            egui::Window::new("📋 Image Info")
                .default_pos([10.0, 10.0])
                .default_size([250.0, 150.0])
                .resizable(true)
                .collapsible(true)
                .show(ui.ctx(), |ui| {
                    ui.label(format!("Path: {}", image.path.display()));
                    
                    if let Some((w, h)) = image.dimensions {
                        ui.label(format!("Dimensions: {} x {}", w, h));
                        let mp = (w as f64 * h as f64) / 1_000_000.0;
                        ui.label(format!("Megapixels: {:.2} MP", mp));
                    }
                    
                    ui.separator();
                    ui.label(format!("Zoom: {:.1}%", self.scale * 100.0));
                    ui.label(format!("Offset: ({:.0}, {:.0})", self.offset.x, self.offset.y));
                });
        }
    }

    /// Render zoom percentage indicator.
    fn render_zoom_indicator(
        &self,
        ui: &mut Ui,
        rect: Rect,
    ) {
        let zoom_text = format!("{:.0}%", self.scale * 100.0);
        let pos = rect.right_bottom() - Vec2::new(10.0, 10.0);
        
        // Background pill
        let font = egui::FontId::proportional(12.0);
        let text_size = ui.painter().layout(
            zoom_text.clone(),
            font.clone(),
            Color32::WHITE,
            f32::INFINITY,
        ).size();
        
        let pill_rect = Rect::from_center_size(
            pos - Vec2::new(text_size.x / 2.0 + 5.0, text_size.y / 2.0 + 5.0),
            text_size + Vec2::new(16.0, 10.0),
        );
        
        ui.painter().rect_filled(
            pill_rect,
            4.0,
            Color32::from_rgba_premultiplied(0, 0, 0, 180),
        );
        
        ui.painter().text(
            pill_rect.center(),
            egui::Align2::CENTER_CENTER,
            zoom_text,
            font,
            Color32::WHITE,
        );
    }

    /// Zoom in by one step.
    pub fn zoom_in(&mut self) {
        self.scale = (self.scale * self.config.zoom_step)
            .min(self.config.max_scale);
    }

    /// Zoom out by one step.
    pub fn zoom_out(&mut self) {
        self.scale = (self.scale / self.config.zoom_step)
            .max(self.config.min_scale);
    }

    /// Reset zoom to 100%.
    pub fn reset_zoom(&mut self) {
        self.scale = 1.0;
        self.offset = Vec2::ZERO;
    }

    /// Fit image to window.
    pub fn fit_to_window(&mut self) {
        self.scale = 1.0;
        self.offset = Vec2::ZERO;
    }

    /// Get current scale.
    pub fn scale(&self) -> f32 {
        self.scale
    }

    /// Get current offset.
    pub fn offset(&self) -> Vec2 {
        self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewer_new() {
        let config = ViewerConfig::default();
        let viewer = Viewer::new(config);
        
        assert_eq!(viewer.scale(), 1.0);
        assert_eq!(viewer.offset(), Vec2::ZERO);
    }

    #[test]
    fn test_zoom_in() {
        let config = ViewerConfig::default();
        let mut viewer = Viewer::new(config);
        
        viewer.zoom_in();
        assert!(viewer.scale() > 1.0);
        assert!(viewer.scale() <= 20.0); // max_scale
    }

    #[test]
    fn test_zoom_out() {
        let config = ViewerConfig::default();
        let mut viewer = Viewer::new(config);
        
        viewer.zoom_out();
        assert!(viewer.scale() < 1.0);
        assert!(viewer.scale() >= 0.1); // min_scale
    }

    #[test]
    fn test_reset_zoom() {
        let config = ViewerConfig::default();
        let mut viewer = Viewer::new(config);
        
        viewer.zoom_in();
        viewer.zoom_in();
        viewer.reset_zoom();
        
        assert_eq!(viewer.scale(), 1.0);
        assert_eq!(viewer.offset(), Vec2::ZERO);
    }

    #[test]
    fn test_zoom_limits() {
        let config = ViewerConfig {
            min_scale: 0.1,
            max_scale: 5.0,
            zoom_step: 2.0,
            ..Default::default()
        };
        let mut viewer = Viewer::new(config);
        
        // Zoom in beyond max
        for _ in 0..10 {
            viewer.zoom_in();
        }
        assert_eq!(viewer.scale(), 5.0);
        
        // Reset and zoom out beyond min
        viewer.reset_zoom();
        for _ in 0..10 {
            viewer.zoom_out();
        }
        assert_eq!(viewer.scale(), 0.1);
    }

    #[test]
    fn test_set_and_clear_image() {
        let config = ViewerConfig::default();
        let mut viewer = Viewer::new(config);
        
        viewer.set_image(std::path::PathBuf::from("test.png"));
        // Image should be set (can't easily verify without egui context)
        
        viewer.clear();
        // Should reset state
        assert_eq!(viewer.scale(), 1.0);
        assert_eq!(viewer.offset(), Vec2::ZERO);
    }
}
