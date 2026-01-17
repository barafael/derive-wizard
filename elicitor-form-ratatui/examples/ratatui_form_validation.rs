//! Validation example
//!
//! Demonstrates:
//! - #[validate("fn_name")] for field-level validation
//! - Custom validator functions
//! - Using ResponseValue and Responses for validation
//!
//! Run with: cargo run -p derive-ratatui-form --example ratatui_form_validation

use elicitor_form_ratatui::RatatuiFormBackend;
use example_surveys::AccountCreation;

fn main() -> anyhow::Result<()> {
    let backend = RatatuiFormBackend::new();
    let result = AccountCreation::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
