//! Vec/List example types
//!
//! Demonstrates:
//! - Vec<String> for collecting lists of strings
//! - Vec<numeric> for collecting lists of numbers
//! - Min/max bounds on numeric list elements

use elicitor::Survey;

#[derive(Survey, Debug)]
pub struct ShoppingList {
    #[ask("Enter items to buy (one per entry):")]
    pub items: Vec<String>,

    #[ask("Enter the quantity for each item:")]
    #[min(1)]
    #[max(100)]
    pub quantities: Vec<u32>,

    #[ask("Enter the price for each item (in cents):")]
    #[min(1)]
    pub prices: Vec<u32>,
}

#[derive(Survey, Debug)]
pub struct StudentGrades {
    #[ask("Student name:")]
    pub name: String,

    #[ask("Enter test scores (0-100):")]
    #[min(0)]
    #[max(100)]
    pub scores: Vec<u32>,

    #[ask("Enter homework grades (0-100):")]
    #[min(0)]
    #[max(100)]
    pub homework: Vec<u32>,
}
