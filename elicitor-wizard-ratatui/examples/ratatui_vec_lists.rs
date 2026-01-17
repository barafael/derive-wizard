//! Vec/List example
//!
//! Run with: cargo run -p derive-ratatui-wizard --example vec_lists

use elicitor_wizard_ratatui::RatatuiBackend;
use example_surveys::{ShoppingList, StudentGrades};

fn main() -> anyhow::Result<()> {
    let backend = RatatuiBackend::new();
    let shopping = ShoppingList::builder().run(backend)?;
    println!("{shopping:#?}");

    let backend = RatatuiBackend::new();
    let grades = StudentGrades::builder().run(backend)?;
    println!("{grades:#?}");

    Ok(())
}
