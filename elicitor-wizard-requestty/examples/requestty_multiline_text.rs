//! Multiline text input example
//!
//! Demonstrates:
//! - #[multiline] attribute for opening a text editor or textarea
//!
//! Run with: cargo run --example multiline_text

use elicitor_wizard_requestty::RequesttyBackend;
use example_surveys::BlogPost;

fn main() -> anyhow::Result<()> {
    let backend = RequesttyBackend::new();
    let result = BlogPost::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
