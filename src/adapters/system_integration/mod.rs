//! 系统集成模块 - 提供系统级集成功能
//!
//! 包括：
//! - Windows: 注册表操作（右键菜单、文件关联）
//! - macOS: LSRegister（文件关联）
//! - Linux: xdg-mime/.desktop 文件

use crate::core::domain::Language;

mod platform;

/// 系统集成状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IntegrationStatus {
    /// 是否已添加到右键菜单（Windows）
    pub context_menu_registered: bool,
    /// 是否已设为默认图片查看器
    pub default_app_registered: bool,
}

/// 获取当前系统集成状态
pub fn get_integration_status() -> IntegrationStatus {
    platform::get_integration_status()
}

/// 注册右键菜单（Windows 专用）
///
/// # 错误
/// 在非 Windows 平台上返回错误
pub fn register_context_menu() -> anyhow::Result<()> {
    platform::register_context_menu()
}

/// 注销右键菜单（Windows 专用）
///
/// # 错误
/// 在非 Windows 平台上返回错误
pub fn unregister_context_menu() -> anyhow::Result<()> {
    platform::unregister_context_menu()
}

/// 设为默认图片查看器
pub fn set_as_default_app() -> anyhow::Result<()> {
    platform::set_as_default_app()
}

/// 获取本地化错误信息
pub fn get_error_message(error: &anyhow::Error, language: Language) -> String {
    match language {
        Language::Chinese => format!("操作失败: {}", error),
        Language::English => format!("Operation failed: {}", error),
    }
}

/// 获取本地化成功信息
pub fn get_success_message(operation: &str, language: Language) -> String {
    match language {
        Language::Chinese => format!("{}成功", operation),
        Language::English => format!("{} successfully", operation),
    }
}
