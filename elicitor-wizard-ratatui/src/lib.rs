//! # derive-ratatui-wizard
//!
//! Ratatui wizard backend for derive-survey.
//!
//! This crate provides a rich terminal user interface (TUI) for collecting survey responses
//! using the `ratatui` library. Questions are presented step-by-step in a wizard style with
//! a progress bar, keyboard navigation, and visual feedback.
//!
//! ## Features
//!
//! - Rich TUI with panels and borders
//! - Progress indicator showing current question
//! - Keyboard navigation (arrow keys, Enter, Esc)
//! - Real-time validation with error display
//! - Customizable color themes
//! - Support for all question types (input, select, multi-select, confirm, etc.)
//!
//! ## Example
//!
//! ```ignore
//! use elicitor::Survey;
//! use elicitor_wizard_ratatui::RatatuiBackend;
//!
//! #[derive(Survey)]
//! struct User {
//!     #[ask("What is your name?")]
//!     name: String,
//!
//!     #[ask("How old are you?")]
//!     age: i64,
//! }
//!
//! fn main() -> anyhow::Result<()> {
//!     let backend = RatatuiBackend::new()
//!         .with_title("User Registration");
//!     let user = User::builder().run(backend)?;
//!     println!("Hello, {} ({} years old)!", user.name, user.age);
//!     Ok(())
//! }
//! ```

mod backend;

pub use backend::{RatatuiBackend, RatatuiError, Theme};
