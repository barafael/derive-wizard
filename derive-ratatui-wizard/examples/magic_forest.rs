//! Magic Forest - A comprehensive example demonstrating ALL derive-survey features
//!
//! This example showcases:
//! - Prelude and epilogue messages
//! - All primitive types (String, integers, floats, bool)
//! - Text input with validation
//! - Password/masked input
//! - Multiline text input
//! - Integer input with min/max constraints
//! - Float input with min/max constraints
//! - Boolean confirmation
//! - Enum selection (OneOf) with unit, newtype, tuple, and struct variants
//! - Multi-select (AnyOf) with validation
//! - Nested structs (AllOf)
//! - Deeply nested structures
//! - Field-level validation
//! - Builder API with suggestions and assumptions
//! - PathBuf support

use derive_ratatui_wizard::{RatatuiBackend, Theme};
use derive_survey::{ResponseValue, Responses, Survey};
use ratatui::style::Color;
use std::path::PathBuf;

/// Validates that a name is between 3 and 50 characters
fn validate_name(value: &ResponseValue, _responses: &Responses) -> Result<(), String> {
    let ResponseValue::String(name) = value else {
        return Ok(());
    };
    if name.trim().is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    if name.len() < 3 {
        return Err("Name must be at least 3 characters".to_string());
    }
    if name.len() > 50 {
        return Err("Name must be at most 50 characters".to_string());
    }
    if !name.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
        return Err("Name can only contain letters and spaces".to_string());
    }
    Ok(())
}

/// Validates email format
fn validate_email(value: &ResponseValue, _responses: &Responses) -> Result<(), String> {
    let ResponseValue::String(email) = value else {
        return Ok(());
    };
    if !email.contains('@') || !email.contains('.') {
        return Err("Please enter a valid email address".to_string());
    }
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err("Email must be in format 'user@domain.com'".to_string());
    }
    Ok(())
}

/// Validates the secret passphrase
fn validate_passphrase(value: &ResponseValue, _responses: &Responses) -> Result<(), String> {
    let ResponseValue::String(pass) = value else {
        return Ok(());
    };
    if pass.len() < 8 {
        return Err("Passphrase must be at least 8 characters".to_string());
    }
    if !pass.chars().any(|c| c.is_uppercase()) {
        return Err("Passphrase must contain at least one uppercase letter".to_string());
    }
    if !pass.chars().any(|c| c.is_numeric()) {
        return Err("Passphrase must contain at least one number".to_string());
    }
    Ok(())
}

/// Validates biography length
fn validate_bio(value: &ResponseValue, _responses: &Responses) -> Result<(), String> {
    let ResponseValue::String(bio) = value else {
        return Ok(());
    };
    if bio.len() > 500 {
        return Err("Biography must be at most 500 characters".to_string());
    }
    Ok(())
}

/// Validates inventory budget (called on multi-select)
fn validate_inventory_budget(value: &ResponseValue, _responses: &Responses) -> Result<(), String> {
    let ResponseValue::ChosenVariants(selections) = value else {
        return Ok(());
    };

    const STARTING_BUDGET: u32 = 200;
    let mut total_cost: u32 = 0;

    for &variant_idx in selections {
        let item_cost = match variant_idx {
            0 => 80,  // Sword
            1 => 50,  // Shield
            2 => 20,  // Potion
            3 => 10,  // Scroll
            4 => 15,  // ChewingGum base
            5 => 100, // MagicWand
            _ => 0,
        };
        total_cost += item_cost;
    }

    if total_cost > STARTING_BUDGET {
        Err(format!(
            "Over budget! Total: {} gold, limit: {} gold. Remove some items.",
            total_cost, STARTING_BUDGET
        ))
    } else {
        Ok(())
    }
}

/// Validates that at least one skill is selected
fn validate_skills(value: &ResponseValue, _responses: &Responses) -> Result<(), String> {
    let ResponseValue::ChosenVariants(selections) = value else {
        return Ok(());
    };
    if selections.is_empty() {
        return Err("You must select at least one skill".to_string());
    }
    if selections.len() > 3 {
        return Err("You can select at most 3 skills".to_string());
    }
    Ok(())
}

