pub use derive_wizard_macro::*;
pub use requestty::Question;
pub use requestty::prompt_one;

pub trait Wizard {
    fn wizard() -> Self;
}
