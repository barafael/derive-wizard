//! Dialoguer Multi-Select Example
//!
//! A simple example demonstrating multi-select with the dialoguer backend.
//!
//! Run with: cargo run --example dialoguer_multi_select --features dialoguer-backend

use derive_wizard::Wizard;

#[derive(Debug, Clone, Copy, Wizard)]
pub enum Ingredient {
    Lettuce,
    Tomato,
    Onion,
    Pickles,
    Cheese,
    Bacon,
}

#[derive(Debug, Clone, Copy, Wizard)]
pub enum Sauce {
    Ketchup,
    Mustard,
    Mayo,
    BBQ,
}

#[derive(Debug, Clone, Copy, Default, Wizard)]
pub enum BunType {
    #[default]
    Sesame,
    Plain,
    Brioche,
    Pretzel,
}

#[derive(Debug, Wizard)]
#[prelude("🍔 Build Your Burger!")]
struct BurgerOrder {
    #[prompt("Your name:")]
    name: String,

    #[prompt("Bun type:")]
    bun: BunType,

    #[prompt("Select your ingredients:")]
    ingredients: Vec<Ingredient>,

    #[prompt("Select your sauces:")]
    sauces: Vec<Sauce>,
}

fn main() {
    let backend = derive_wizard::DialoguerBackend::new();
    let order = BurgerOrder::wizard_builder()
        .with_backend(backend)
        .build()
        .unwrap();

    println!("\n🍔 Burger Order");
    println!("───────────────");
    println!("Name: {}", order.name);
    println!("Bun: {:?}", order.bun);
    println!("Ingredients: {:?}", order.ingredients);
    println!("Sauces: {:?}", order.sauces);
}
