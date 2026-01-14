//! Order form example types
//!
//! Demonstrates:
//! - OneOf (enum selection with radio buttons)
//! - AllOf (nested structs)
//! - Variants with data

use derive_survey::Survey;

/// Payment method selection (OneOf example).
#[derive(Debug, Survey)]
pub enum PaymentMethod2 {
    #[ask("Credit Card")]
    CreditCard {
        #[ask("Card number:")]
        card_number: String,
        #[ask("Expiry (MM/YY):")]
        expiry: String,
        #[ask("CVV:")]
        #[mask]
        cvv: String,
    },

    #[ask("PayPal")]
    PayPal {
        #[ask("PayPal email:")]
        email: String,
    },

    #[ask("Bank Transfer")]
    BankTransfer {
        #[ask("Account number:")]
        account_number: String,
        #[ask("Routing number:")]
        routing_number: String,
    },

    #[ask("Cash on Delivery")]
    CashOnDelivery,
}

/// Shipping address (AllOf example - nested struct).
#[derive(Debug, Survey)]
pub struct ShippingAddress {
    #[ask("Street address:")]
    pub street: String,

    #[ask("City:")]
    pub city: String,

    #[ask("State/Province:")]
    pub state: String,

    #[ask("Postal code:")]
    pub postal_code: String,

    #[ask("Country:")]
    pub country: String,
}

/// Shipping speed options.
#[derive(Debug, Survey)]
pub enum ShippingSpeed {
    #[ask("Standard (5-7 business days)")]
    Standard,

    #[ask("Express (2-3 business days)")]
    Express,

    #[ask("Overnight (next business day)")]
    Overnight,
}

/// Complete order form with nested structures.
#[derive(Debug, Survey)]
#[prelude("Complete your order by filling in the details below.")]
#[epilogue("Thank you for your order! We'll process it shortly.")]
pub struct OrderForm {
    #[ask("Your name:")]
    pub customer_name: String,

    #[ask("Email for order confirmation:")]
    pub email: String,

    #[ask("Phone number:")]
    pub phone: String,

    #[ask("Shipping Address")]
    pub shipping_address: ShippingAddress,

    #[ask("Shipping speed:")]
    pub shipping_speed: ShippingSpeed,

    #[ask("Payment method:")]
    pub payment_method: PaymentMethod2,

    #[ask("Order notes (optional):")]
    #[multiline]
    pub notes: String,

    #[ask("Save for future orders?")]
    pub save_details: bool,
}
