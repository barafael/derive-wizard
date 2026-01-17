//! Min/max bounds example. Run with: cargo run --example min_max_bounds

use elicitor_wizard_dialoguer::DialoguerBackend;
use example_surveys::GameSettings;

fn main() -> anyhow::Result<()> {
    let backend = DialoguerBackend::new();
    let result = GameSettings::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