/// Validates character stats - total points cannot exceed threshold
/// This validator is called each time a stat value is entered, checking the running total.
const MAX_STAT_POINTS: i64 = 75;

fn validate_stat_total(value: &ResponseValue, responses: &Responses) -> Result<(), String> {
    let ResponseValue::Int(current_value) = value else {
        return Ok(());
    };

    // Use typed accessors instead of string paths
    let total: i64 = *current_value
        + CharacterStats::get_strength(responses).unwrap_or(0) as i64
        + CharacterStats::get_dexterity(responses).unwrap_or(0) as i64
        + CharacterStats::get_intelligence(responses).unwrap_or(0) as i64
        + CharacterStats::get_wisdom(responses).unwrap_or(0) as i64
        + CharacterStats::get_charisma(responses).unwrap_or(0) as i64
        + CharacterStats::get_constitution(responses).unwrap_or(0) as i64;

    let remaining = MAX_STAT_POINTS - total + current_value; // Points remaining after this

    if total > MAX_STAT_POINTS {
        Err(format!(
            "Total stat points ({}) exceeds maximum of {}! You have {} points remaining.",
            total,
            MAX_STAT_POINTS,
            remaining.max(0)
        ))
    } else {
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Survey, Debug)]
struct HomeLocation {
    #[ask("What realm do you hail from?")]
    realm: String,

    #[ask("What is your village name?")]
    village: String,

    #[ask("How far is your home from here (in leagues)?")]
    #[min(0)]
    #[max(1000)]
    distance_leagues: f64,
}

#[allow(dead_code)]
#[derive(Survey, Debug)]
#[validate_fields(validate_stat_total)]
struct CharacterStats {
    #[ask("Strength (1-20, total max 75):")]
    #[min(1)]
    #[max(20)]
    strength: u8,

    #[ask("Dexterity (1-20, total max 75):")]
    #[min(1)]
    #[max(20)]
    dexterity: u8,

    #[ask("Intelligence (1-20, total max 75):")]
    #[min(1)]
    #[max(20)]
    intelligence: u8,

    #[ask("Wisdom (1-20, total max 75):")]
    #[min(1)]
    #[max(20)]
    wisdom: u8,

    #[ask("Charisma (1-20, total max 75):")]
    #[min(1)]
    #[max(20)]
    charisma: u8,

    #[ask("Constitution (1-20, total max 75):")]
    #[min(1)]
    #[max(20)]
    constitution: u8,
}

/// Companion details - demonstrates struct enum variant
#[allow(dead_code)]
#[derive(Survey, Debug)]
struct CompanionDetails {
    #[ask("Companion's name:")]
    name: String,

    #[ask("Companion's species:")]
    species: CompanionSpecies,

    #[ask("Years together:")]
    #[min(0)]
    #[max(100)]
    years_together: u32,
}

