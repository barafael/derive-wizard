//! Optional fields example
//!
//! Demonstrates:
//! - Option<T> fields that can be skipped
//! - Optional nested structs
//!
//! Run with: cargo run -p derive-ratatui-form --example ratatui_form_optional_fields

use derive_ratatui_form::RatatuiFormBackend;
use example_surveys::ProjectConfig;

fn main() -> anyhow::Result<()> {
    let backend = RatatuiFormBackend::new();
    let result = ProjectConfig::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
