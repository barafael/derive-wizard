//! Simple example demonstrating the requestty backend for derive-survey.
//!
//! Run with: cargo run -p derive-requestty-wizard --example simple

use elicitor_wizard_requestty::RequesttyBackend;
use example_surveys::UserProfile;

fn main() -> anyhow::Result<()> {
    let backend = RequesttyBackend::new();

    let profile = UserProfile::builder().run(backend)?;

    println!("{profile:#?}");

    Ok(())
}
