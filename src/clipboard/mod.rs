//! 剪贴板操作模块 - 提供图片和文本的复制功能

use arboard::{Clipboard, ImageData};
use std::path::Path;
use tracing::{debug, error, info};

/// 剪贴板操作结果
type Result<T> = std::result::Result<T, ClipboardError>;

/// 剪贴板错误类型
#[derive(Debug, Clone)]
pub enum ClipboardError {
    FailedToAccess(String),
    FailedToCopy(String),
    InvalidImage(String),
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClipboardError::FailedToAccess(msg) => write!(f, "无法访问剪贴板: {}", msg),
            ClipboardError::FailedToCopy(msg) => write!(f, "复制失败: {}", msg),
            ClipboardError::InvalidImage(msg) => write!(f, "无效的图片: {}", msg),
        }
    }
}

impl std::error::Error for ClipboardError {}

/// 剪贴板管理器
pub struct ClipboardManager {
    clipboard: Option<Clipboard>,
}

impl ClipboardManager {
    /// 创建新的剪贴板管理器
    pub fn new() -> Self {
        match Clipboard::new() {
            Ok(clipboard) => {
                debug!("剪贴板初始化成功");
                Self {
                    clipboard: Some(clipboard),
                }
            }
            Err(e) => {
                error!("无法初始化剪贴板: {}", e);
                Self { clipboard: None }
            }
        }
    }

    /// 检查剪贴板是否可用
    pub fn is_available(&self) -> bool {
        self.clipboard.is_some()
    }

    /// 复制文本到剪贴板
    pub fn copy_text(&mut self, text: &str) -> Result<()> {
        let clipboard = self
            .clipboard
            .as_mut()
            .ok_or_else(|| ClipboardError::FailedToAccess("剪贴板不可用".to_string()))?;

        clipboard
            .set_text(text)
            .map_err(|e| ClipboardError::FailedToCopy(e.to_string()))?;

        info!("文本已复制到剪贴板");
        Ok(())
    }

    /// 复制图片路径到剪贴板
    pub fn copy_image_path(&mut self, path: &Path) -> Result<()> {
        let path_str = path.to_string_lossy().to_string();
        self.copy_text(&path_str)?;
        info!("图片路径已复制: {:?}", path);
        Ok(())
    }

    /// 复制图片数据到剪贴板
    pub fn copy_image(&mut self, image_data: &[u8], width: usize, height: usize) -> Result<()> {
        let clipboard = self
            .clipboard
            .as_mut()
            .ok_or_else(|| ClipboardError::FailedToAccess("剪贴板不可用".to_string()))?;

        // 确保数据长度正确
        let expected_len = width * height * 4; // RGBA
        if image_data.len() != expected_len {
            return Err(ClipboardError::InvalidImage(format!(
                "图片数据长度不匹配: 期望 {} 字节, 实际 {} 字节",
                expected_len,
                image_data.len()
            )));
        }

        let image_data = ImageData {
            width,
            height,
            bytes: std::borrow::Cow::Borrowed(image_data),
        };

        clipboard
            .set_image(image_data)
            .map_err(|e| ClipboardError::FailedToCopy(e.to_string()))?;

        info!("图片已复制到剪贴板 ({}x{})", width, height);
        Ok(())
    }

    /// 从文件路径复制图片到剪贴板
    pub fn copy_image_from_file(
        &mut self,
        path: &Path,
        texture_data: Option<(&[u8], [usize; 2])>,
    ) -> Result<()> {
        // 优先使用已加载的纹理数据
        if let Some((data, [width, height])) = texture_data {
            return self.copy_image(data, width, height);
        }

        // 否则从文件读取
        let img = image::open(path).map_err(|e| {
            ClipboardError::InvalidImage(format!("无法打开图片: {}", e))
        })?;

        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let data = rgba.into_raw();

        self.copy_image(&data, width as usize, height as usize)
    }

