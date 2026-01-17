//! Vec/List example. Run with: cargo run -p derive-dialoguer-wizard --example vec_lists

use elicitor_wizard_dialoguer::DialoguerBackend;
use example_surveys::{ShoppingList, StudentGrades};

fn main() -> anyhow::Result<()> {
    let backend = DialoguerBackend::new();
    let shopping = ShoppingList::builder().run(backend)?;
    println!("{shopping:#?}");

    let backend = DialoguerBackend::new();
    let grades = StudentGrades::builder().run(backend)?;
    println!("{grades:#?}");

    Ok(())
}
