/// Dialoguer-based wizard implementation for better testing support
///
/// Dialoguer has built-in support for testing via the `console` crate's
/// `set_colors_enabled()` and terminal abstraction.
use dialoguer::{Confirm, Input, theme::ColorfulTheme};

pub trait DialoguerWizard: Sized {
    fn wizard_dialoguer() -> Self;
}

// Helper functions that work in test mode
pub fn prompt_string(message: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .interact_text()
        .expect("Failed to read input")
}

pub fn prompt_bool(message: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .interact()
        .expect("Failed to read input")
}

pub fn prompt_number<T>(message: &str) -> T
where
    T: std::str::FromStr + std::fmt::Display + Clone,
    T::Err: std::fmt::Display + std::fmt::Debug,
{
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .interact_text()
        .expect("Failed to read input")
}
