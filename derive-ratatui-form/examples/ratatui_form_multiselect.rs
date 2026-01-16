//! Multi-select example
//!
//! Demonstrates:
//! - Vec<Enum> as multi-select checkbox fields
//! - #[multiselect] attribute
//! - Multiple selection validation
//!
//! Run with: cargo run -p derive-ratatui-form --example ratatui_form_multiselect

use derive_ratatui_form::RatatuiFormBackend;
use example_surveys::DeveloperProfile;

fn main() -> anyhow::Result<()> {
    let backend = RatatuiFormBackend::new();
    let result = DeveloperProfile::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
