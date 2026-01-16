//! Min/max bounds example
//!
//! Demonstrates:
//! - #[min(n)] and #[max(n)] for numeric bounds
//! - Automatic validation of numeric ranges
//! - Error messages for out-of-range values
//!
//! Run with: cargo run -p derive-ratatui-form --example ratatui_form_min_max_bounds

use derive_ratatui_form::RatatuiFormBackend;
use example_surveys::GameSettings;

fn main() -> anyhow::Result<()> {
    let backend = RatatuiFormBackend::new();
    let result = GameSettings::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
