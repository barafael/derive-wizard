//! Server Configuration Wizard 🖥️
//!
//! A technical example for configuring a server deployment with validation.
//!
//! Run with: cargo run --example ratatui_server_config --features ratatui-backend

use derive_wizard::Wizard;

#[derive(Debug, Wizard)]
#[allow(dead_code)]
enum Environment {
    Development,
    Staging,
    Production,
}

#[derive(Debug, Wizard)]
#[allow(dead_code)]
enum DatabaseType {
    PostgreSQL,
    MySQL,
    SQLite,
    MongoDB,
}

#[derive(Debug, Wizard)]
#[allow(dead_code)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Validates IP address or hostname format
pub fn validate_host(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Host cannot be empty".to_string());
    }
    // Allow 'localhost' or IP-like patterns
    if trimmed == "localhost" {
        return Ok(());
    }
    // Basic IP validation - 4 octets
    let parts: Vec<&str> = trimmed.split('.').collect();
    if parts.len() == 4 && parts.iter().all(|p| p.parse::<u8>().is_ok()) {
        return Ok(());
    }
    // Accept hostnames with at least one dot or localhost
    if trimmed
        .chars()
        .all(|c| c.is_alphanumeric() || c == '.' || c == '-')
    {
        return Ok(());
    }
    Err("Please enter a valid hostname or IP address".to_string())
}

/// Validates bind address
pub fn validate_bind_address(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Bind address cannot be empty".to_string());
    }
    // Common bind addresses
    if trimmed == "0.0.0.0" || trimmed == "127.0.0.1" || trimmed == "localhost" {
        return Ok(());
    }
    // Validate as IP
    let parts: Vec<&str> = trimmed.split('.').collect();
    if parts.len() == 4 && parts.iter().all(|p| p.parse::<u8>().is_ok()) {
        return Ok(());
    }
    Err("Please enter a valid IP address (e.g., 0.0.0.0 or 127.0.0.1)".to_string())
}

/// Validates database name - alphanumeric and underscores only
pub fn validate_db_name(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Database name cannot be empty".to_string());
    }
    if trimmed.len() > 64 {
        return Err("Database name is too long (max 64 characters)".to_string());
    }
    if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Database name can only contain letters, numbers, and underscores".to_string());
    }
    if trimmed
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        return Err("Database name cannot start with a number".to_string());
    }
    Ok(())
}

/// Validates application name
pub fn validate_app_name(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Application name cannot be empty".to_string());
    }
    if trimmed.len() < 2 {
        return Err("Application name must be at least 2 characters".to_string());
    }
    if trimmed.len() > 50 {
        return Err("Application name is too long (max 50 characters)".to_string());
    }
    Ok(())
}

#[derive(Debug, Wizard)]
#[allow(dead_code)]
struct DatabaseConfig {
    #[prompt("Database type:")]
    db_type: DatabaseType,

    #[prompt("Database host (e.g., localhost):")]
    #[validate("validate_host")]
    host: String,

    #[prompt("Database port:")]
    #[min(1)]
    #[max(65535)]
    port: i64,

    #[prompt("Database name:")]
    #[validate("validate_db_name")]
    name: String,

    #[prompt("Database username:")]
    username: String,

    #[prompt("Database password:")]
    #[mask]
    password: String,

    #[prompt("Connection pool size:")]
    #[min(1)]
    #[max(100)]
    pool_size: i64,
}

#[derive(Debug, Wizard)]
#[allow(dead_code)]
struct ServerSettings {
    #[prompt("Server bind address (e.g., 0.0.0.0):")]
    #[validate("validate_bind_address")]
    bind_address: String,

    #[prompt("HTTP port:")]
    #[min(1)]
    #[max(65535)]
    http_port: i64,

    #[prompt("Enable HTTPS?")]
    enable_https: bool,

    #[prompt("Max concurrent connections:")]
    #[min(10)]
    #[max(10000)]
    max_connections: i64,

    #[prompt("Request timeout (seconds):")]
    #[min(1)]
    #[max(300)]
    timeout_seconds: i64,
}

#[derive(Debug, Wizard)]
#[allow(dead_code)]
#[prelude(
    "🖥️  Server Deployment Configuration\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\nThis wizard will help you configure your server deployment.\nPress Esc at any time to cancel."
)]
#[epilogue(
    "✅ Configuration complete!\n\nYour settings have been saved.\nRun 'deploy.sh' to apply this configuration."
)]
struct DeploymentConfig {
    #[prompt("Application name:")]
    #[validate("validate_app_name")]
    app_name: String,

    #[prompt("Deployment environment:")]
    environment: Environment,

    #[prompt("Server settings")]
    server: ServerSettings,

    #[prompt("Database configuration")]
    database: DatabaseConfig,

    #[prompt("Logging level:")]
    log_level: LogLevel,

    #[prompt("Enable metrics collection?")]
    enable_metrics: bool,

    #[prompt("Enable health check endpoint?")]
    enable_health_check: bool,

    #[prompt("Number of worker threads:")]
    #[min(1)]
    #[max(64)]
    worker_threads: i64,

    #[prompt("Memory limit (MB):")]
    #[min(128)]
    #[max(65536)]
    memory_limit_mb: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use derive_wizard::{InterviewBackend, RatatuiBackend, RatatuiColor, RatatuiTheme};

    // Cyberpunk-inspired theme
    let theme = RatatuiTheme {
        primary: RatatuiColor::Rgb(0, 255, 136),    // Neon green
        secondary: RatatuiColor::Rgb(138, 43, 226), // Purple
        highlight: RatatuiColor::Rgb(0, 255, 255),  // Cyan
        success: RatatuiColor::Rgb(0, 255, 136),    // Neon green
        error: RatatuiColor::Rgb(255, 0, 128),      // Hot pink
        text: RatatuiColor::Rgb(200, 200, 200),     // Light gray
        background: RatatuiColor::Reset,
        border: RatatuiColor::Rgb(100, 100, 100),
    };

    let interview = DeploymentConfig::interview();
    let backend = RatatuiBackend::new()
        .with_title("⚙️  Deployment Configuration Wizard")
        .with_theme(theme);

    // Use execute_with_validator to enable real-time validation
    let answers = backend.execute_with_validator(&interview, &DeploymentConfig::validate_field)?;
    let config = DeploymentConfig::from_answers(&answers)?;

    println!("\n🔧 Generated Configuration:");
    println!("════════════════════════════════════════════════════════");
    println!();
    println!("[application]");
    println!("name = \"{}\"", config.app_name);
    println!("environment = \"{:?}\"", config.environment);
    println!("workers = {}", config.worker_threads);
    println!("memory_limit = \"{}MB\"", config.memory_limit_mb);
    println!();
    println!("[server]");
    println!("bind = \"{}\"", config.server.bind_address);
    println!("port = {}", config.server.http_port);
    println!("https = {}", config.server.enable_https);
    println!("max_connections = {}", config.server.max_connections);
    println!("timeout = {}", config.server.timeout_seconds);
    println!();
    println!("[database]");
    println!("type = \"{:?}\"", config.database.db_type);
    println!("host = \"{}\"", config.database.host);
    println!("port = {}", config.database.port);
    println!("name = \"{}\"", config.database.name);
    println!("pool_size = {}", config.database.pool_size);
    println!();
    println!("[logging]");
    println!("level = \"{:?}\"", config.log_level);
    println!();
    println!("[monitoring]");
    println!("metrics = {}", config.enable_metrics);
    println!("health_check = {}", config.enable_health_check);
    println!("════════════════════════════════════════════════════════");

    Ok(())
}
