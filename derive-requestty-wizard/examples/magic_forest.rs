//! Magic Forest example demonstrating the requestty backend for derive-survey.
//!
//! This example showcases:
//! - Text input with validation
//! - Password/masked input
//! - Integer input with min/max constraints
//! - Enum selection (OneOf)
//! - Multi-select (AnyOf) with budget validation
//!
//! Run with: cargo run -p derive-requestty-wizard --example magic_forest

use derive_requestty_wizard::RequesttyBackend;
use example_surveys::SimpleMagicForest;

fn main() {
    let survey_result = SimpleMagicForest::builder()
        .run(RequesttyBackend::new())
        .expect("Survey failed");

    println!("{:#?}", survey_result);
}
