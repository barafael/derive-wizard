//! Magic Forest example demonstrating the dialoguer backend for derive-survey.
//!
//! This example showcases:
//! - Text input with validation
//! - Password/masked input
//! - Integer input with min/max constraints
//! - Enum selection (OneOf)
//! - Multi-select (AnyOf) with budget validation
//!
//! Run with: cargo run -p derive-dialoguer-wizard --example magic_forest

use derive_dialoguer_wizard::DialoguerBackend;
use example_surveys::SimpleMagicForest;

fn main() {
    let survey_result = SimpleMagicForest::builder()
        .run(DialoguerBackend::new())
        .expect("Survey failed");

    println!("=== Your Character ===");
    println!("{:#?}", survey_result);
}
