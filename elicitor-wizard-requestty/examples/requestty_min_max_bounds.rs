//! Min/max bounds example
//!
//! Demonstrates:
//! - #[min(n)] attribute for minimum numeric value
//! - #[max(n)] attribute for maximum numeric value
//!
//! Run with: cargo run --example min_max_bounds

use elicitor_wizard_requestty::RequesttyBackend;
use example_surveys::GameSettings;

fn main() -> anyhow::Result<()> {
    let backend = RequesttyBackend::new();
    let result = GameSettings::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
