//! Nested struct example types
//!
//! Demonstrates:
//! - Nested structs create "AllOf" question groups
//! - All nested fields are collected together

use elicitor::Survey;

#[derive(Survey, Debug)]
pub struct Address {
    #[ask("Street address:")]
    pub street: String,

    #[ask("City:")]
    pub city: String,

    #[ask("Postal code:")]
    pub postal_code: String,

    #[ask("Country:")]
    pub country: String,
}

#[derive(Survey, Debug)]
pub struct ContactInfo {
    #[ask("Email address:")]
    pub email: String,

    #[ask("Phone number:")]
    pub phone: String,
}

#[derive(Survey, Debug)]
pub struct UserRegistration {
    #[ask("Full name:")]
    pub name: String,

    #[ask("Age:")]
    #[min(18)]
    pub age: u32,

    #[ask("Contact information:")]
    pub contact: ContactInfo,

    #[ask("Home address:")]
    pub address: Address,
}
