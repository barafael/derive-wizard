//! Basic field types example. Run with: cargo run --example basic_fields

use elicitor_wizard_dialoguer::DialoguerBackend;
use example_surveys::BasicFields;

fn main() -> anyhow::Result<()> {
    let backend = DialoguerBackend::new();
    let result = BasicFields::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
