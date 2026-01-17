//! Enum (OneOf) example. Run with: cargo run --example enum_oneof

use elicitor_wizard_dialoguer::DialoguerBackend;
use example_surveys::Checkout;

fn main() -> anyhow::Result<()> {
    let backend = DialoguerBackend::new();
    let result = Checkout::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
