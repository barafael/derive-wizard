//! Optional fields example
//!
//! Demonstrates:
//! - Option<T> for optional fields
//! - Using builder to suggest or assume optional values
//!
//! Run with: cargo run --example optional_fields

use derive_ratatui_wizard::RatatuiBackend;
use example_surveys::ProjectConfig;

fn main() -> anyhow::Result<()> {
    println!("=== Creating project config ===");
    let backend = RatatuiBackend::new();
    let config = ProjectConfig::builder().run(backend)?;
    println!("{:#?}", config);

    Ok(())
}
