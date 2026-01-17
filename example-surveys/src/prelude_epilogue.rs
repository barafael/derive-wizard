//! Prelude and epilogue example types
//!
//! Demonstrates:
//! - #[prelude("...")] for showing a message before the survey starts
//! - #[epilogue("...")] for showing a message after the survey completes

use elicitor::Survey;

#[derive(Survey, Debug)]
#[prelude(
    "Welcome to the Fitness Tracker setup!\n\nThis wizard will help you configure your fitness goals and preferences.\nPlease answer the following questions."
)]
#[epilogue(
    "Setup complete! Your fitness profile has been created.\n\nYou can now start tracking your workouts and progress.\nGood luck on your fitness journey!"
)]
pub struct FitnessProfile {
    #[ask("What is your name?")]
    pub name: String,

    #[ask("What is your current weight (kg)?")]
    #[min(20)]
    #[max(300)]
    pub weight: f64,

    #[ask("What is your height (cm)?")]
    #[min(100)]
    #[max(250)]
    pub height: u32,

    #[ask("What is your target weight (kg)?")]
    #[min(20)]
    #[max(300)]
    pub target_weight: f64,

    #[ask("Do you have any previous fitness experience?")]
    pub has_experience: bool,
}
