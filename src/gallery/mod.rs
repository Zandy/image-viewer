//! 图库模块 - 用于显示图像缩略图
//!
//! 以可配置大小和间距的网格显示图像缩略图，支持异步加载。

use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};

use egui::{Color32, Rect, Response, Ui, Vec2};
use tracing::{debug, error, info};

use crate::config::GalleryConfig;

/// 缩略图加载请求
struct ThumbnailRequest {
    path: PathBuf,
    index: usize,
}

/// 缩略图加载结果
struct ThumbnailResult {
    index: usize,
    texture: Option<egui::TextureHandle>,
}

/// 缩略图加载器 - 在后台线程加载缩略图
pub struct ThumbnailLoader {
    sender: Sender<ThumbnailRequest>,
    receiver: Receiver<ThumbnailResult>,
}

impl ThumbnailLoader {
    pub fn new(ctx: egui::Context) -> Self {
        let (request_tx, request_rx) = channel::<ThumbnailRequest>();
        let (result_tx, result_rx) = channel::<ThumbnailResult>();

        // 启动后台线程处理缩略图加载
        std::thread::spawn(move || {
            while let Ok(request) = request_rx.recv() {
                let ThumbnailRequest { path, index } = request;
                
                // 加载缩略图
                let texture = Self::load_thumbnail_internal(&path, &ctx);
                
                // 发送结果回主线程
                let _ = result_tx.send(ThumbnailResult { index, texture });
                
                // 触发重绘以更新UI
                ctx.request_repaint();
            }
        });

        Self {
            sender: request_tx,
            receiver: result_rx,
        }
    }

    fn load_thumbnail_internal(
        path: &PathBuf,
        ctx: &egui::Context,
    ) -> Option<egui::TextureHandle> {
        const THUMBNAIL_SIZE: u32 = 120;
        
        // 首先尝试使用 image::open 加载
        let img_result = image::open(path);
        
        let img = match img_result {
            Ok(img) => img,
            Err(e) => {
                debug!("缩略图自动格式检测失败 {:?}: {}，尝试备用方法...", path, e);
                
                match std::fs::read(path) {
                    Ok(data) => {
                        match image::load_from_memory(&data) {
                            Ok(img) => {
                                info!("缩略图使用备用方法成功加载: {:?}", path);
                                img
                            }
                            Err(e2) => {
                                error!("缩略图备用解码也失败 {:?}: {}", path, e2);
                                return None;
                            }
                        }
                    }
                    Err(io_err) => {
                        error!("无法读取缩略图文件 {:?}: {}", path, io_err);
                        return None;
                    }
                }
            }
        };
        
        // 调整为缩略图大小
        let resized = img.resize(
            THUMBNAIL_SIZE,
            THUMBNAIL_SIZE,
            image::imageops::FilterType::Lanczos3,
        );
        
        let rgba = resized.to_rgba8();
        let size = [rgba.width() as usize, rgba.height() as usize];
        let pixels = rgba.as_raw();
        
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels);
        let texture_name = format!("thumb_{}", path.file_name()?.to_string_lossy());
        
        Some(ctx.load_texture(
            texture_name,
            color_image,
            egui::TextureOptions::LINEAR,
        ))
    }

    /// 请求加载缩略图
    pub fn request(&self, index: usize, path: PathBuf) {
        let _ = self.sender.send(ThumbnailRequest { path, index });
    }

    /// 处理已完成的缩略图加载 - 返回处理的数量
    pub fn process_results(&self, images: &mut [GalleryImage]) -> usize {
        let mut count = 0;
        while let Ok(result) = self.receiver.try_recv() {
            if result.index < images.len() {
                images[result.index].thumbnail = result.texture;
                images[result.index].is_loading = false;
                count += 1;
            }
        }
        count
    }
}

/// 图库状态和渲染
pub struct Gallery {
    config: GalleryConfig,
    images: Vec<GalleryImage>,
    selected_index: Option<usize>,
    thumbnail_loader: Option<ThumbnailLoader>,
    items_per_row: usize,
}

#[derive(Clone)]
pub struct GalleryImage {
    pub path: std::path::PathBuf,
    pub thumbnail: Option<egui::TextureHandle>,
    pub is_loading: bool,
}

/// 键盘导航结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavAction {
    None,
    SelectAndOpen(usize),
}

impl Gallery {
    /// 使用给定配置创建新的图库
    pub fn new(config: GalleryConfig) -> Self {
        debug!("初始化图库，配置: {:?}", config);
        
        Self {
            config,
            images: Vec::new(),
            selected_index: None,
            thumbnail_loader: None,
            items_per_row: 0,
        }
    }

