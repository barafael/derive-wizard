//! Simple example for dialoguer backend. Run with: cargo run -p derive-dialoguer-wizard --example simple

use elicitor_wizard_dialoguer::DialoguerBackend;
use example_surveys::UserProfile;

fn main() -> anyhow::Result<()> {
    let backend = DialoguerBackend::new();
    let profile = UserProfile::builder().run(backend)?;
    println!("{profile:#?}");
    Ok(())
}
