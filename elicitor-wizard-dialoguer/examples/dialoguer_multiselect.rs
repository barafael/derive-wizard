//! Multiselect example. Run with: cargo run --example multiselect

use elicitor_wizard_dialoguer::DialoguerBackend;
use example_surveys::DeveloperProfile;

fn main() -> anyhow::Result<()> {
    let backend = DialoguerBackend::new();
    let result = DeveloperProfile::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
