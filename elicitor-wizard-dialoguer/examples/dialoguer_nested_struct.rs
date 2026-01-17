//! Nested struct example. Run with: cargo run --example nested_struct

use elicitor_wizard_dialoguer::DialoguerBackend;
use example_surveys::UserRegistration;

fn main() -> anyhow::Result<()> {
    let backend = DialoguerBackend::new();
    let result = UserRegistration::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
