//! User profile example types (simple demo)
//!
//! A simple user profile survey demonstrating basic field types.

use elicitor::Survey;

/// A simple user profile survey.
#[derive(Debug, Survey)]
pub struct UserProfile {
    /// User's full name.
    #[ask("What is your name?")]
    pub name: String,

    /// User's age.
    #[ask("How old are you?")]
    #[min(0)]
    #[max(150)]
    pub age: i64,

    /// User's email address.
    #[ask("What is your email?")]
    pub email: String,

    /// Brief bio.
    #[ask("Tell us about yourself")]
    #[multiline]
    pub bio: String,

    /// Whether the user wants to receive the newsletter.
    #[ask("Would you like to receive our newsletter?")]
    pub newsletter: bool,
}
