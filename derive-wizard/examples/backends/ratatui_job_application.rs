//! Job Application Wizard 💼
//!
//! A professional form demonstrating various input types and validation.
//!
//! Run with: cargo run --example ratatui_job_application --features ratatui-backend

use derive_wizard::Wizard;

#[derive(Debug, Wizard)]
#[allow(dead_code)]
enum EmploymentType {
    FullTime,
    PartTime,
    Contract,
    Internship,
}

#[derive(Debug, Wizard)]
#[allow(dead_code)]
enum ExperienceLevel {
    Entry,
    Junior,
    Mid,
    Senior,
    Lead,
    Principal,
}

#[derive(Debug, Wizard)]
#[allow(dead_code)]
enum Department {
    Engineering,
    Design,
    Marketing,
    Sales,
    HumanResources,
    Finance,
    Operations,
}

/// Validates full name - must have at least first and last name
pub fn validate_full_name(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.len() < 2 {
        return Err("Please enter your full name (first and last name)".to_string());
    }
    if parts.iter().any(|p| p.len() < 2) {
        return Err("Each name part must be at least 2 characters".to_string());
    }
    Ok(())
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

/// Validates phone number
pub fn validate_phone(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() < 10 {
        return Err("Phone number must have at least 10 digits".to_string());
    }
    if digits.len() > 15 {
        return Err("Phone number is too long".to_string());
    }
    Ok(())
}

/// Validates city name
pub fn validate_city(input: &str, _answers: &derive_wizard::Answers) -> Result<(), String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("City cannot be empty".to_string());
    }
    if trimmed.len() < 2 {
        return Err("City name must be at least 2 characters".to_string());
    }
    if trimmed.chars().any(|c| c.is_ascii_digit()) {
        return Err("City name should not contain numbers".to_string());
    }
    Ok(())
}

#[derive(Debug, Wizard)]
#[allow(dead_code)]
#[prelude(
    "Welcome to TechCorp Careers!\n\nPlease fill out this application form.\nAll information will be kept confidential."
)]
#[epilogue(
    "Thank you for applying!\n\nWe will review your application and contact you within 5 business days."
)]
struct JobApplication {
    // Personal Information
    #[prompt("Full legal name:")]
    #[validate("validate_full_name")]
    full_name: String,

    #[prompt("Email address:")]
    #[validate("validate_email")]
    email: String,

    #[prompt("Phone number:")]
    #[validate("validate_phone")]
    phone: String,

    #[prompt("City of residence:")]
    #[validate("validate_city")]
    city: String,

    // Position Details
    #[prompt("Which department are you applying for?")]
    department: Department,

    #[prompt("Desired employment type:")]
    employment_type: EmploymentType,

    #[prompt("Your experience level:")]
    experience_level: ExperienceLevel,

    #[prompt("Years of professional experience:")]
    #[min(0)]
    #[max(50)]
    years_experience: i64,

    #[prompt("Expected annual salary (USD):")]
    #[min(0.0)]
    expected_salary: f64,

    // Additional Information
    #[prompt("Are you authorized to work in this country?")]
    work_authorized: bool,

    #[prompt("Can you start within 2 weeks if selected?")]
    available_soon: bool,

    #[prompt("How did you hear about us?")]
    referral_source: String,

    #[prompt("Any additional comments for the hiring team?")]
    comments: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use derive_wizard::{InterviewBackend, RatatuiBackend, RatatuiColor, RatatuiTheme};

    let theme = RatatuiTheme {
        primary: RatatuiColor::Rgb(0, 122, 204), // Professional blue
        secondary: RatatuiColor::Rgb(100, 100, 100), // Gray
        highlight: RatatuiColor::Rgb(0, 200, 150), // Teal accent
        success: RatatuiColor::Rgb(40, 167, 69), // Green
        error: RatatuiColor::Rgb(220, 53, 69),   // Red
        text: RatatuiColor::White,
        background: RatatuiColor::Reset,
        border: RatatuiColor::Rgb(80, 80, 80),
    };

    let interview = JobApplication::interview();
    let backend = RatatuiBackend::new()
        .with_title("💼 TechCorp Job Application Portal")
        .with_theme(theme);

    // Use execute_with_validator to enable real-time validation
    let answers = backend.execute_with_validator(&interview, &JobApplication::validate_field)?;
    let application = JobApplication::from_answers(&answers)?;

    println!("\n📄 Application Submitted:");
    println!("═══════════════════════════════════════════════════");
    println!("Applicant: {}", application.full_name);
    println!("Department: {:?}", application.department);
    println!("Position Type: {:?}", application.employment_type);
    println!(
        "Experience: {:?} ({} years)",
        application.experience_level, application.years_experience
    );
    println!("Expected Salary: ${:.2}", application.expected_salary);
    println!("═══════════════════════════════════════════════════");

    Ok(())
}
