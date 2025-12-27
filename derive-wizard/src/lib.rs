#![doc = include_str!("../../README.md")]

pub mod backend;

#[cfg(feature = "egui-backend")]
pub mod egui_backend;

#[cfg(feature = "dialoguer-backend")]
pub mod dialoguer_backend;

#[cfg(feature = "requestty-backend")]
pub mod requestty_backend;

pub use backend::{AnswerValue, Answers, BackendError, InterviewBackend, TestBackend};
pub use derive_wizard_macro::*;
pub use derive_wizard_types::{interview, question};

#[cfg(feature = "requestty-backend")]
pub use requestty::{ExpandItem, ListItem, Question, prompt_one};

#[cfg(feature = "egui-backend")]
pub use egui_backend::EguiBackend;

#[cfg(feature = "dialoguer-backend")]
pub use dialoguer_backend::DialoguerBackend;

#[cfg(feature = "requestty-backend")]
pub use requestty_backend::RequesttyBackend;

pub trait Wizard: Sized {
    /// Get the interview structure for this type
    fn interview() -> interview::Interview;

    /// Build this type from collected answers
    fn from_answers(answers: &Answers) -> Result<Self, BackendError>;

    /// Execute the interview with the default backend (requestty)
    #[cfg(feature = "requestty-backend")]
    fn wizard() -> Self {
        Self::wizard_with_backend(&RequesttyBackend)
    }

    /// Execute the interview with a custom backend
    fn wizard_with_backend<B: InterviewBackend>(backend: &B) -> Self {
        let interview = Self::interview();
        let answers = backend
            .execute(&interview)
            .expect("Failed to execute interview");
        Self::from_answers(&answers).expect("Failed to build from answers")
    }

    #[cfg(feature = "requestty-backend")]
    fn wizard_with_message(message: &str) -> Self {
        let _ = message;
        Self::wizard()
    }

    #[cfg(feature = "requestty-backend")]
    fn wizard_with_defaults(self) -> Self {
        self
    }
}
