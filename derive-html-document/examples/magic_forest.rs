//! Magic Forest example - generate a comprehensive HTML form.
//!
//! Run with: cargo run -p derive-html-document --example magic_forest

use derive_html_document::{HtmlOptions, to_html_with_options};
use example_surveys::HtmlMagicForest;

fn main() {
    let options = HtmlOptions::new()
        .with_title("Magic Forest Adventure")
        .with_styles(true)
        .full_document(true);

    let html = to_html_with_options::<HtmlMagicForest>(options);

    // Write to file
    std::fs::write("magic_forest.html", &html).expect("Failed to write HTML file");
}
