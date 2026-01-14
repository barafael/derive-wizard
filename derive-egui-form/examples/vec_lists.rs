//! Vec/List example
//!
//! Demonstrates:
//! - Vec<String> for collecting lists of strings
//! - Vec<numeric> for collecting lists of numbers
//! - Min/max bounds on numeric list elements
//!
//! Run with: cargo run --example vec_lists

use derive_egui_form::EguiBackend;
use example_surveys::{ShoppingList, StudentGrades};

fn main() -> anyhow::Result<()> {
    println!("=== Shopping List Example ===");
    let backend = EguiBackend::new();
    let shopping = ShoppingList::builder().run(backend)?;
    println!("{:#?}", shopping);

    println!("=== Student Grades Example ===");
    let backend = EguiBackend::new();
    let grades = StudentGrades::builder().run(backend)?;
    println!("{:#?}", grades);

    Ok(())
}
