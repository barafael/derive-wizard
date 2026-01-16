//! Vec/List fields example
//!
//! Demonstrates:
//! - Vec<String> for comma-separated string lists
//! - Vec<i64> for comma-separated integer lists
//! - Vec<f64> for comma-separated float lists
//!
//! Run with: cargo run -p derive-ratatui-form --example ratatui_form_vec_lists

use derive_ratatui_form::RatatuiFormBackend;
use example_surveys::ShoppingList;

fn main() -> anyhow::Result<()> {
    let backend = RatatuiFormBackend::new();
    let result = ShoppingList::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
