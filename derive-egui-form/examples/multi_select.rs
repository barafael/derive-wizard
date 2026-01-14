//! Multi-select example demonstrating AnyOf (Vec<Enum>) in the egui backend.
//!
//! This example shows how to use multi-select checkboxes for enum vectors.
//!
//! Run with: cargo run -p derive-egui-form --example multi_select

use derive_egui_form::EguiBackend;
use example_surveys::DeveloperProfile;

fn main() -> anyhow::Result<()> {
    println!("=== Developer Profile Survey - egui Multi-Select Demo ===\n");

    let backend = EguiBackend::new()
        .with_title("Developer Profile Survey")
        .with_window_size([550.0, 650.0]);

    let profile = DeveloperProfile::builder().run(backend)?;

    println!("{:#?}", profile);

    Ok(())
}
