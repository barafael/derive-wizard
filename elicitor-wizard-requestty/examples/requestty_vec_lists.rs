//! Vec/List example
//!
//! Run with: cargo run -p derive-requestty-wizard --example vec_lists

use elicitor_wizard_requestty::RequesttyBackend;
use example_surveys::{ShoppingList, StudentGrades};

fn main() -> anyhow::Result<()> {
    let backend = RequesttyBackend::new();
    let shopping = ShoppingList::builder().run(backend)?;
    println!("{shopping:#?}");

    let backend = RequesttyBackend::new();
    let grades = StudentGrades::builder().run(backend)?;
    println!("{grades:#?}");

    Ok(())
}
