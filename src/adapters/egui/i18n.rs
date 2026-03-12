//! 国际化 (i18n) 模块 - 提供多语言支持

use crate::core::domain::Language;

/// 获取翻译文本
/// 
/// # 参数
/// - `key`: 文本键名
/// - `lang`: 目标语言
/// 
/// # 返回值
/// 返回对应的翻译文本，如果没有找到则返回键名本身
pub fn get_text(key: &str, lang: Language) -> &str {
    match (key, lang) {
        // 菜单
        ("menu_file", Language::Chinese) => "文件",
        ("menu_file", Language::English) => "File",
        ("menu_view", Language::Chinese) => "视图",
        ("menu_view", Language::English) => "View",
        ("menu_image", Language::Chinese) => "图片",
        ("menu_image", Language::English) => "Image",
        ("menu_help", Language::Chinese) => "帮助",
        ("menu_help", Language::English) => "Help",

        // 菜单项
        ("open", Language::Chinese) => "打开...",
        ("open", Language::English) => "Open...",
        ("exit", Language::Chinese) => "退出",
        ("exit", Language::English) => "Exit",
        ("gallery", Language::Chinese) => "图库视图",
        ("gallery", Language::English) => "Gallery",
        ("viewer", Language::Chinese) => "查看器",
        ("viewer", Language::English) => "Viewer",
        ("fullscreen", Language::Chinese) => "全屏切换",
        ("fullscreen", Language::English) => "Fullscreen",
        ("zoom_in", Language::Chinese) => "放大",
        ("zoom_in", Language::English) => "Zoom In",
        ("zoom_out", Language::Chinese) => "缩小",
        ("zoom_out", Language::English) => "Zoom Out",
        ("reset_zoom", Language::Chinese) => "重置缩放",
        ("reset_zoom", Language::English) => "Reset Zoom",
        ("fit_to_window", Language::Chinese) => "适应窗口",
        ("fit_to_window", Language::English) => "Fit to Window",
        ("original_size", Language::Chinese) => "原始尺寸",
        ("original_size", Language::English) => "Original Size",
        ("about", Language::Chinese) => "关于",
        ("about", Language::English) => "About",

        // 按钮
        ("close", Language::Chinese) => "关闭",
        ("close", Language::English) => "Close",

        // 提示
        ("drag_hint", Language::Chinese) => "释放以打开图片",
        ("drag_hint", Language::English) => "Drop to open image",
        ("no_image", Language::Chinese) => "未选择图像\n按 Ctrl+O 打开图像或从图库中选择\n也可以直接拖拽图像到窗口",
        ("no_image", Language::English) => "No image selected\nPress Ctrl+O to open or select from gallery\nYou can also drag and drop images",
        ("empty_gallery", Language::Chinese) => "暂无图片\n\n按 Ctrl+O 打开图片或拖拽图片到窗口",
        ("empty_gallery", Language::English) => "No images\n\nPress Ctrl+O or drag and drop images",
        ("loading", Language::Chinese) => "加载中...",
        ("loading", Language::English) => "Loading...",

        // 右键菜单
        ("copy_image", Language::Chinese) => "复制图片",
        ("copy_image", Language::English) => "Copy Image",
        ("copy_path", Language::Chinese) => "复制文件路径",
        ("copy_path", Language::English) => "Copy Path",
        ("show_in_folder", Language::Chinese) => "在文件夹中显示",
        ("show_in_folder", Language::English) => "Show in Folder",

        // 信息面板
        ("image_info", Language::Chinese) => "图像信息",
        ("image_info", Language::English) => "Image Info",
        ("file_info", Language::Chinese) => "文件信息",
        ("file_info", Language::English) => "File Info",
        ("file_name", Language::Chinese) => "文件名",
        ("file_name", Language::English) => "File Name",
        ("dimensions", Language::Chinese) => "尺寸",
        ("dimensions", Language::English) => "Dimensions",
        ("file_size", Language::Chinese) => "文件大小",
        ("file_size", Language::English) => "File Size",
        ("modified_time", Language::Chinese) => "修改时间",
        ("modified_time", Language::English) => "Modified Time",
        ("format", Language::Chinese) => "格式",
        ("format", Language::English) => "Format",
        ("megapixels", Language::Chinese) => "百万像素",
        ("megapixels", Language::English) => "Megapixels",
        ("bit_depth", Language::Chinese) => "位深度",
        ("bit_depth", Language::English) => "Bit Depth",
        ("color_space", Language::Chinese) => "色彩空间",
        ("color_space", Language::English) => "Color Space",
        ("no_exif", Language::Chinese) => "无EXIF数据",
        ("no_exif", Language::English) => "No EXIF data",
        ("loading_exif", Language::Chinese) => "正在加载EXIF数据...",
        ("loading_exif", Language::English) => "Loading EXIF data...",

        // EXIF 信息
        ("exif_info", Language::Chinese) => "EXIF 信息",
        ("exif_info", Language::English) => "EXIF Info",
        ("camera", Language::Chinese) => "相机",
        ("camera", Language::English) => "Camera",
        ("lens", Language::Chinese) => "镜头",
        ("lens", Language::English) => "Lens",
        ("date_time", Language::Chinese) => "拍摄时间",
        ("date_time", Language::English) => "Date Taken",
        ("iso", Language::Chinese) => "ISO",
        ("iso", Language::English) => "ISO",
        ("aperture", Language::Chinese) => "光圈",
        ("aperture", Language::English) => "Aperture",
        ("shutter", Language::Chinese) => "快门",
        ("shutter", Language::English) => "Shutter",
        ("focal_length", Language::Chinese) => "焦距",
        ("focal_length", Language::English) => "Focal Length",
        ("gps_latitude", Language::Chinese) => "纬度",
        ("gps_latitude", Language::English) => "Latitude",
        ("gps_longitude", Language::Chinese) => "经度",
        ("gps_longitude", Language::English) => "Longitude",
        ("unknown", Language::Chinese) => "未知",
        ("unknown", Language::English) => "Unknown",

        // 快捷键帮助
        ("shortcuts_title", Language::Chinese) => "快捷键帮助",
        ("shortcuts_title", Language::English) => "Keyboard Shortcuts",
        ("shortcuts_keyboard", Language::Chinese) => "键盘快捷键",
        ("shortcuts_keyboard", Language::English) => "Shortcuts",
        ("navigation", Language::Chinese) => "导航",
        ("navigation", Language::English) => "Navigation",
        ("zoom", Language::Chinese) => "缩放",
        ("zoom", Language::English) => "Zoom",
        ("view", Language::Chinese) => "视图",
        ("view", Language::English) => "View",
        ("other", Language::Chinese) => "其他",
        ("other", Language::English) => "Other",
        ("file_ops", Language::Chinese) => "文件",
        ("file_ops", Language::English) => "File",

        // 快捷键描述
        ("shortcut_open", Language::Chinese) => "打开图像/文件夹",
        ("shortcut_open", Language::English) => "Open image/folder",
        ("shortcut_exit_fullscreen", Language::Chinese) => "退出全屏 / 关闭面板",
        ("shortcut_exit_fullscreen", Language::English) => "Exit fullscreen / close panel",
        ("shortcut_prev_next", Language::Chinese) => "切换到上/下一张图片",
        ("shortcut_prev_next", Language::English) => "Previous/next image",
        ("shortcut_toggle_view", Language::Chinese) => "切换画廊/查看器视图",
        ("shortcut_toggle_view", Language::English) => "Toggle gallery/viewer",
        ("shortcut_fullscreen", Language::Chinese) => "全屏切换",
        ("shortcut_fullscreen", Language::English) => "Toggle fullscreen",
        ("shortcut_zoom_in", Language::Chinese) => "放大",
        ("shortcut_zoom_in", Language::English) => "Zoom in",
        ("shortcut_zoom_out", Language::Chinese) => "缩小",
        ("shortcut_zoom_out", Language::English) => "Zoom out",
        ("shortcut_fit_window", Language::Chinese) => "适应窗口",
        ("shortcut_fit_window", Language::English) => "Fit to window",
        ("shortcut_original", Language::Chinese) => "1:1 原始尺寸",
        ("shortcut_original", Language::English) => "Original size (1:1)",
        ("shortcut_info_panel", Language::Chinese) => "显示/隐藏信息面板",
        ("shortcut_info_panel", Language::English) => "Toggle info panel",
        ("shortcut_dbl_click", Language::Chinese) => "全屏切换",
        ("shortcut_dbl_click", Language::English) => "Toggle fullscreen",
        ("shortcut_help", Language::Chinese) => "显示/隐藏此帮助面板",
        ("shortcut_help", Language::English) => "Toggle this help panel",

        // 关于窗口
        ("about_title", Language::Chinese) => "关于",
        ("about_title", Language::English) => "About",
        ("version", Language::Chinese) => "版本",
        ("version", Language::English) => "Version",
        ("license", Language::Chinese) => "许可证",
        ("license", Language::English) => "License",

        // 语言切换
        ("language", Language::Chinese) => "语言",
        ("language", Language::English) => "Language",

        // 缩略图大小提示
        ("thumbnail_size", Language::Chinese) => "缩略图",
        ("thumbnail_size", Language::English) => "Thumbnail",

        // 常用标签
        ("common", Language::Chinese) => "常用",
        ("common", Language::English) => "Common",
        ("actions", Language::Chinese) => "操作",
        ("actions", Language::English) => "Actions",
        ("view_mode", Language::Chinese) => "视图模式",
        ("view_mode", Language::English) => "View Mode",
        ("display", Language::Chinese) => "显示",
        ("display", Language::English) => "Display",
        ("previous", Language::Chinese) => "上一张",
        ("previous", Language::English) => "Previous",
        ("next", Language::Chinese) => "下一张",
        ("next", Language::English) => "Next",
        ("about_app", Language::Chinese) => "关于 OAS Image Viewer",
        ("about_app", Language::English) => "About OAS Image Viewer",

        // 默认情况：返回键名
        _ => key,
    }
}

