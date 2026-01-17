//! Basic field types example types
//!
//! Demonstrates:
//! - String input
//! - Boolean confirmation
//! - Numeric types (u32, i32, f64)

use elicitor::Survey;

#[derive(Survey, Debug)]
pub struct BasicFields {
    #[ask("What is your name?")]
    pub name: String,

    #[ask("How old are you?")]
    pub age: u32,

    #[ask("What is your height in meters?")]
    pub height: f64,

    #[ask("What is the temperature (can be negative)?")]
    pub temperature: i32,

    #[ask("Are you enjoying this example?")]
    pub enjoying: bool,
}
