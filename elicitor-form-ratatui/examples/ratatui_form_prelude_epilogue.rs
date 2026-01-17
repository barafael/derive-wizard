//! Prelude and epilogue example
//!
//! Demonstrates:
//! - #[prelude("...")] for introductory text
//! - #[epilogue("...")] for closing text
//! - Survey-level attributes
//!
//! Run with: cargo run -p derive-ratatui-form --example ratatui_form_prelude_epilogue

use elicitor_form_ratatui::RatatuiFormBackend;
use example_surveys::FitnessProfile;

fn main() -> anyhow::Result<()> {
    let backend = RatatuiFormBackend::new();
    let result = FitnessProfile::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
