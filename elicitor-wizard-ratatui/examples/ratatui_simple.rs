//! Simple example demonstrating the ratatui wizard backend for derive-survey.
//!
//! Run with: cargo run -p derive-ratatui-wizard --example simple

use elicitor_wizard_ratatui::RatatuiBackend;
use example_surveys::UserProfile;

fn main() -> anyhow::Result<()> {
    let backend = RatatuiBackend::new().with_title("User Profile Survey");

    let profile: UserProfile = UserProfile::builder().run(backend)?;

    println!("{profile:#?}");

    Ok(())
}
