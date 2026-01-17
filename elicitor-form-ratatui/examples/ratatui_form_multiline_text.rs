//! Multiline text example
//!
//! Demonstrates:
//! - #[multiline] attribute for text areas
//! - Multi-line input handling
//!
//! Run with: cargo run -p derive-ratatui-form --example ratatui_form_multiline_text

use elicitor_form_ratatui::RatatuiFormBackend;
use example_surveys::BlogPost;

fn main() -> anyhow::Result<()> {
    let backend = RatatuiFormBackend::new();
    let result = BlogPost::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
