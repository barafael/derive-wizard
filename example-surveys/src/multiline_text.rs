//! Multiline text input example types
//!
//! Demonstrates:
//! - #[multiline] attribute for opening a text editor or textarea

use elicitor::Survey;

#[derive(Survey, Debug)]
pub struct BlogPost {
    #[ask("What is the title of your blog post?")]
    pub title: String,

    #[ask("Write your blog post content:")]
    #[multiline]
    pub content: String,

    #[ask("Add any final notes or comments:")]
    #[multiline]
    pub notes: String,
}