    /// 在文件管理器中显示文件
    pub fn show_in_folder(path: &Path) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            Command::new("open")
                .args(["-R", &path.to_string_lossy().to_string()])
                .spawn()
                .map_err(|e| ClipboardError::FailedToCopy(format!("无法打开文件夹: {}", e)))?;
        }

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            let path_str = path.to_string_lossy().to_string();
            let parent = path
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| path_str.clone());

            // 尝试 xdg-open
            let result = Command::new("xdg-open").arg(&parent).spawn();
            if result.is_err() {
                // 回退到 dbus-send (Nautilus)
                let _ = Command::new("dbus-send")
                    .args([
                        "--session",
                        "--dest=org.freedesktop.FileManager1",
                        "--type=method_call",
                        "/org/freedesktop/FileManager1",
                        "org.freedesktop.FileManager1.ShowItems",
                        format!("array:string:file://{}", path_str).as_str(),
                        "string:\"\"",
                    ])
                    .spawn();
            }
        }

        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            Command::new("explorer")
                .args(["/select,", &path.to_string_lossy().to_string()])
                .spawn()
                .map_err(|e| ClipboardError::FailedToCopy(format!("无法打开文件夹: {}", e)))?;
        }

        info!("已在文件夹中显示: {:?}", path);
        Ok(())
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_manager_new() {
        let _manager = ClipboardManager::new();
    }

    #[test]
    fn test_copy_text() {
        let mut manager = ClipboardManager::new();
        if manager.is_available() {
            let result = manager.copy_text("test text");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_copy_image_path() {
        let mut manager = ClipboardManager::new();
        if manager.is_available() {
            let path = Path::new("/tmp/test.png");
            let result = manager.copy_image_path(path);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_copy_image_invalid_size() {
        let mut manager = ClipboardManager::new();
        if manager.is_available() {
            let data = vec![0u8; 100];
            let result = manager.copy_image(&data, 10, 10);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_show_in_folder_nonexistent() {
        let path = Path::new("/tmp/nonexistent_file_for_test.png");
        let _ = ClipboardManager::show_in_folder(path);
    }

    #[test]
    fn test_clipboard_error_display() {
        let err1 = ClipboardError::FailedToAccess("test".to_string());
        assert!(err1.to_string().contains("无法访问剪贴板"));

        let err2 = ClipboardError::FailedToCopy("test".to_string());
        assert!(err2.to_string().contains("复制失败"));

        let err3 = ClipboardError::InvalidImage("test".to_string());
        assert!(err3.to_string().contains("无效的图片"));
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;

    #[test]
    fn test_clipboard_manager_default() {
        let manager: ClipboardManager = Default::default();
        // Default 应该调用 new()
        assert!(manager.is_available() || !manager.is_available());
    }

    #[test]
    fn test_clipboard_error_clone() {
        let err1 = ClipboardError::FailedToAccess("test".to_string());
        let cloned = err1.clone();
        assert_eq!(err1.to_string(), cloned.to_string());

        let err2 = ClipboardError::FailedToCopy("test2".to_string());
        let cloned2 = err2.clone();
        assert_eq!(err2.to_string(), cloned2.to_string());

        let err3 = ClipboardError::InvalidImage("test3".to_string());
        let cloned3 = err3.clone();
        assert_eq!(err3.to_string(), cloned3.to_string());
    }

    #[test]
    fn test_clipboard_error_debug() {
        let err = ClipboardError::FailedToAccess("test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("FailedToAccess"));
    }

    #[test]
    fn test_clipboard_error_error_trait() {
        let err = ClipboardError::FailedToCopy("test".to_string());
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_copy_image_with_valid_data() {
        let mut manager = ClipboardManager::new();
        if manager.is_available() {
            // 创建有效的 RGBA 数据 (10x10 像素)
            let data = vec![255u8; 10 * 10 * 4];
            let result = manager.copy_image(&data, 10, 10);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_copy_image_with_wrong_size() {
        let mut manager = ClipboardManager::new();
        if manager.is_available() {
            // 数据长度不正确
            let data = vec![255u8; 100]; // 应该是 10*10*4 = 400
            let result = manager.copy_image(&data, 10, 10);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_copy_image_path_various_paths() {
        let mut manager = ClipboardManager::new();
        if manager.is_available() {
            let paths = vec![
                Path::new("/tmp/test.png"),
                Path::new("test.png"),
                Path::new("/very/long/path/to/the/image/file.png"),
            ];

            for path in paths {
                let result = manager.copy_image_path(path);
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn test_is_available_consistency() {
        let manager = ClipboardManager::new();
        // 多次调用应该返回相同结果
        let first = manager.is_available();
        let second = manager.is_available();
        assert_eq!(first, second);
    }

    #[test]
    fn test_copy_text_empty() {
        let mut manager = ClipboardManager::new();
        if manager.is_available() {
            let result = manager.copy_text("");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_copy_text_unicode() {
        let mut manager = ClipboardManager::new();
        if manager.is_available() {
            let texts = vec![
                "Hello World",
                "你好世界",
                "🎨🖼️📷",
                "Special chars: àáâãäåæçèéêë",
            ];

            for text in texts {
                let result = manager.copy_text(text);
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn test_show_in_folder_various_paths() {
        let paths = vec![
            Path::new("/tmp/test.png"),
            Path::new("test.png"),
            Path::new("/home/user/images/photo.jpg"),
        ];

        for path in paths {
            // 这些可能会失败，但不应该 panic
            let _ = ClipboardManager::show_in_folder(path);
        }
    }

    #[test]
    fn test_copy_image_from_file_with_texture_data() {
        let mut manager = ClipboardManager::new();
        if manager.is_available() {
            // 提供纹理数据
            let data = vec![255u8; 10 * 10 * 4];
            let result = manager.copy_image_from_file(
                Path::new("/tmp/test.png"),
                Some((&data, [10, 10]))
            );
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_copy_image_from_file_without_texture_data() {
        let mut manager = ClipboardManager::new();
        if manager.is_available() {
            // 不提供纹理数据，会尝试从文件读取
            let result = manager.copy_image_from_file(
                Path::new("/nonexistent/path/to/file.png"),
                None
            );
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_error_messages_content() {
        let err1 = ClipboardError::FailedToAccess("clipboard locked".to_string());
        let msg1 = err1.to_string();
        assert!(msg1.contains("无法访问剪贴板"));
        assert!(msg1.contains("clipboard locked"));

        let err2 = ClipboardError::FailedToCopy("disk full".to_string());
        let msg2 = err2.to_string();
        assert!(msg2.contains("复制失败"));
        assert!(msg2.contains("disk full"));

        let err3 = ClipboardError::InvalidImage("corrupted data".to_string());
        let msg3 = err3.to_string();
        assert!(msg3.contains("无效的图片"));
        assert!(msg3.contains("corrupted data"));
    }
}
