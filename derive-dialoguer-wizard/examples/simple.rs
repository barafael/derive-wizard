//! Simple example demonstrating the dialoguer backend for derive-survey.
//!
//! Run with: cargo run -p derive-dialoguer-wizard --example simple

use derive_dialoguer_wizard::DialoguerBackend;
use example_surveys::UserProfile;

fn main() -> anyhow::Result<()> {
    println!("=== User Profile Survey ===\n");

    let backend = DialoguerBackend::new();

    let profile = UserProfile::builder().run(backend)?;

    println!("=== Profile Created ===");
    println!("Name: {}", profile.name);
    println!("Age: {}", profile.age);
    println!("Height: {} cm", profile.height_cm);
    println!("Email: {}", profile.email);
    println!(
        "Newsletter: {}",
        if profile.newsletter { "Yes" } else { "No" }
    );

    Ok(())
}
