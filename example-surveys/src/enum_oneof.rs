//! Enum (OneOf) example types
//!
//! Demonstrates:
//! - Enum fields for selecting one option
//! - Unit variants (simple choices)
//! - Newtype variants (with follow-up question)
//! - Struct variants (with multiple follow-up questions)

use elicitor::Survey;

#[derive(Survey, Debug)]
pub enum ShippingMethod {
    #[ask("Standard (5-7 business days)")]
    Standard,

    #[ask("Express (2-3 business days)")]
    Express,

    #[ask("Overnight")]
    Overnight,
}

#[derive(Survey, Debug)]
pub enum PaymentMethod {
    #[ask("Credit Card")]
    CreditCard {
        #[ask("Card number:")]
        card_number: String,

        #[ask("CVV:")]
        cvv: String,
    },

    #[ask("PayPal")]
    PayPal(#[ask("Enter your PayPal email:")] String),

    #[ask("Cash on Delivery")]
    CashOnDelivery,

    #[ask("Other method")]
    Other(#[ask("Describe other method:")] String),
}

#[derive(Survey, Debug)]
pub struct Checkout {
    #[ask("Select shipping method:")]
    pub shipping: ShippingMethod,

    #[ask("Select payment method:")]
    pub payment: PaymentMethod,
}
