//! Validation example types
//!
//! Demonstrates:
//! - #[validate("fn_name")] for field-level validation
//! - Custom validator functions
//! - Using ResponseValue and Responses for validation

use derive_survey::{ResponsePath, ResponseValue, Responses, Survey};

pub fn validate_email(
    value: &ResponseValue,
    _responses: &Responses,
    _path: &ResponsePath,
) -> Result<(), String> {
    let ResponseValue::String(email) = value else {
        return Ok(());
    };
    if !email.contains('@') {
        return Err("Email must contain '@' symbol".to_string());
    }
    if !email.contains('.') {
        return Err("Email must contain a domain".to_string());
    }
    Ok(())
}

pub fn validate_password(
    value: &ResponseValue,
    _responses: &Responses,
    _path: &ResponsePath,
) -> Result<(), String> {
    let ResponseValue::String(password) = value else {
        return Ok(());
    };
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err("Password must contain at least one number".to_string());
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err("Password must contain at least one uppercase letter".to_string());
    }
    Ok(())
}

pub fn validate_username(
    value: &ResponseValue,
    _responses: &Responses,
    _path: &ResponsePath,
) -> Result<(), String> {
    let ResponseValue::String(username) = value else {
        return Ok(());
    };
    if username.len() < 3 {
        return Err("Username must be at least 3 characters".to_string());
    }
    if !username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err("Username can only contain letters, numbers, and underscores".to_string());
    }
    Ok(())
}

#[derive(Survey, Debug)]
pub struct AccountCreation {
    #[ask("Choose a username:")]
    #[validate(validate_username)]
    pub username: String,

    #[ask("Enter your email:")]
    #[validate(validate_email)]
    pub email: String,

    #[ask("Create a password:")]
    #[mask]
    #[validate(validate_password)]
    pub password: String,

    #[ask("Your age:")]
    #[min(13)]
    #[max(120)]
    pub age: u32,
}
