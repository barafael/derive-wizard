//! Example demonstrating the ratatui TUI backend with validation
//!
//! Run with: cargo run --example ratatui --features ratatui-backend

use derive_wizard::Wizard;

#[derive(Debug, Wizard)]
#[allow(dead_code)]
enum Subscription {
    Free,
    Basic,
    Premium,
}

/// Validates email address format
pub fn validate_email(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    if input.is_empty() {
        return Err("Email cannot be empty".to_string());
    }
    if !input.contains('@') {
        return Err("Email must contain an @ symbol".to_string());
    }
    let parts: Vec<&str> = input.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err("Email must be in format 'user@domain'".to_string());
    }
    if !parts[1].contains('.') {
        return Err("Email domain must contain a dot (e.g., example.com)".to_string());
    }
    Ok(())
}

/// Validates that name is not empty and has reasonable length
pub fn validate_name(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    if input.trim().is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    if input.len() < 2 {
        return Err("Name must be at least 2 characters".to_string());
    }
    if input.len() > 100 {
        return Err("Name must be less than 100 characters".to_string());
    }
    Ok(())
}

/// A user profile form with various field types
#[derive(Debug, Wizard)]
#[allow(dead_code)]
struct UserProfile {
    #[prompt("What is your name?")]
    #[validate("validate_name")]
    name: String,

    #[prompt("How old are you?")]
    #[min(0)]
    #[max(150)]
    age: i64,

    #[prompt("Enter your email:")]
    #[validate("validate_email")]
    email: String,

    #[prompt("What is your monthly income?")]
    #[min(0.0)]
    income: f64,

    #[prompt("Subscribe to newsletter?")]
    subscribe: bool,

    #[prompt("Select subscription tier:")]
    tier: Subscription,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use derive_wizard::{InterviewBackend, RatatuiBackend};

    println!("Starting ratatui wizard...\n");

    let interview = UserProfile::interview();
    let backend = RatatuiBackend::new().with_title("✨ User Profile Setup");

    // Use execute_with_validator to enable real-time validation
    let answers = backend.execute_with_validator(&interview, &UserProfile::validate_field)?;

    println!("\n📋 Collected Answers:");
    println!("━━━━━━━━━━━━━━━━━━━━━");
    for (key, value) in answers.iter() {
        println!("  {}: {:?}", key, value);
    }

    let profile = UserProfile::from_answers(&answers)?;
    println!("\n👤 Profile Created:");
    println!("{:#?}", profile);

    Ok(())
}
