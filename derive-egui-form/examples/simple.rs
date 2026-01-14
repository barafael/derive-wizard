//! Simple example demonstrating the egui backend for derive-survey.
//!
//! This example shows basic field types: strings, integers, floats, and booleans.
//!
//! Run with: cargo run -p derive-egui-form --example simple

use derive_egui_form::EguiBackend;
use example_surveys::UserProfile;

fn main() -> anyhow::Result<()> {
    println!("=== User Profile Survey - egui Demo ===\n");

    let backend = EguiBackend::new()
        .with_title("User Profile Survey")
        .with_window_size([450.0, 400.0]);

    let profile = UserProfile::builder().run(backend)?;

    println!("{:#?}", profile);

    Ok(())
}
