//! # derive-html-document
//!
//! HTML document generator for derive-survey.
//!
//! This crate generates fillable HTML forms from survey definitions.
//! It does NOT collect responses — use it to generate static HTML forms
//! that can be served, printed, or processed by other tools.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use derive_survey::Survey;
//! use derive_html_document::to_html;
//!
//! #[derive(Survey)]
//! struct UserProfile {
//!     #[ask("What is your name?")]
//!     name: String,
//!
//!     #[ask("How old are you?")]
//!     #[min(0)]
//!     #[max(150)]
//!     age: i64,
//! }
//!
//! fn main() {
//!     let html = to_html::<UserProfile>(Some("User Profile"));
//!     std::fs::write("form.html", html).unwrap();
//! }
//! ```

mod generator;

pub use generator::{HtmlOptions, to_html, to_html_with_options};
