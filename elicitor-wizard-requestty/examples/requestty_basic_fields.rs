//! Basic field types example
//!
//! Demonstrates:
//! - String input
//! - Boolean confirmation
//! - Numeric types (u32, i32, f64)
//!
//! Run with: cargo run --example basic_fields

use elicitor_wizard_requestty::RequesttyBackend;
use example_surveys::BasicFields;

fn main() -> anyhow::Result<()> {
    let backend = RequesttyBackend::new();
    let result = BasicFields::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
