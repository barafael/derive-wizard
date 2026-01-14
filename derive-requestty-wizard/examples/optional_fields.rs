//! Optional fields example
//!
//! Demonstrates:
//! - Option<T> for optional fields
//! - Using builder to suggest or assume optional values
//!
//! Run with: cargo run --example optional_fields

use derive_requestty_wizard::RequesttyBackend;
use example_surveys::ProjectConfig;

fn main() -> anyhow::Result<()> {
    println!("=== Creating project config ===");
    let backend = RequesttyBackend::new();
    let config = ProjectConfig::builder().run(backend)?;
    println!("{:#?}", config);

    Ok(())
}
