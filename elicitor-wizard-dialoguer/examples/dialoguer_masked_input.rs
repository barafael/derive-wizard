//! Masked input example. Run with: cargo run --example masked_input

use elicitor_wizard_dialoguer::DialoguerBackend;
use example_surveys::Login;

fn main() -> anyhow::Result<()> {
    let backend = DialoguerBackend::new();
    let result = Login::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
