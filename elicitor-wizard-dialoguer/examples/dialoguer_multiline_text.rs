//! Multiline text input example. Run with: cargo run --example multiline_text

use elicitor_wizard_dialoguer::DialoguerBackend;
use example_surveys::BlogPost;

fn main() -> anyhow::Result<()> {
    let backend = DialoguerBackend::new();
    let result = BlogPost::builder().run(backend)?;
    println!("{result:#?}");
    Ok(())
}
