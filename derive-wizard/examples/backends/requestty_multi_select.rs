//! Requestty Multi-Select Example
//!
//! A simple example demonstrating multi-select with the requestty backend.
//!
//! Run with: cargo run --example requestty_multi_select

use derive_wizard::Wizard;

#[derive(Debug, Clone, Copy, Wizard)]
pub enum Topping {
    Pepperoni,
    Mushrooms,
    Olives,
    Onions,
    Peppers,
    ExtraCheese,
}

#[derive(Debug, Clone, Copy, Default, Wizard)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Debug, Wizard)]
#[prelude("🍕 Welcome to Pizza Order!")]
struct PizzaOrder {
    #[prompt("Your name:")]
    name: String,

    #[prompt("Pizza size:")]
    size: Size,

    #[prompt("Select your toppings:")]
    toppings: Vec<Topping>,
}

fn main() {
    let backend = derive_wizard::RequesttyBackend;
    let order = PizzaOrder::wizard_builder()
        .with_backend(backend)
        .build()
        .unwrap();

    println!("\n📋 Order Summary");
    println!("────────────────");
    println!("Name: {}", order.name);
    println!("Size: {:?}", order.size);
    println!("Toppings: {:?}", order.toppings);
}
