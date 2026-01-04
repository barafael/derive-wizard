//! Example demonstrating nested structs, enums, and validation with the egui backend.
//!
//! This example shows:
//! - Nested struct validation (Address inside Person)
//! - Enum with tuple and struct variants (ContactMethod)
//! - Real-time validation feedback in egui
//!
//! Run with: cargo run --example egui_nested_validation --features egui-backend

use derive_wizard::Wizard;

/// A person's contact information
#[derive(Debug, Wizard)]
#[prelude("Please fill in the contact registration form.")]
#[epilogue("Thank you for registering!")]
#[allow(dead_code)]
struct ContactInfo {
    #[prompt("Full name:")]
    #[validate("validate_name")]
    name: String,

    #[prompt("Email address:")]
    #[validate("validate_email")]
    email: String,

    #[prompt("Mailing address:")]
    address: Address,

    #[prompt("Preferred contact method:")]
    contact_method: ContactMethod,
}

/// Physical address with validation
#[derive(Debug, Wizard)]
#[allow(dead_code)]
struct Address {
    #[prompt("Street address:")]
    #[validate("validate_street")]
    street: String,

    #[prompt("City:")]
    #[validate("validate_city")]
    city: String,

    #[prompt("State (2-letter code):")]
    #[validate("validate_state")]
    state: String,

    #[prompt("ZIP code (5 digits):")]
    #[validate("validate_zip")]
    zip: String,
}

/// Contact method enum with various variants
#[derive(Debug, Wizard)]
#[allow(dead_code)]
enum ContactMethod {
    /// Contact via email only
    EmailOnly,

    /// Contact via phone
    Phone {
        #[prompt("Phone number (10 digits):")]
        #[validate("validate_phone")]
        number: String,

        #[prompt("Allow text messages?")]
        allow_sms: bool,
    },

    /// Contact via social media
    Social(
        #[prompt("Social media handle:")]
        #[validate("validate_handle")]
        String,
    ),

    /// Contact via mail only
    MailOnly,
}

// === Validators ===

pub fn validate_name(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    if input.is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    if input.len() < 2 {
        return Err(format!("Name too short ({}/2 minimum)", input.len()));
    }
    if input.len() > 100 {
        return Err("Name too long (max 100 characters)".to_string());
    }
    if !input
        .chars()
        .all(|c| c.is_alphabetic() || c.is_whitespace() || c == '-' || c == '\'')
    {
        return Err("Name can only contain letters, spaces, hyphens, and apostrophes".to_string());
    }
    Ok(())
}

pub fn validate_email(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    if input.is_empty() {
        return Err("Email cannot be empty".to_string());
    }
    if !input.contains('@') {
        return Err("Missing @ symbol".to_string());
    }
    let parts: Vec<&str> = input.split('@').collect();
    if parts.len() != 2 {
        return Err("Invalid email format".to_string());
    }
    if parts[0].is_empty() {
        return Err("Missing username before @".to_string());
    }
    if !parts[1].contains('.') {
        return Err("Domain must contain a dot".to_string());
    }
    Ok(())
}

pub fn validate_street(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    if input.is_empty() {
        return Err("Street address cannot be empty".to_string());
    }
    if input.len() < 5 {
        return Err(format!(
            "Street address too short ({}/5 minimum)",
            input.len()
        ));
    }
    Ok(())
}

pub fn validate_city(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    if input.is_empty() {
        return Err("City cannot be empty".to_string());
    }
    if input.len() < 2 {
        return Err(format!("City name too short ({}/2 minimum)", input.len()));
    }
    if !input
        .chars()
        .all(|c| c.is_alphabetic() || c.is_whitespace() || c == '-')
    {
        return Err("City can only contain letters, spaces, and hyphens".to_string());
    }
    Ok(())
}

pub fn validate_state(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    if input.is_empty() {
        return Err("State code cannot be empty".to_string());
    }
    if input.len() != 2 {
        return Err(format!(
            "State must be exactly 2 letters (got {})",
            input.len()
        ));
    }
    if !input.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err("State must be letters only".to_string());
    }
    Ok(())
}

pub fn validate_zip(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    if input.is_empty() {
        return Err("ZIP code cannot be empty".to_string());
    }
    if input.len() != 5 {
        return Err(format!(
            "ZIP must be exactly 5 digits (got {})",
            input.len()
        ));
    }
    if !input.chars().all(|c| c.is_ascii_digit()) {
        return Err("ZIP must contain only digits".to_string());
    }
    Ok(())
}

pub fn validate_phone(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    // Remove common formatting characters
    let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.is_empty() {
        return Err("Phone number cannot be empty".to_string());
    }
    if digits.len() < 10 {
        return Err(format!(
            "Phone number needs 10 digits (got {})",
            digits.len()
        ));
    }
    if digits.len() > 10 {
        return Err(format!(
            "Phone number too long (got {} digits)",
            digits.len()
        ));
    }
    Ok(())
}

pub fn validate_handle(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    if input.is_empty() {
        return Err("Handle cannot be empty".to_string());
    }
    if input.len() < 3 {
        return Err(format!("Handle too short ({}/3 minimum)", input.len()));
    }
    if input.len() > 30 {
        return Err("Handle too long (max 30 characters)".to_string());
    }
    // Allow @ prefix for social handles
    let check = input.strip_prefix('@').unwrap_or(input);
    if !check
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
    {
        return Err("Handle can only contain letters, numbers, underscores, and dots".to_string());
    }
    Ok(())
}

fn main() {
    println!("=== Contact Registration - egui Demo ===");
    println!("This example demonstrates nested structs, enums, and validation.\n");

    // Use the egui backend with custom window settings
    let backend = derive_wizard::EguiBackend::new()
        .with_title("Contact Registration")
        .with_window_size([450.0, 400.0]);

    match ContactInfo::wizard_builder().with_backend(backend).build() {
        Ok(contact) => {
            println!("\n=== Registration Complete ===");
            println!("{:#?}", contact);
        }
        Err(e) => {
            println!("\nRegistration cancelled or failed: {}", e);
        }
    }
}
