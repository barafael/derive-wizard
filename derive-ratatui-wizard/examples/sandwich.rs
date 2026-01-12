//! Sandwich Builder - A minimal example showcasing all derive-survey features
//!
//! Features demonstrated:
//! - Prelude/epilogue messages
//! - Primitives: String, bool, i32, u32
//! - Optional fields (Option<T> - skipped = None)
//! - Text input with validation (#[validate])
//! - Password/masked input (#[mask])
//! - Multiline text input (#[multiline])
//! - Numeric bounds (#[min], #[max])
//! - Enum selection (unit, newtype, struct variants)
//! - Multi-select (#[multiselect])
//! - Nested struct with propagated validation (#[validate_fields])
//! - PathBuf support
//! - Builder API with suggestions and closures

use derive_ratatui_wizard::{RatatuiBackend, Theme};
use derive_survey::{ResponseValue, Responses, Survey};
use ratatui::style::Color;
use std::path::PathBuf;

// Single validator: toppings budget (each topping = $0.50, max $3 = 6 toppings)
fn validate_toppings(value: &ResponseValue, _: &Responses) -> Result<(), String> {
    let ResponseValue::ChosenVariants(picks) = value else {
        return Ok(());
    };
    if picks.len() > 6 {
        return Err(format!(
            "Max 6 toppings ($3 budget) - you picked {}",
            picks.len()
        ));
    }
    Ok(())
}

// Propagated validator for nutrition info
fn validate_nutrition(value: &ResponseValue, responses: &Responses) -> Result<(), String> {
    let ResponseValue::Int(current) = value else {
        return Ok(());
    };
    let cals = Nutrition::get_calories(responses).unwrap_or(0) as i64;
    let protein = Nutrition::get_protein(responses).unwrap_or(0) as i64;
    let total = cals + protein * 4 + current; // rough check
    if total > 1500 {
        return Err("That's a lot of food! Consider a lighter option.".into());
    }
    Ok(())
}

/// Bread choice
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum Bread {
    Italian,
    Wheat,
    HoneyOat,
    Flatbread,
    Wrap,
}

/// Filling - demonstrates unit, newtype, and struct variants
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum Filling {
    Turkey,
    Ham,
    RoastBeef,
    Tuna,
    Meatball,
    VeggiePatty,
    /// Double portion
    Double(#[ask("Which filling to double?")] FillingType),
    /// Custom combo
    Combo {
        #[ask("First filling:")]
        first: FillingType,
        #[ask("Second filling:")]
        second: FillingType,
    },
}

#[allow(dead_code)]
#[derive(Survey, Debug)]
enum FillingType {
    Turkey,
    Ham,
    Bacon,
    Chicken,
    Falafel,
}

/// Cheese selection
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum Cheese {
    American,
    Provolone,
    Swiss,
    Cheddar,
    Pepper,
    None,
}

/// Toppings for multi-select
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum Topping {
    Lettuce,
    Tomato,
    Onion,
    Pickle,
    Olive,
    Jalapeno,
    Banana,
    Spinach,
    Avocado,
    Bacon,
}

/// Sauce choice
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum Sauce {
    Mayo,
    Mustard,
    Ranch,
    Chipotle,
    OilVinegar,
    None,
}

/// Size options
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum Size {
    #[ask("6 inch ($7)")]
    Six,
    #[ask("Footlong ($12)")]
    Footlong,
}

/// Nested struct for nutrition tracking
#[allow(dead_code)]
#[derive(Survey, Debug)]
#[validate_fields(validate_nutrition)]
struct Nutrition {
    #[ask("Calorie limit:")]
    #[min(200)]
    #[max(1200)]
    calories: u32,

    #[ask("Protein goal (g):")]
    #[min(10)]
    #[max(100)]
    protein: u32,
}

/// Main sandwich order
#[allow(dead_code)]
#[derive(Survey, Debug)]
#[prelude("Welcome to Rusty's Subs!\nLet's build your perfect sandwich.\n")]
#[epilogue("Order placed! Your sandwich will be ready in 5 minutes.")]
struct SandwichOrder {
    #[ask("Name for the order:")]
    name: String,

    #[ask("Rewards PIN (4 digits):")]
    #[mask]
    pin: String,

    #[ask("Choose your bread:")]
    bread: Bread,

    #[ask("Select your filling:")]
    filling: Filling,

    #[ask("What cheese?")]
    cheese: Cheese,

    #[ask("Pick your toppings (max 6, $0.50 each):")]
    #[multiselect]
    #[validate(validate_toppings)]
    toppings: Vec<Topping>,

    #[ask("Choose a sauce:")]
    sauce: Sauce,

    #[ask("What size?")]
    size: Size,

    #[ask("Toast it?")]
    toasted: bool,

    #[ask("Nutrition preferences:")]
    nutrition: Nutrition,

    #[ask("Tip amount (-$5 to +$20):")]
    #[min(-5)]
    #[max(20)]
    tip: i32,

    #[ask("Special instructions:")]
    #[multiline]
    notes: String,

    #[ask("Receipt file (optional):")]
    receipt_path: Option<PathBuf>,
}

fn main() {
    let theme = Theme {
        primary: Color::Yellow,
        secondary: Color::LightYellow,
        background: Color::Reset,
        text: Color::White,
        highlight: Color::Green,
        error: Color::Red,
        success: Color::LightGreen,
        border: Color::DarkGray,
    };

    let backend = RatatuiBackend::new()
        .with_title("Rusty's Subs - Order")
        .with_theme(theme);

    let result = SandwichOrder::builder()
        .suggest_name("Ferris".to_string())
        .suggest_toasted(true)
        .suggest_tip(3)
        .suggest_bread(|b| b.suggest_italian())
        .suggest_filling(|f| f.suggest_turkey())
        .suggest_cheese(|c| c.suggest_provolone())
        .suggest_sauce(|s| s.suggest_oil_vinegar())
        .suggest_size(|sz| sz.suggest_six())
        .suggest_nutrition(|n| n.calories(600).protein(30))
        .run(backend);

    match result {
        Ok(order) => {
            println!("\n=== Order Confirmed ===\n");
            println!("Name: {}", order.name);
            println!("Bread: {:?}", order.bread);
            println!("Filling: {:?}", order.filling);
            println!("Cheese: {:?}", order.cheese);
            println!("Toppings: {:?}", order.toppings);
            println!("Sauce: {:?}", order.sauce);
            println!("Size: {:?}", order.size);
            println!("Toasted: {}", if order.toasted { "Yes" } else { "No" });
            println!("Tip: ${}", order.tip);
            if !order.notes.is_empty() {
                println!("Notes: {}", order.notes);
            }
            println!("\n{:#?}", order);
        }
        Err(e) => {
            eprintln!("Order cancelled: {e}");
            std::process::exit(1);
        }
    }
}
