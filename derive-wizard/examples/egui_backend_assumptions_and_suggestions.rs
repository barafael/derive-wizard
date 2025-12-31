use derive_wizard::Wizard;

/// This example demonstrates the difference between assumptions and suggestions
/// using the egui backend.
#[derive(Debug, Clone, Wizard)]
struct ServerConfig {
    #[prompt("Server name:")]
    server_name: String,

    #[prompt("Environment:")]
    environment: String,

    #[prompt("Port number:")]
    #[min(1024)]
    #[max(65535)]
    port: i32,

    #[prompt("Max connections:")]
    #[min(1)]
    #[max(10000)]
    max_connections: i32,

    #[prompt("Enable SSL:")]
    enable_ssl: bool,

    #[prompt("Enable debug logging:")]
    debug_logging: bool,
}

fn main() {
    println!("=== Egui Assumptions vs Suggestions Demo ===\n");
    println!("This demo shows three scenarios:");
    println!("1. Fresh configuration (no defaults)");
    println!("2. Configuration with suggestions (pre-filled but editable)");
    println!("3. Configuration with assumptions (fixed values, some fields skipped)\n");

    // Scenario 1: Fresh configuration
    println!("--- Scenario 1: Fresh Configuration ---");
    println!("Creating a new configuration from scratch...\n");

    let backend1 = derive_wizard::EguiBackend::new()
        .with_title("Scenario 1: Fresh Configuration")
        .with_window_size([550.0, 500.0]);

    let fresh_config = ServerConfig::wizard_builder()
        .with_backend(backend1)
        .build();

    println!("Fresh config created:");
    println!("{:#?}\n", fresh_config);

    // Scenario 2: Configuration with suggestions
    println!("--- Scenario 2: Configuration with Suggestions ---");
    println!("Using the previous config as suggestions.");
    println!("All fields will be shown with pre-filled values that you can edit.\n");

    let backend2 = derive_wizard::EguiBackend::new()
        .with_title("Scenario 2: With Suggestions (All Fields Shown)")
        .with_window_size([550.0, 500.0]);

    let config_with_suggestions = ServerConfig::wizard_builder()
        .with_suggestions(fresh_config.clone())
        .with_backend(backend2)
        .build();

    println!("Config with suggestions:");
    println!("{:#?}\n", config_with_suggestions);

    // Scenario 3: Configuration with assumptions
    println!("--- Scenario 3: Configuration with Assumptions ---");
    println!("Using assumptions for security-critical settings.");
    println!("These fields will be SKIPPED and use the assumed values:\n");

    println!("Assumed values (will NOT be asked):");
    println!("  - environment: production (fixed)");
    println!("  - enable_ssl: true (security requirement)");
    println!("  - debug_logging: false (security requirement)");
    println!("  - port: 443 (standard HTTPS port)");
    println!("  - max_connections: 5000 (performance tuning)");
    println!("  - server_name: prod-server-01 (naming convention)\n");
    println!("Since ALL fields have assumptions, the wizard will complete immediately!");

    let backend3 = derive_wizard::EguiBackend::new()
        .with_title("Scenario 3: With Assumptions (No Questions Asked)")
        .with_window_size([550.0, 500.0]);

    let config_with_assumptions = ServerConfig::wizard_builder()
        // .assume_field("server_name", "prod-server-01".to_string())
        .assume_field("environment", "production".to_string())
        .assume_field("port", 443)
        .assume_field("max_connections", 5000)
        .assume_field("enable_ssl", true)
        .assume_field("debug_logging", false)
        .with_backend(backend3)
        .build();

    println!("\n=== Final Production Config (from assumptions) ===");
    println!("{:#?}", config_with_assumptions);

    println!("\n=== Summary ===");
    println!("Suggestions: Pre-fill values but still show questions");
    println!("Assumptions: Skip questions entirely and use fixed values");
    println!("\nAssumptions are perfect for:");
    println!("  - Automated deployments");
    println!("  - Security-critical settings that shouldn't be changed");
    println!("  - Configuration templates with enforced policies");
}
