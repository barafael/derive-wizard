//! Masked input example
//!
//! Demonstrates:
//! - #[mask] attribute for password-style input
//! - Hidden character display
//! - Secure input handling
//!
//! Run with: cargo run -p derive-ratatui-form --example ratatui_form_masked_input

use elicitor_form_ratatui::RatatuiFormBackend;
use example_surveys::Login;

fn main() -> anyhow::Result<()> {
    let backend = RatatuiFormBackend::new();
    let result = Login::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