/// 获取当前语言的文本（便捷函数）
/// 
/// 这个函数会在未来从全局状态中读取当前语言设置
/// 目前作为过渡，先根据 crate::is_chinese_supported() 来判断
pub fn t(key: &str) -> &str {
    use crate::core::domain::Language;
    let lang = if crate::is_chinese_supported() {
        Language::Chinese
    } else {
        Language::English
    };
    get_text(key, lang)
}

/// 为特定语言格式化缩略图大小提示
pub fn format_thumbnail_hint(size: u32, lang: Language) -> String {
    match lang {
        Language::Chinese => format!("缩略图: {}px", size),
        Language::English => format!("Thumbnail: {}px", size),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_text_menu() {
        assert_eq!(get_text("menu_file", Language::Chinese), "文件");
        assert_eq!(get_text("menu_file", Language::English), "File");
        assert_eq!(get_text("menu_view", Language::Chinese), "视图");
        assert_eq!(get_text("menu_view", Language::English), "View");
    }

    #[test]
    fn test_get_text_buttons() {
        assert_eq!(get_text("open", Language::Chinese), "打开...");
        assert_eq!(get_text("open", Language::English), "Open...");
        assert_eq!(get_text("close", Language::Chinese), "关闭");
        assert_eq!(get_text("close", Language::English), "Close");
    }

    #[test]
    fn test_get_text_unknown_key() {
        assert_eq!(get_text("unknown_key", Language::Chinese), "unknown_key");
        assert_eq!(get_text("unknown_key", Language::English), "unknown_key");
    }

    #[test]
    fn test_format_thumbnail_hint() {
        assert_eq!(format_thumbnail_hint(100, Language::Chinese), "缩略图: 100px");
        assert_eq!(format_thumbnail_hint(100, Language::English), "Thumbnail: 100px");
    }

    #[test]
    fn test_all_menu_keys_exist() {
        // 确保主要菜单键都有翻译
        let keys = [
            "menu_file", "menu_view", "menu_image", "menu_help",
            "open", "exit", "gallery", "viewer", "fullscreen",
            "about", "close", "drag_hint", "no_image",
            "image_info", "file_name", "dimensions", "file_size",
            "shortcuts_title", "navigation", "zoom", "view", "other",
        ];
        
        for key in &keys {
            let chinese = get_text(key, Language::Chinese);
            let english = get_text(key, Language::English);
            assert_ne!(chinese, *key, "Chinese translation missing for: {}", key);
            assert_ne!(english, *key, "English translation missing for: {}", key);
        }
    }
}
