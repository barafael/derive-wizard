//! Nested struct example
//!
//! Demonstrates:
//! - Nested Survey structs
//! - Automatic flattening of nested questions
//! - Path-based response collection
//!
//! Run with: cargo run -p derive-ratatui-form --example ratatui_form_nested_struct

use derive_ratatui_form::RatatuiFormBackend;
use example_surveys::UserRegistration;

fn main() -> anyhow::Result<()> {
    let backend = RatatuiFormBackend::new();
    let result = UserRegistration::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
