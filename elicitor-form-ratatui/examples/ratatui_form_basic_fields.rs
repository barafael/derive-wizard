//! Basic field types example
//!
//! Demonstrates:
//! - String input
//! - Boolean confirmation
//! - Numeric types (u32, i32, f64)
//!
//! Run with: cargo run -p derive-ratatui-form --example ratatui_form_basic_fields

use elicitor_form_ratatui::RatatuiFormBackend;
use example_surveys::BasicFields;

fn main() -> anyhow::Result<()> {
    let backend = RatatuiFormBackend::new();
    let result = BasicFields::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
