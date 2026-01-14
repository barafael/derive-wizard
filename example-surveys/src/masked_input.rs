//! Masked input example types
//!
//! Demonstrates:
//! - #[mask] attribute for hiding sensitive input like passwords
//! - Cross-field validation for password confirmation

use derive_survey::{ResponsePath, ResponseValue, Responses, Survey};

/// Validates that the password confirmation matches the original password
pub fn passwords_match(
    value: &ResponseValue,
    responses: &Responses,
    path: &ResponsePath,
) -> Result<(), String> {
    // The current value being validated is password_confirm
    let ResponseValue::String(password_confirm) = value else {
        return Ok(());
    };

    // Get the parent path automatically - no need to hardcode "passwords"
    // The path parameter contains the full path to the current field (e.g., "passwords.password_confirm")
    // We need the parent path to create the validation context
    let parent_path = path.parent();

    // Create a validation context for accessing sibling fields using the automatic parent path
    let ctx = PasswordsValidationContext::new(responses, parent_path);

    // Get the original password using the typed accessor
    let password = ctx.get_password().unwrap_or_default();

    if password != *password_confirm {
        return Err("Passwords do not match".to_string());
    }

    Ok(())
}

#[derive(Survey, Debug)]
pub struct Passwords {
    #[ask("Enter your password:")]
    #[mask]
    pub password: String,

    #[ask("Confirm your password:")]
    #[mask]
    #[validate(passwords_match)]
    pub password_confirm: String,
}

#[derive(Survey, Debug)]
pub struct Login {
    #[ask("Enter your username:")]
    pub username: String,

    #[ask("Enter your password:")]
    pub passwords: Passwords,
}
