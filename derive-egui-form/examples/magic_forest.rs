//! Magic Forest example - comprehensive demo of all features in egui.
//!
//! This example combines:
//! - Text input with validation
//! - Password/masked fields
//! - Integer fields with bounds
//! - Enum selection (OneOf)
//! - Multi-select (AnyOf)
//! - Nested struct fields within enum variants
//! - Prelude and epilogue messages
//!
//! Run with: cargo run -p derive-egui-form --example magic_forest

use derive_egui_form::EguiBackend;
use example_surveys::MagicForest;

fn main() -> anyhow::Result<()> {
    println!("=== Magic Forest Adventure - egui Complete Demo ===\n");

    let backend = EguiBackend::new()
        .with_title("Magic Forest Adventure")
        .with_window_size([550.0, 700.0]);

    let adventure = MagicForest::builder().run(backend)?;

    println!("{:#?}", adventure);

    Ok(())
}