    /// 初始化缩略图加载器（需要在有 egui 上下文时调用）
    pub fn init_thumbnail_loader(&mut self, ctx: &egui::Context) {
        if self.thumbnail_loader.is_none() {
            self.thumbnail_loader = Some(ThumbnailLoader::new(ctx.clone()));
        }
    }

    /// 添加图像到图库
    pub fn add_image(&mut self, path: std::path::PathBuf) {
        let index = self.images.len();
        self.images.push(GalleryImage {
            path: path.clone(),
            thumbnail: None,
            is_loading: true,
        });
        
        // 异步请求加载缩略图
        if let Some(ref loader) = self.thumbnail_loader {
            loader.request(index, path);
        }
    }

    /// 处理异步加载的缩略图结果
    pub fn process_async_results(&mut self) {
        if let Some(ref loader) = self.thumbnail_loader {
            loader.process_results(&mut self.images);
        }
    }

    /// 从图库中移除指定索引的图像
    pub fn remove_image(&mut self, index: usize) -> Option<std::path::PathBuf> {
        if index < self.images.len() {
            let image = self.images.remove(index);
            // 更新选中索引
            if let Some(selected) = self.selected_index {
                if selected == index {
                    self.selected_index = None;
                } else if selected > index {
                    self.selected_index = Some(selected - 1);
                }
            }
            Some(image.path)
        } else {
            None
        }
    }

    /// 获取指定索引的图像路径
    pub fn get_image_path(&self, index: usize) -> Option<&std::path::Path> {
        self.images.get(index).map(|img| img.path.as_path())
    }

    /// 获取选中的图像路径
    pub fn get_selected_path(&self) -> Option<&std::path::Path> {
        self.selected_index.and_then(|idx| self.get_image_path(idx))
    }

    /// 选中指定索引的图像
    pub fn select_image(&mut self, index: usize) -> bool {
        if index < self.images.len() {
            self.selected_index = Some(index);
            debug!("选中图像，索引: {}", index);
            true
        } else {
            false
        }
    }

    /// 键盘导航 - 选择上一个图像
    pub fn select_prev(&mut self) -> bool {
        if let Some(selected) = self.selected_index {
            if selected > 0 {
                return self.select_image(selected - 1);
            }
        } else if !self.images.is_empty() {
            return self.select_image(self.images.len() - 1);
        }
        false
    }

    /// 键盘导航 - 选择下一个图像
    pub fn select_next(&mut self) -> bool {
        if let Some(selected) = self.selected_index {
            if selected < self.images.len() - 1 {
                return self.select_image(selected + 1);
            }
        } else if !self.images.is_empty() {
            return self.select_image(0);
        }
        false
    }

    /// 键盘导航 - 选择上一行
    pub fn select_up(&mut self) -> bool {
        if self.items_per_row == 0 {
            return false;
        }
        if let Some(selected) = self.selected_index {
            if selected >= self.items_per_row {
                return self.select_image(selected - self.items_per_row);
            }
        }
        false
    }

    /// 键盘导航 - 选择下一行
    pub fn select_down(&mut self) -> bool {
        if self.items_per_row == 0 {
            return false;
        }
        if let Some(selected) = self.selected_index {
            let new_index = selected + self.items_per_row;
            if new_index < self.images.len() {
                return self.select_image(new_index);
            }
        }
        false
    }

    /// 处理键盘输入，返回导航动作
    pub fn handle_keyboard(&mut self, ctx: &egui::Context) -> NavAction {
        let mut action = NavAction::None;
        
        // 方向键导航
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            if self.select_prev() {
                action = NavAction::SelectAndOpen(self.selected_index.unwrap_or(0));
            }
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            if self.select_next() {
                action = NavAction::SelectAndOpen(self.selected_index.unwrap_or(0));
            }
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            if self.select_up() {
                action = NavAction::SelectAndOpen(self.selected_index.unwrap_or(0));
            }
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            if self.select_down() {
                action = NavAction::SelectAndOpen(self.selected_index.unwrap_or(0));
            }
        }
        
