//! Job Application - A compact example showcasing ALL derive-survey features
//!
//! Features demonstrated:
//! - Prelude/epilogue messages
//! - All primitives: String, bool, integers (i32, u8, u32)
//! - Text input with validation (#[validate])
//! - Password/masked input (#[mask])
//! - Multiline text input (#[multiline])
//! - Numeric bounds (#[min], #[max])
//! - Enum selection with unit, newtype, tuple, and struct variants
//! - Multi-select with validation (#[multiselect])
//! - List input for Vec<String> (comma-separated)
//! - Nested structs with propagated field validation (#[validate_fields])
//! - PathBuf support
//! - Builder API: suggestions, closures for nested types

use derive_ratatui_wizard::{RatatuiBackend, Theme};
use derive_survey::{ResponseValue, Responses, Survey};
use ratatui::style::Color;
use std::path::PathBuf;

// ============================================================================
// Validators
// ============================================================================

fn validate_email(value: &ResponseValue, _: &Responses) -> Result<(), String> {
    let ResponseValue::String(email) = value else {
        return Ok(());
    };
    if !email.contains('@') || !email.split('@').last().is_some_and(|d| d.contains('.')) {
        return Err("Enter a valid email (e.g., you@example.com)".into());
    }
    Ok(())
}

fn validate_password(value: &ResponseValue, _: &Responses) -> Result<(), String> {
    let ResponseValue::String(pw) = value else {
        return Ok(());
    };
    if pw.len() < 6 {
        return Err("Password must be at least 6 characters".into());
    }
    Ok(())
}

fn validate_cover_letter(value: &ResponseValue, _: &Responses) -> Result<(), String> {
    let ResponseValue::String(text) = value else {
        return Ok(());
    };
    let words: Vec<_> = text.split_whitespace().collect();
    if words.len() < 10 {
        return Err(format!("Write at least 10 words ({} so far)", words.len()));
    }
    Ok(())
}

/// Skills: must pick 1-5
fn validate_skills(value: &ResponseValue, _: &Responses) -> Result<(), String> {
    let ResponseValue::ChosenVariants(picks) = value else {
        return Ok(());
    };
    if picks.is_empty() {
        return Err("Select at least one skill".into());
    }
    if picks.len() > 5 {
        return Err("Select at most 5 skills".into());
    }
    Ok(())
}

/// Salary expectations must total <= $250k (base + bonus)
const MAX_TOTAL_COMP: i64 = 250_000;

fn validate_salary(value: &ResponseValue, responses: &Responses) -> Result<(), String> {
    let ResponseValue::Int(current) = value else {
        return Ok(());
    };
    let base = Salary::get_base(responses).unwrap_or(0) as i64;
    let bonus = Salary::get_bonus(responses).unwrap_or(0) as i64;
    let total = base + bonus + current;

    if total > MAX_TOTAL_COMP {
        return Err(format!(
            "Total comp ${total}k exceeds ${MAX_TOTAL_COMP}k limit"
        ));
    }
    Ok(())
}

// ============================================================================
// Types
// ============================================================================

/// Salary expectations with cross-field validation
#[allow(dead_code)]
#[derive(Survey, Debug)]
#[validate_fields(validate_salary)]
struct Salary {
    #[ask("Base salary ($k/year):")]
    #[min(30)]
    #[max(200)]
    base: u32,

    #[ask("Expected bonus ($k/year):")]
    #[min(0)]
    #[max(100)]
    bonus: u32,
}

/// Work experience entry
#[allow(dead_code)]
#[derive(Survey, Debug)]
struct Experience {
    #[ask("Company name:")]
    company: String,

    #[ask("Months at company:")]
    #[min(1)]
    #[max(600)]
    months: u32,

    #[ask("Was this a remote position?")]
    remote: bool,
}

