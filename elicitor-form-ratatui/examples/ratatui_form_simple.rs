//! Simple example demonstrating the ratatui form backend for derive-survey.
//!
//! Run with: cargo run -p derive-ratatui-form --example ratatui_form_simple

use elicitor_form_ratatui::RatatuiFormBackend;
use example_surveys::UserProfile;

fn main() -> anyhow::Result<()> {
    let backend = RatatuiFormBackend::new().with_title("User Profile Survey");

    let profile: UserProfile = UserProfile::builder().run(backend)?;

    println!("{profile:#?}");

    Ok(())
}
