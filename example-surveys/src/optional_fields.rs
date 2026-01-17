//! Optional fields example types
//!
//! Demonstrates:
//! - Option<T> for optional fields
//! - Using builder to suggest or assume optional values

use elicitor::Survey;
use std::path::PathBuf;

#[derive(Survey, Debug)]
pub struct ProjectConfig {
    #[ask("Project name:")]
    pub name: String,

    #[ask("Project description (optional):")]
    pub description: Option<String>,

    #[ask("License file path (optional):")]
    pub license_path: Option<PathBuf>,

    #[ask("Initial version number (optional):")]
    pub version: Option<String>,

    #[ask("Enable debug mode?")]
    pub debug: bool,

    #[ask("Log level (optional, 0-5):")]
    #[min(0)]
    #[max(5)]
    pub log_level: Option<u32>,
}
