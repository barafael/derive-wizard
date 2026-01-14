pub mod app_settings;
pub mod basic_fields;
pub mod enum_oneof;
pub mod job_application;
pub mod magic_forest;
pub mod masked_input;
pub mod min_max_bounds;
pub mod multiline_text;
pub mod multiselect;
pub mod nested_struct;
pub mod optional_fields;
pub mod order_form;
pub mod prelude_epilogue;
pub mod sandwich;
pub mod simple_magic_forest;
pub mod user_profile;
pub mod validation;
pub mod vec_lists;

// Re-export app_settings types
pub use app_settings::AppSettings;

// Re-export basic_fields types
pub use basic_fields::BasicFields;

// Re-export enum_oneof types
pub use enum_oneof::{Checkout, PaymentMethod, ShippingMethod};

// Re-export job_application types
pub use job_application::{
    Experience, FocusArea, JobApplication, JobSkill, MAX_TOTAL_COMP, Position, Referral, Salary,
    WorkStyle, validate_cover_letter, validate_email as validate_job_email,
    validate_password as validate_job_password, validate_salary,
    validate_skills as validate_job_skills,
};

// Re-export magic_forest types
pub use magic_forest::{
    Cast, CharacterStats, Companion, CompanionDetails, CompanionSpecies, FamiliarForm,
    HomeLocation, Item, Language, MAX_STAT_POINTS, MagicForest, Role, Skill, WandMaterial,
    validate_bio, validate_email as validate_magic_email, validate_inventory_budget, validate_name,
    validate_passphrase, validate_skills as validate_magic_skills, validate_stat_total,
};

// Re-export masked_input types
pub use masked_input::{Login, Passwords, passwords_match};

// Re-export min_max_bounds types
pub use min_max_bounds::GameSettings;

// Re-export multiline_text types
pub use multiline_text::BlogPost;

// Re-export multiselect types
pub use multiselect::{DeveloperProfile, Hobby, ProgrammingLanguage};

// Re-export nested_struct types
pub use nested_struct::{Address, ContactInfo, UserRegistration};

// Re-export optional_fields types
pub use optional_fields::ProjectConfig;

// Re-export order_form types
pub use order_form::{OrderForm, PaymentMethod2, ShippingAddress, ShippingSpeed};

// Re-export prelude_epilogue types
pub use prelude_epilogue::FitnessProfile;

// Re-export sandwich types
pub use sandwich::{
    Bread, Cheese, Filling, FillingType, Nutrition, SandwichOrder, Sauce, Size, Topping,
    validate_nutrition, validate_toppings,
};

// Re-export simple_magic_forest types
pub use simple_magic_forest::{
    SimpleItem, SimpleMagicForest, SimpleRole, is_valid_name, is_within_starting_budget,
};

// Re-export user_profile types
pub use user_profile::UserProfile;

// Re-export validation types
pub use validation::{AccountCreation, validate_email, validate_password, validate_username};

// Re-export vec_lists types
pub use vec_lists::{ShoppingList, StudentGrades};
