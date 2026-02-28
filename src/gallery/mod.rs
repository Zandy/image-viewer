//! Gallery module for displaying image thumbnails
//!
//! Displays a grid of image thumbnails with configurable size and spacing.

use egui::{Color32, Rect, Response, Ui, Vec2};
use tracing::debug;

use crate::config::GalleryConfig;

/// Gallery state and rendering
pub struct Gallery {
    config: GalleryConfig,
    images: Vec<GalleryImage>,
    selected_index: Option<usize>,
}

#[derive(Clone)]
pub struct GalleryImage {
    pub path: std::path::PathBuf,
    pub thumbnail: Option<egui::TextureHandle>,
}

impl Gallery {
    /// Create a new Gallery with the given configuration.
    pub fn new(config: GalleryConfig) -> Self {
        debug!("Initializing Gallery with config: {:?}", config);
        
        Self {
            config,
            images: Vec::new(),
            selected_index: None,
        }
    }

    /// Add an image to the gallery.
    pub fn add_image(&mut self, path: std::path::PathBuf) {
        self.images.push(GalleryImage {
            path,
            thumbnail: None,
        });
    }

    /// Clear all images from the gallery.
    pub fn clear(&mut self) {
        self.images.clear();
        self.selected_index = None;
    }

    /// Render the gallery UI.
    pub fn ui(&mut self, ui: &mut Ui) {
        let available_width = ui.available_width();
        
        // Calculate items per row based on configuration
        let items_per_row = if self.config.items_per_row > 0 {
            self.config.items_per_row
        } else {
            // Auto-calculate based on available width
            let item_width = self.config.thumbnail_size as f32 + self.config.grid_spacing;
            (available_width / item_width).max(1.0) as usize
        };

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Use configured grid spacing
            let spacing = self.config.grid_spacing;
            ui.spacing_mut().item_spacing = Vec2::new(spacing, spacing);

            // Create grid layout
            egui::Grid::new("gallery_grid")
                .num_columns(items_per_row)
                .spacing([spacing, spacing])
                .show(ui, |ui| {
                    for (index, image) in self.images.iter_mut().enumerate() {
                        let response = self.render_thumbnail(ui, image, index);
                        
                        if response.clicked() {
                            self.selected_index = Some(index);
                            debug!("Selected image at index: {}", index);
                        }

                        // Move to next row after items_per_row items
                        if (index + 1) % items_per_row == 0 {
                            ui.end_row();
                        }
                    }
                });
        });
    }

    fn render_thumbnail(
        &mut self,
        ui: &mut Ui,
        image: &mut GalleryImage,
        index: usize,
    ) -> Response {
        let size = Vec2::splat(self.config.thumbnail_size as f32);
        let is_selected = self.selected_index == Some(index);

        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Background with hover/selection states
            let bg_color = if is_selected {
                Color32::from_rgb(52, 152, 219)  // Blue for selected (from UI_SPEC)
            } else if response.hovered() {
                Color32::from_rgb(60, 60, 60)
            } else {
                Color32::from_rgb(40, 40, 40)
            };

            // Rounded corners (4px radius from UI_SPEC)
            painter.rect_filled(rect, 4.0, bg_color);

            // Selection border
            if is_selected {
                painter.rect_stroke(rect, 4.0, egui::Stroke::new(2.0, Color32::WHITE));
                // Add subtle shadow effect for selected item
                painter.rect_stroke(
                    rect.expand(2.0), 
                    4.0, 
                    egui::Stroke::new(1.0, Color32::from_rgba_premultiplied(52, 152, 219, 100))
                );
            }

            // Thumbnail or placeholder
            if let Some(texture) = &image.thumbnail {
                painter.image(
                    texture.id(), 
                    rect.shrink(4.0),  // Slight padding
                    Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), 
                    Color32::WHITE
                );
            } else {
                // Placeholder with file name
                let text = image
                    .path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown");
                
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    text,
                    egui::FontId::proportional(12.0),
                    Color32::GRAY,
                );
            }

            // Filename label if enabled
            if self.config.show_filenames {
                let filename = image
                    .path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown");
                
                // Truncate long filenames
                let display_name = if filename.len() > 20 {
                    format!("{}...", &filename[..17])
                } else {
                    filename.to_string()
                };

                painter.text(
                    rect.center_bottom() + Vec2::new(0.0, -2.0),
                    egui::Align2::CENTER_BOTTOM,
                    display_name,
                    egui::FontId::proportional(10.0),
                    Color32::LIGHT_GRAY,
                );
            }
        }

        response
    }

    /// Get the currently selected image index.
    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    /// Get the number of images in the gallery.
    pub fn len(&self) -> usize {
        self.images.len()
    }

    /// Check if the gallery is empty.
    pub fn is_empty(&self) -> bool {
        self.images.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gallery_new() {
        let config = GalleryConfig::default();
        let gallery = Gallery::new(config);
        
        assert!(gallery.is_empty());
        assert_eq!(gallery.len(), 0);
        assert_eq!(gallery.selected_index(), None);
    }

    #[test]
    fn test_gallery_add_image() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config);
        
        gallery.add_image(std::path::PathBuf::from("test.png"));
        
        assert_eq!(gallery.len(), 1);
        assert!(!gallery.is_empty());
    }

    #[test]
    fn test_gallery_clear() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config);
        
        gallery.add_image(std::path::PathBuf::from("test1.png"));
        gallery.add_image(std::path::PathBuf::from("test2.png"));
        gallery.clear();
        
        assert!(gallery.is_empty());
        assert_eq!(gallery.selected_index(), None);
    }
}
