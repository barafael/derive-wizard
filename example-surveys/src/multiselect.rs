//! Multiselect example types
//!
//! Demonstrates:
//! - #[multiselect] attribute for Vec<Enum> fields
//! - Allows selecting multiple enum variants at once
//! - Data-carrying variants (newtype and struct) with follow-up questions

use elicitor::Survey;

#[derive(Survey, Debug)]
pub enum ProgrammingLanguage {
    #[ask("Rust")]
    Rust,

    #[ask("Python")]
    Python,

    #[ask("JavaScript")]
    JavaScript,

    #[ask("Go")]
    Go,

    #[ask("C++")]
    Cpp,

    #[ask("Java")]
    Java,

    #[ask("Other language")]
    Other(#[ask("Which language?")] String),
}

#[derive(Survey, Debug)]
pub enum Hobby {
    #[ask("Reading")]
    Reading(#[ask("Favorite book genre?")] String),

    #[ask("Gaming")]
    Gaming {
        #[ask("Favorite game?")]
        favorite_game: String,

        #[ask("Hours per week?")]
        #[min(1)]
        #[max(100)]
        hours_per_week: u32,
    },

    #[ask("Cooking")]
    Cooking,

    #[ask("Sports")]
    Sports(#[ask("Which sport?")] String),

    #[ask("Music")]
    Music {
        #[ask("Do you play an instrument?")]
        plays_instrument: bool,

        #[ask("Favorite genre?")]
        favorite_genre: String,
    },

    #[ask("Travel")]
    Travel(#[ask("Which country do you want to visit next?")] String),
}

#[derive(Survey, Debug)]
pub struct DeveloperProfile {
    #[ask("What is your name?")]
    pub name: String,

    #[ask("Select the programming languages you know:")]
    #[multiselect]
    pub languages: Vec<ProgrammingLanguage>,

    #[ask("Select your hobbies:")]
    #[multiselect]
    pub hobbies: Vec<Hobby>,
}
