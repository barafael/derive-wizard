//! Nested structs and enums example for the egui backend.
//!
//! This example demonstrates:
//! - OneOf (enum selection with radio buttons)
//! - AllOf (nested structs)
//! - Variants with data
//!
//! Run with: cargo run -p derive-egui-form --example nested_structs

use derive_egui_form::EguiBackend;
use example_surveys::OrderForm;

fn main() -> anyhow::Result<()> {
    println!("=== Order Form - egui Nested Structs Demo ===\n");

    let backend = EguiBackend::new()
        .with_title("Order Form")
        .with_window_size([550.0, 700.0]);

    let order = OrderForm::builder().run(backend)?;

    println!("{:#?}", order);

    Ok(())
}
