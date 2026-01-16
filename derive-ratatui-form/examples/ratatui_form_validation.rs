//! Validation example
//!
//! Demonstrates:
//! - #[validate("fn_name")] for field-level validation
//! - Custom validator functions
//! - Using ResponseValue and Responses for validation
//!
//! Run with: cargo run -p derive-ratatui-form --example ratatui_form_validation

use derive_ratatui_form::RatatuiFormBackend;
use example_surveys::AccountCreation;

fn main() -> anyhow::Result<()> {
    let backend = RatatuiFormBackend::new();
    let result = AccountCreation::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
