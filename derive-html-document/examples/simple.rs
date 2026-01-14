//! Simple example - generate an HTML form from a basic survey.
//!
//! Run with: cargo run -p derive-html-document --example simple

use derive_html_document::to_html;
use example_surveys::HtmlUserProfile;

fn main() {
    let html = to_html::<HtmlUserProfile>(Some("User Profile"));

    // Write to file
    std::fs::write("user_profile.html", &html).expect("Failed to write HTML file");

    println!("Generated user_profile.html");
}
