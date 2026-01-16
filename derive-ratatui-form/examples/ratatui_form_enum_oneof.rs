//! Enum OneOf example
//!
//! Demonstrates:
//! - Enum types as single-select (radio button) fields
//! - Variant documentation as option descriptions
//! - Automatic variant name formatting
//!
//! Run with: cargo run -p derive-ratatui-form --example ratatui_form_enum_oneof

use derive_ratatui_form::RatatuiFormBackend;
use example_surveys::Checkout;

fn main() -> anyhow::Result<()> {
    let backend = RatatuiFormBackend::new();
    let result = Checkout::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