/// Companion species enum
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum CompanionSpecies {
    Dog,
    Cat,
    Horse,
    Dragon,
    Phoenix,
    Other(#[ask("What species?")] String),
}

#[allow(dead_code)]
#[derive(Survey, Debug)]
enum Role {
    /// A fierce warrior
    Warrior,
    /// A mystical spellcaster
    Mage,
    /// A stealthy shadow
    Rogue,
    /// A holy healer
    Cleric,
    /// A nature guardian
    Ranger,
    /// A musical enchanter
    Bard,
    /// Custom class with description
    Custom(#[ask("Name your custom class:")] String),
}

/// Character background - demonstrates struct variants
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum Cast {
    Noble {
        #[ask("Name of your noble house:")]
        house_name: String,
        #[ask("Your title:")]
        title: String,
    },
    Commoner {
        #[ask("Your former trade:")]
        trade: String,
    },
    Outlaw {
        #[ask("What crime were you accused of?")]
        crime: String,
        #[ask("Are you actually guilty?")]
        guilty: bool,
    },
    Hermit {
        #[ask("Years spent in solitude:")]
        #[min(1)]
        #[max(50)]
        years: u32,
        #[ask("What wisdom did you discover?")]
        #[multiline]
        wisdom: String,
    },
    Traveler,
}

/// Companion type - demonstrates tuple and struct variants
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum Companion {
    /// Travel alone
    None,
    /// A loyal pet
    Pet(#[ask("Pet's name:")] String),
    /// A trusted friend with full details
    Friend(CompanionDetails),
    /// A magical familiar
    Familiar {
        #[ask("Familiar's name:")]
        name: String,
        #[ask("What form does it take?")]
        form: FamiliarForm,
    },
}

/// Familiar form
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum FamiliarForm {
    Cat,
    Owl,
    Raven,
    Toad,
    Imp,
    Sprite,
    Other(#[ask("Describe the form:")] String),
}

/// Inventory items - demonstrates multi-select with budget validation
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum Item {
    #[ask("Sword (80 gold)")]
    Sword,
    #[ask("Shield (50 gold)")]
    Shield,
    #[ask("Potion (20 gold)")]
    Potion,
    #[ask("Scroll (10 gold)")]
    Scroll,
    #[ask("Chewing Gum (15 gold)")]
    ChewingGum {
        #[ask("Flavor:")]
        flavor: String,
        #[ask("How many pieces?")]
        #[min(1)]
        #[max(10)]
        quantity: u32,
    },
    #[ask("Magic Wand (100 gold)")]
    MagicWand {
        #[ask("Wand material:")]
        material: WandMaterial,
        #[ask("Core type:")]
        core: String,
    },
}

/// Wand material
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum WandMaterial {
    Oak,
    Willow,
    Elder,
    Holly,
    Ebony,
}

/// Character skills - demonstrates simple multi-select
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum Skill {
    #[ask("Swordsmanship")]
    Swordsmanship,
    #[ask("Archery")]
    Archery,
    #[ask("Magic")]
    Magic,
    #[ask("Stealth")]
    Stealth,
    #[ask("Persuasion")]
    Persuasion,
    #[ask("Alchemy")]
    Alchemy,
    #[ask("Herbalism")]
    Herbalism,
    #[ask("Lockpicking")]
    Lockpicking,
}

/// Languages known
#[allow(dead_code)]
#[derive(Survey, Debug)]
enum Language {
    Common,
    Elvish,
    Dwarvish,
    Orcish,
    Draconic,
    Celestial,
    Infernal,
    Sylvan,
}

/// The complete Magic Forest character creation survey
#[allow(dead_code)]
#[derive(Survey, Debug)]
#[prelude(
    "Welcome, brave adventurer, to the Magic Forest!\nYou stand at the edge of an ancient woodland, ready to begin your journey.\nFirst, tell about yourself...\n\n"
)]
#[epilogue("Your character has been created!\nMay your legend in the Magic Forest be adventurous.")]
struct MagicForest {
    #[ask("What is your name?")]
    #[validate(validate_name)]
    name: String,

    #[ask("What is your age in years?")]
    #[min(16)]
    #[max(1000)]
    age: u32,

    #[ask("What is your contact email? (for the guild records)")]
    #[validate(validate_email)]
    email: String,

    #[ask("Create a secret passphrase (8+ chars, uppercase & number required):")]
    #[mask]
    #[validate(validate_passphrase)]
    passphrase: String,

    #[ask("Tell us your backstory:")]
    #[multiline]
    #[validate(validate_bio)]
    biography: String,

    #[ask("Choose your class:")]
    role: Role,

    #[ask("What is your cast?")]
    background: Cast,

    #[ask("Allocate your character stats:")]
    stats: CharacterStats,

    #[ask("Where do you come from?")]
    home: HomeLocation,

    #[ask("Do you travel with a companion?")]
    companion: Companion,

    #[ask("Select your skills (1-3):")]
    #[multiselect]
    #[validate(validate_skills)]
    skills: Vec<Skill>,

    #[ask("What languages do you speak?")]
    #[multiselect]
    languages: Vec<Language>,

    #[ask("Choose your starting inventory (budget: 200 gold):")]
    #[multiselect]
    #[validate(validate_inventory_budget)]
    inventory: Vec<Item>,

    #[ask("Your character portrait file:")]
    portrait_path: PathBuf,

    #[ask("Enable hardcore mode? (permadeath)")]
    hardcore_mode: bool,

    #[ask("Your lucky number:")]
    #[min(-999)]
    #[max(999)]
    lucky_number: i32,

    #[ask("Starting gold multiplier (1-10, will be divided by 5):")]
    #[min(1)]
    #[max(10)]
    gold_multiplier_raw: i32,

    #[ask("Any additional notes for the Dungeon Master?")]
    #[multiline]
    dm_notes: String,
}

fn main() {
    let fantasy_theme = Theme {
        primary: Color::Magenta,
        secondary: Color::LightMagenta,
        background: Color::Reset,
        text: Color::White,
        highlight: Color::Yellow,
        error: Color::LightRed,
        success: Color::LightGreen,
        border: Color::DarkGray,
    };

    let backend = RatatuiBackend::new()
        .with_title("Magic Forest - Character Creation")
        .with_theme(fantasy_theme);

    let result = MagicForest::builder()
        // Simple field suggestions
        .suggest_name("Gandalf".to_string())
        .suggest_age(500) // Wizards live long, but within the 1000 year max
        .suggest_email("gandalf@middleearth.org".to_string())
        .suggest_lucky_number(7)
        .suggest_gold_multiplier_raw(5)
        .suggest_hardcore_mode(false)
        // Nested struct suggestion using closure API
        .suggest_home(|home| {
            home.realm("Middle-earth")
                .village("Hobbiton")
                .distance_leagues(500.0)
        })
        // Nested struct for character stats (total must be <= 75)
        .suggest_stats(|stats| {
            stats
                .strength(8)
                .dexterity(10)
                .intelligence(18)
                .wisdom(16)
                .charisma(12)
                .constitution(10)
            // Total: 8+10+18+16+12+10 = 74, within the 75 point limit
        })
        // Enum suggestion with variant selection and nested fields
        .suggest_role(|role| {
            // Pre-select Mage as the default class
            role.suggest_mage()
        })
        // Enum with struct variant fields
        .suggest_background(|bg| {
            // Pre-select Hermit and suggest its fields (years max is 50)
            bg.suggest_hermit().hermit(|h| {
                h.years(42)
                    .wisdom("A wizard is never late, nor is he early.")
            })
        })
        // Enum with various variant types
        .suggest_companion(|comp| {
            // Pre-select Familiar variant and configure its fields
            comp.suggest_familiar()
                .familiar(|f| f.name("Shadowfax").form(|form| form.suggest_other()))
                // Also suggest values for Friend variant (in case user picks it)
                .friend(|details| details.name("Hynix"))
        })
        .run(backend);

    match result {
        Ok(character) => {
            let gold_multiplier = character.gold_multiplier_raw as f64 / 5.0;
            println!("\n");
            println!("╔═════════════════════════════════════════════════════════╗");
            println!("║              CHARACTER CREATION COMPLETE                ║");
            println!("╠═════════════════════════════════════════════════════════╣");
            println!("║                                                         ║");
            println!("  Name: {}", character.name);
            println!("  Age: {} years", character.age);
            println!("  Email: {}", character.email);
            println!("  Role: {:?}", character.role);
            println!("  Background: {:?}", character.background);
            println!(
                "  Home: {} in {}",
                character.home.village, character.home.realm
            );
            println!("  Companion: {:?}", character.companion);
            println!("  Skills: {:?}", character.skills);
            println!("  Languages: {:?}", character.languages);
            println!("  Inventory: {} items", character.inventory.len());
            println!(
                "  Stats: STR:{} DEX:{} INT:{} WIS:{} CHA:{} CON:{}",
                character.stats.strength,
                character.stats.dexterity,
                character.stats.intelligence,
                character.stats.wisdom,
                character.stats.charisma,
                character.stats.constitution,
            );
            println!(
                "  Hardcore: {}",
                if character.hardcore_mode { "YES" } else { "No" }
            );
            println!("  Lucky Number: {}", character.lucky_number);
            println!("  Gold Multiplier: {:.1}x", gold_multiplier);
            println!("  Portrait: {:?}", character.portrait_path);
            println!("║                                                               ║");
            println!("╚══════════════════════════════════════════════════════════════╝");
            println!("\n=== Full Character Data ===\n");
            println!("{:#?}", character);
        }
        Err(e) => {
            eprintln!("\nCharacter creation failed: {}", e);
            std::process::exit(1);
        }
    }
}
