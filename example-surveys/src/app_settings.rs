//! Application settings example types (suggestions demo)
//!
//! Demonstrates:
//! - Setting default/suggested values that users can modify
//! - Assumptions that skip questions entirely
//! - Editing existing data

use derive_survey::Survey;

/// Application settings with suggested defaults.
#[derive(Debug, Clone, Survey)]
pub struct AppSettings {
    #[ask("Application name:")]
    pub app_name: String,

    #[ask("Port number:")]
    #[min(1024)]
    #[max(65535)]
    pub port: i64,

    #[ask("Max connections:")]
    #[min(1)]
    #[max(10000)]
    pub max_connections: i64,

    #[ask("Timeout in seconds:")]
    #[min(1)]
    #[max(300)]
    pub timeout: i64,

    #[ask("Enable debug mode:")]
    pub debug_mode: bool,

    #[ask("Log file path:")]
    pub log_path: String,
}