/// Position applying for - demonstrates unit, newtype, tuple, and struct variants
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum Position {
    /// Junior developer
    Junior,
    /// Senior developer
    Senior,
    /// Tech lead with team size
    TechLead(#[ask("Team size you'd manage:")] u8),
    /// Staff engineer
    Staff {
        #[ask("Primary focus area:")]
        focus: FocusArea,
        #[ask("Years of staff+ experience:")]
        #[min(0)]
        #[max(30)]
        years_at_level: u32,
    },
    /// Other role with custom title
    Other(#[ask("Role title:")] String, #[ask("Level (1-10):")] u8),
}

/// Engineering focus area
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum FocusArea {
    Backend,
    Frontend,
    Fullstack,
    Infrastructure,
    Security,
    Data,
}

/// Work preference
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum WorkStyle {
    Remote,
    Hybrid,
    OnSite,
}

/// Available skills for multi-select
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum Skill {
    Rust,
    Python,
    TypeScript,
    Go,
    SQL,
    Docker,
    Kubernetes,
    AWS,
    Leadership,
    Communication,
}

/// How did you hear about us?
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum Referral {
    LinkedIn,
    JobBoard,
    Referral(#[ask("Who referred you?")] String),
    Conference {
        #[ask("Conference name:")]
        name: String,
        #[ask("Year:")]
        #[min(2020)]
        #[max(2030)]
        year: u32,
    },
    Other(String),
}

// ============================================================================
// Main Survey
// ============================================================================

#[allow(dead_code)]
#[derive(Survey, Debug)]
#[prelude("Welcome to Acme Corp!\nLet's get your application started.\n")]
#[epilogue("Application submitted! We'll be in touch within 5 business days.")]
struct JobApplication {
    // Basic info with validation
    #[ask("Full name:")]
    name: String,

    #[ask("Email address:")]
    #[validate(validate_email)]
    email: String,

    #[ask("Create a portal password:")]
    #[mask]
    #[validate(validate_password)]
    password: String,

    // Enum selections
    #[ask("Position applying for:")]
    position: Position,

    #[ask("Preferred work style:")]
    work_style: WorkStyle,

    #[ask("How did you hear about us?")]
    referral: Referral,

    // Nested struct
    #[ask("Most recent experience:")]
    experience: Experience,

    // Nested struct with propagated validation
    #[ask("Salary expectations:")]
    salary: Salary,

    // Multi-select with validation
    #[ask("Your top skills (1-5):")]
    #[multiselect]
    #[validate(validate_skills)]
    skills: Vec<Skill>,

    // List of strings - schools attended
    #[ask("Schools attended (comma-separated):")]
    schools_attended: Vec<String>,

    // Multiline with validation
    #[ask("Cover letter:")]
    #[multiline]
    #[validate(validate_cover_letter)]
    cover_letter: String,

    // PathBuf
    #[ask("Resume file path:")]
    resume: PathBuf,

    // Simple bool
    #[ask("Willing to relocate?")]
    relocate: bool,

    // Signed integer with bounds
    #[ask("Timezone offset from UTC (-12 to +14):")]
    #[min(-12)]
    #[max(14)]
    timezone: i32,
}

fn main() {
    let theme = Theme {
        primary: Color::Blue,
        secondary: Color::LightBlue,
        background: Color::Reset,
        text: Color::White,
        highlight: Color::Cyan,
        error: Color::Red,
        success: Color::Green,
        border: Color::DarkGray,
    };

    let backend = RatatuiBackend::new()
        .with_title("Acme Corp - Job Application")
        .with_theme(theme);

    let result = JobApplication::builder()
        // Simple suggestions
        .suggest_name("Jane Doe".to_string())
        .suggest_email("jane@example.com".to_string())
        .suggest_timezone(-5) // EST
        .suggest_relocate(false)
        // Nested struct suggestions via closure
        .suggest_experience(|exp| exp.company("Previous Corp").months(30).remote(true))
        .suggest_salary(|sal| sal.base(120).bonus(20))
        // Enum variant selection
        .suggest_position(|pos| pos.suggest_senior())
        .suggest_work_style(|ws| ws.suggest_remote())
        // Enum with nested fields
        .suggest_referral(|r| {
            r.suggest_linked_in()
                // Also pre-fill Conference in case they switch
                .conference(|c| c.name("RustConf").year(2025))
        })
        .run(backend);

    match result {
        Ok(app) => {
            println!("\n=== Application Received ===\n");
            println!("Name: {}", app.name);
            println!("Email: {}", app.email);
            println!("Position: {:?}", app.position);
            println!("Work style: {:?}", app.work_style);
            println!("Referral: {:?}", app.referral);
            println!(
                "Experience: {} months at {}",
                app.experience.months, app.experience.company
            );
            println!(
                "Salary: ${}k base + ${}k bonus",
                app.salary.base, app.salary.bonus
            );
            println!("Skills: {:?}", app.skills);
            println!(
                "Schools attended: {}",
                if app.schools_attended.is_empty() {
                    "None".to_string()
                } else {
                    app.schools_attended.join(", ")
                }
            );
            println!("Resume: {:?}", app.resume);
            println!("Relocate: {}", if app.relocate { "Yes" } else { "No" });
            println!("Timezone: UTC{:+}", app.timezone);
            println!("\n{:#?}", app);
        }
        Err(e) => {
            eprintln!("Application cancelled: {e}");
            std::process::exit(1);
        }
    }
}
