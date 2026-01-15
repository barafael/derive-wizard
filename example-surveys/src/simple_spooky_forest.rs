//! Simple Magic Forest example types
//!
//! A simplified version of the spooky forest adventure for quick demos.
//! Demonstrates:
//! - Text input with validation
//! - Password/masked input
//! - Integer input with min/max constraints
//! - Enum selection (OneOf)
//! - Multi-select (AnyOf) with budget validation

use derive_survey::{ResponsePath, ResponseValue, Responses, Survey};

pub fn is_valid_name(
    value: &ResponseValue,
    _responses: &Responses,
    _path: &ResponsePath,
) -> Result<(), String> {
    let ResponseValue::String(name) = value else {
        return Ok(());
    };
    if name.len() > 2 && name.len() < 100 {
        Ok(())
    } else {
        Err("Name must be between 3 and 99 characters".to_string())
    }
}

pub fn is_within_starting_budget(
    value: &ResponseValue,
    _responses: &Responses,
    _path: &ResponsePath,
) -> Result<(), String> {
    let ResponseValue::ChosenVariants(selections) = value else {
        return Ok(());
    };

    const STARTING_BUDGET: u32 = 150;
    let mut total_cost: u32 = 0;

    for &variant_idx in selections {
        let item_cost = match variant_idx {
            0 => 80, // Sword
            1 => 50, // Shield
            2 => 20, // Potion
            3 => 10, // Scroll
            4 => 2,  // ChewingGum base cost
            _ => 0,
        };
        total_cost += item_cost;
    }

    if total_cost <= STARTING_BUDGET {
        Ok(())
    } else {
        Err(format!(
            "Over budget! Total: {} gold, limit: {} gold",
            total_cost, STARTING_BUDGET
        ))
    }
}

#[derive(Survey, Debug)]
pub enum SimpleRole {
    Streetfighter,
    Mage,
    Archer,
    Thief,
    Other(#[ask("What then?!")] String),
}

#[derive(Survey, Debug)]
pub enum SimpleItem {
    #[ask("Sword (value: 80)")]
    Sword,

    #[ask("Shield (value: 50)")]
    Shield,

    #[ask("Potion (value: 20)")]
    Potion,

    #[ask("Scroll (value: 10)")]
    Scroll,

    #[ask("Chewing Gum (value: 2 * quantity)")]
    ChewingGum {
        #[ask("Flavor:")]
        flavor: String,
        #[ask("How many?")]
        #[min(1)]
        #[max(10)]
        quantity: u32,
    },
}

#[derive(Survey, Debug)]
#[prelude("A journey begins...!")]
#[epilogue("Good luck.")]
pub struct SimpleSpookyForest {
    #[ask("What is your name?")]
    #[validate(is_valid_name)]
    pub name: String,

    #[ask("What's the secret passphrase?")]
    #[mask]
    pub passphrase: String,

    #[ask("How old are you?")]
    #[min(18)]
    #[max(233)]
    pub age: u32,

    #[ask("What is your role?")]
    pub role: SimpleRole,

    #[ask("Pick your inventory:")]
    #[multiselect]
    #[validate(is_within_starting_budget)]
    pub inventory: Vec<SimpleItem>,
}