        // Enter 键打开选中的图像
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            if let Some(selected) = self.selected_index {
                action = NavAction::SelectAndOpen(selected);
            }
        }
        
        action
    }

    /// 清除所有图像
    pub fn clear(&mut self) {
        self.images.clear();
        self.selected_index = None;
    }

    /// 渲染图库界面，返回点击的图像索引
    pub fn ui(&mut self, ui: &mut Ui) -> Option<usize> {
        // 处理异步加载结果
        self.process_async_results();
        
        let available_width = ui.available_width();
        let mut clicked_index: Option<usize> = None;
        
        // 基于配置计算每行项目数
        self.items_per_row = if self.config.items_per_row > 0 {
            self.config.items_per_row
        } else {
            // 基于可用宽度自动计算
            let item_width = self.config.thumbnail_size as f32 + self.config.grid_spacing;
            (available_width / item_width).max(1.0) as usize
        };

        let items_per_row = self.items_per_row;
        
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                // 使用配置的网格间距
                let spacing = self.config.grid_spacing;
                ui.spacing_mut().item_spacing = Vec2::new(spacing, spacing);

                // 创建网格布局
                egui::Grid::new("gallery_grid")
                    .num_columns(items_per_row)
                    .spacing([spacing, spacing])
                    .show(ui, |ui| {
                        for index in 0..self.images.len() {
                            let response = self.render_thumbnail(ui, index);
                            
                            if response.clicked() {
                                self.selected_index = Some(index);
                                clicked_index = Some(index);
                                debug!("点击选中图像，索引: {}", index);
                            }

                            // 每行 items_per_row 个项目后换行
                            if (index + 1) % items_per_row == 0 {
                                ui.end_row();
                            }
                        }
                    });
            });
        
        clicked_index
    }

    fn render_thumbnail(&mut self, ui: &mut Ui, index: usize) -> Response {
        let size = Vec2::splat(self.config.thumbnail_size as f32);
        let is_selected = self.selected_index == Some(index);
        
        // 确保图像存在
        if index >= self.images.len() {
            return ui.allocate_exact_size(size, egui::Sense::click()).1;
        }

        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // 带悬停/选中状态的背景
            let bg_color = if is_selected {
                Color32::from_rgb(52, 152, 219)  // 选中时的蓝色
            } else if response.hovered() {
                Color32::from_rgb(60, 60, 60)
            } else {
                Color32::from_rgb(40, 40, 40)
            };

            // 圆角（4px半径）
            painter.rect_filled(rect, 4.0, bg_color);

            // 选中边框
            if is_selected {
                painter.rect_stroke(rect, 4.0, egui::Stroke::new(2.0, Color32::WHITE));
                // 为选中项目添加微妙的阴影效果
                painter.rect_stroke(
                    rect.expand(2.0), 
                    4.0, 
                    egui::Stroke::new(1.0, Color32::from_rgba_premultiplied(52, 152, 219, 100))
                );
            }

            // 缩略图或占位符/加载动画
            if let Some(ref texture) = self.images[index].thumbnail {
                painter.image(
                    texture.id(), 
                    rect.shrink(4.0),  // 轻微内边距
                    Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), 
                    Color32::WHITE
                );
            } else if self.images[index].is_loading {
                // 加载动画 - 旋转的圆圈
                let center = rect.center();
                let radius = rect.width().min(rect.height()) * 0.15;
                let time = ui.ctx().input(|i| i.time);
                let angle = (time * 2.0) as f32;
                
                // 绘制加载圆圈
                for i in 0..8 {
                    let dot_angle = angle + i as f32 * std::f32::consts::PI / 4.0;
                    let dot_pos = center + Vec2::new(dot_angle.cos() * radius, dot_angle.sin() * radius);
                    let alpha = ((i as f32 / 8.0) * 255.0) as u8;
                    painter.circle_filled(dot_pos, 2.0, Color32::from_rgba_premultiplied(200, 200, 200, alpha));
                }
                
                // 加载文字
                painter.text(
                    center + Vec2::new(0.0, radius + 12.0),
                    egui::Align2::CENTER_CENTER,
                    "加载中...",
                    egui::FontId::proportional(10.0),
                    Color32::GRAY,
                );
            } else {
                // 加载失败或等待中的占位符
                let text = self.images[index]
                    .path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("未知");
                
                // 绘制文件图标占位符
                let icon_rect = rect.shrink(rect.width() * 0.3);
                painter.rect_stroke(icon_rect, 2.0, egui::Stroke::new(1.0, Color32::GRAY));
                
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    text,
                    egui::FontId::proportional(12.0),
                    Color32::GRAY,
                );
            }

            // 如果启用则显示文件名标签
            if self.config.show_filenames {
                let filename = self.images[index]
                    .path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("未知");
                
                // 截断长文件名
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

    /// 获取当前选中的图像索引
    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    /// 获取图库中的图像数量
    pub fn len(&self) -> usize {
        self.images.len()
    }

    /// 检查图库是否为空
    pub fn is_empty(&self) -> bool {
        self.images.is_empty()
    }

    /// 获取图像列表（只读）
    pub fn images(&self) -> &[GalleryImage] {
        &self.images
    }

    /// 获取配置
    pub fn config(&self) -> &GalleryConfig {
        &self.config
    }

    /// 更新配置
    pub fn update_config(&mut self, config: GalleryConfig) {
        self.config = config;
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
    fn test_gallery_with_custom_config() {
        let config = GalleryConfig {
            thumbnail_size: 150,
            items_per_row: 6,
            grid_spacing: 16.0,
            show_filenames: true,
        };
        let gallery = Gallery::new(config);
        assert!(gallery.is_empty());
        assert_eq!(gallery.config().thumbnail_size, 150);
    }

    #[test]
    fn test_gallery_add_image() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        gallery.add_image(std::path::PathBuf::from("test.png"));
        assert_eq!(gallery.len(), 1);
        assert!(!gallery.is_empty());
    }

    #[test]
    fn test_gallery_add_multiple_images() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        gallery.add_image(std::path::PathBuf::from("test1.png"));
        gallery.add_image(std::path::PathBuf::from("test2.png"));
        gallery.add_image(std::path::PathBuf::from("test3.png"));
        assert_eq!(gallery.len(), 3);
    }

    #[test]
    fn test_gallery_clear() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        gallery.add_image(std::path::PathBuf::from("test1.png"));
        gallery.add_image(std::path::PathBuf::from("test2.png"));
        gallery.clear();
        assert!(gallery.is_empty());
        assert_eq!(gallery.selected_index(), None);
    }

    #[test]
    fn test_gallery_select_image() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        gallery.add_image(std::path::PathBuf::from("test1.png"));
        gallery.add_image(std::path::PathBuf::from("test2.png"));
        assert!(gallery.select_image(0));
        assert_eq!(gallery.selected_index(), Some(0));
        assert!(gallery.select_image(1));
        assert_eq!(gallery.selected_index(), Some(1));
    }

    #[test]
    fn test_gallery_select_invalid_index() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        gallery.add_image(std::path::PathBuf::from("test1.png"));
        assert!(!gallery.select_image(5));
        assert_eq!(gallery.selected_index(), None);
    }

    #[test]
    fn test_gallery_select_next() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        gallery.add_image(std::path::PathBuf::from("test1.png"));
        gallery.add_image(std::path::PathBuf::from("test2.png"));
        gallery.add_image(std::path::PathBuf::from("test3.png"));
        assert!(gallery.select_image(0));
        assert!(gallery.select_next());
        assert_eq!(gallery.selected_index(), Some(1));
        assert!(gallery.select_next());
        assert_eq!(gallery.selected_index(), Some(2));
        assert!(!gallery.select_next());
        assert_eq!(gallery.selected_index(), Some(2));
    }

    #[test]
    fn test_gallery_select_prev() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        gallery.add_image(std::path::PathBuf::from("test1.png"));
        gallery.add_image(std::path::PathBuf::from("test2.png"));
        gallery.add_image(std::path::PathBuf::from("test3.png"));
        assert!(gallery.select_image(2));
        assert!(gallery.select_prev());
        assert_eq!(gallery.selected_index(), Some(1));
        assert!(gallery.select_prev());
        assert_eq!(gallery.selected_index(), Some(0));
        assert!(!gallery.select_prev());
        assert_eq!(gallery.selected_index(), Some(0));
    }

    #[test]
    fn test_gallery_navigate_empty() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        assert!(!gallery.select_next());
        assert!(!gallery.select_prev());
        assert!(!gallery.select_up());
        assert!(!gallery.select_down());
    }

    #[test]
    fn test_gallery_select_up_down() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        for i in 0..10 {
            gallery.add_image(std::path::PathBuf::from(format!("test{}.png", i)));
        }
        gallery.items_per_row = 3;
        gallery.select_image(4);
        assert!(gallery.select_up());
        assert_eq!(gallery.selected_index(), Some(1));
        gallery.select_image(1);
        assert!(gallery.select_down());
        assert_eq!(gallery.selected_index(), Some(4));
    }

    #[test]
    fn test_gallery_get_image_path() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        gallery.add_image(std::path::PathBuf::from("/path/to/test.png"));
        let path = gallery.get_image_path(0);
        assert!(path.is_some());
        assert_eq!(path.unwrap().to_str().unwrap(), "/path/to/test.png");
    }

    #[test]
    fn test_gallery_get_selected_path() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        gallery.add_image(std::path::PathBuf::from("/path/to/selected.png"));
        gallery.add_image(std::path::PathBuf::from("/path/to/other.png"));
        gallery.select_image(0);
        let path = gallery.get_selected_path();
        assert!(path.is_some());
        assert!(path.unwrap().to_str().unwrap().contains("selected"));
    }

    #[test]
    fn test_gallery_remove_image() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        gallery.add_image(std::path::PathBuf::from("test1.png"));
        gallery.add_image(std::path::PathBuf::from("test2.png"));
        let removed = gallery.remove_image(0);
        assert!(removed.is_some());
        assert_eq!(gallery.len(), 1);
    }

    #[test]
    fn test_gallery_remove_selected() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        gallery.add_image(std::path::PathBuf::from("test1.png"));
        gallery.add_image(std::path::PathBuf::from("test2.png"));
        gallery.select_image(0);
        gallery.remove_image(0);
        assert_eq!(gallery.selected_index(), None);
    }

    #[test]
    fn test_gallery_update_config() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        let new_config = GalleryConfig {
            thumbnail_size: 200,
            items_per_row: 8,
            grid_spacing: 20.0,
            show_filenames: false,
        };
        gallery.update_config(new_config);
        assert_eq!(gallery.config().thumbnail_size, 200);
    }

    #[test]
    fn test_gallery_image_new() {
        let image = GalleryImage {
            path: std::path::PathBuf::from("test.png"),
            thumbnail: None,
            is_loading: true,
        };
        assert!(image.thumbnail.is_none());
        assert!(image.is_loading);
    }

    #[test]
    fn test_nav_action_none() {
        let action = NavAction::None;
        assert_eq!(action, NavAction::None);
    }

    #[test]
    fn test_nav_action_select_and_open() {
        let action = NavAction::SelectAndOpen(5);
        assert_eq!(action, NavAction::SelectAndOpen(5));
    }

    #[test]
    fn test_select_up_at_top() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        for i in 0..5 {
            gallery.add_image(std::path::PathBuf::from(format!("test{}.png", i)));
        }
        gallery.items_per_row = 3;
        gallery.select_image(1);
        assert!(!gallery.select_up());
        assert_eq!(gallery.selected_index(), Some(1));
    }

    #[test]
    fn test_select_down_at_bottom() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        for i in 0..5 {
            gallery.add_image(std::path::PathBuf::from(format!("test{}.png", i)));
        }
        gallery.items_per_row = 3;
        gallery.select_image(4);
        assert!(!gallery.select_down());
        assert_eq!(gallery.selected_index(), Some(4));
    }

    #[test]
    fn test_single_image_navigation() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        gallery.add_image(std::path::PathBuf::from("test.png"));
        assert!(gallery.select_image(0));
        assert!(!gallery.select_next());
        assert!(!gallery.select_prev());
        assert!(!gallery.select_up());
        assert!(!gallery.select_down());
    }

    #[test]
    fn test_gallery_navigate_no_selection() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        gallery.add_image(std::path::PathBuf::from("test1.png"));
        gallery.add_image(std::path::PathBuf::from("test2.png"));
        assert!(gallery.select_next());
        assert_eq!(gallery.selected_index(), Some(0));
        let mut gallery2 = Gallery::new(config);
        gallery2.add_image(std::path::PathBuf::from("test1.png"));
        gallery2.add_image(std::path::PathBuf::from("test2.png"));
        assert!(gallery2.select_prev());
        assert_eq!(gallery2.selected_index(), Some(1));
    }

    #[test]
    fn test_items_per_row_calculation() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        for i in 0..10 {
            gallery.add_image(std::path::PathBuf::from(format!("test{}.png", i)));
        }
        assert_eq!(gallery.items_per_row, 0);
        gallery.items_per_row = 3;
        gallery.select_image(0);
        assert!(gallery.select_down());
        assert_eq!(gallery.selected_index(), Some(3));
    }

    #[test]
    fn test_gallery_remove_after_selected() {
        let config = GalleryConfig::default();
        let mut gallery = Gallery::new(config.clone());
        gallery.add_image(std::path::PathBuf::from("test1.png"));
        gallery.add_image(std::path::PathBuf::from("test2.png"));
        gallery.add_image(std::path::PathBuf::from("test3.png"));
        gallery.select_image(2);
        gallery.remove_image(0);
        assert_eq!(gallery.selected_index(), Some(1));
    }
}
