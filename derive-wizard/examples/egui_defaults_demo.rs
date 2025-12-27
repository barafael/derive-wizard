use derive_wizard::Wizard;

/// This example demonstrates the simplified egui backend usage.
/// Compare this to the dialoguer_defaults_demo to see how concise it is!
#[derive(Debug, Clone, Wizard)]
struct AppSettings {
    #[prompt("Application name:")]
    app_name: String,

    #[prompt("Port number:")]
    #[min(1024)]
    #[max(65535)]
    port: i32,

    #[prompt("Max connections:")]
    #[min(1)]
    #[max(10000)]
    max_connections: i32,

    #[prompt("Timeout in seconds:")]
    #[min(0.1)]
    #[max(300.0)]
    timeout: f64,

    #[prompt("Enable debug mode:")]
    debug_mode: bool,

    #[prompt("Log level:")]
    log_level: String,
}

fn main() {
    println!("=== Application Settings - egui Demo ===\n");

    // Use the egui backend
    let backend = derive_wizard::EguiBackend::new()
        .with_title("Application Settings")
        .with_window_size([500.0, 450.0]);

    let settings = AppSettings::wizard_with_backend(&backend);

    println!("\n=== Settings Created ===");
    println!("{:#?}", settings);
}
