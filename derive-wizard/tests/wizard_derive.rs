#![allow(unused)]

use derive_wizard::Wizard;
use std::path::PathBuf;

// NOTE: Automated testing with simulated input is not fully supported because requestty
// requires a real TTY for terminal interactions. The library checks for cursor position
// and other terminal capabilities that don't work with piped stdin in CI environments.
//
// For information on how to write automated tests using PTY (pseudo-terminals),
// see: /TESTING_WITH_PTY.md
//
// TL;DR: Use `expectrl` or `rexpect` to create PTY-based tests:
//   cargo add --dev expectrl
//
//   use expectrl::{spawn, Regex};
//   let mut p = spawn("cargo run --example test_user_input")?;
//   p.expect(Regex("What is your name"))?;
//   p.send_line("Alice")?;
//
// To test interactively with examples, run:
//   cargo run --example test_user_input
//   cargo run --example test_enum_input
//
// These compilation tests verify that the Wizard trait is correctly derived
// and the generated code compiles for various use cases.

#[test]
fn basic_struct_derives_wizard_trait() {
    // Test that a struct with all supported types can derive Wizard
    #[derive(Debug, Wizard, PartialEq)]
    struct TestStruct {
        #[prompt("Enter name:")]
        name: String,

        #[prompt("Enter age:")]
        age: u32,

        #[prompt("Enter height:")]
        height: f64,

        #[prompt("Agree?")]
        agree: bool,
    }

    let _has_wizard_method: fn() -> TestStruct = TestStruct::wizard;
}

#[test]
fn enum_derives_wizard_trait() {
    #[derive(Debug, Wizard, PartialEq)]
    enum Color {
        Red,
        Green,
        Blue,
        Custom(#[prompt("Enter custom color:")] String),
    }

    // Verify the trait is implemented
    let _has_wizard_method: fn() -> Color = Color::wizard;
    let _has_wizard_with_message: fn(&str) -> Color = Color::wizard_with_message;
}

#[test]
fn nested_wizard_struct() {
    #[derive(Debug, Wizard, PartialEq)]
    struct Inner {
        #[prompt("Enter value:")]
        value: String,
    }

    #[derive(Debug, Wizard, PartialEq)]
    struct Outer {
        #[prompt("Enter name:")]
        name: String,

        #[prompt]
        inner: Inner,
    }

    // Verify both have the Wizard trait
    let _inner_wizard: fn() -> Inner = Inner::wizard;
    let _outer_wizard: fn() -> Outer = Outer::wizard;
}

#[test]
fn pathbuf_support() {
    #[derive(Debug, Wizard)]
    struct FileConfig {
        #[prompt("Enter file path:")]
        path: PathBuf,
    }

    // Verify PathBuf is supported
    let _has_wizard_method: fn() -> FileConfig = FileConfig::wizard;
}

#[test]
fn mask_attribute_compiles() {
    #[derive(Debug, Wizard)]
    struct LoginForm {
        #[prompt("Username:")]
        username: String,

        #[prompt("Password:")]
        #[mask]
        password: String,
    }

    let _has_wizard_method: fn() -> LoginForm = LoginForm::wizard;
}

#[test]
fn editor_attribute_compiles() {
    #[derive(Debug, Wizard)]
    struct Article {
        #[prompt("Title:")]
        title: String,

        #[prompt("Content:")]
        #[editor]
        content: String,
    }

    let _has_wizard_method: fn() -> Article = Article::wizard;
}

#[test]
fn all_numeric_types() {
    #[derive(Debug, Wizard)]
    struct NumericTypes {
        #[prompt("u8:")]
        val_u8: u8,

        #[prompt("u16:")]
        val_u16: u16,

        #[prompt("u32:")]
        val_u32: u32,

        #[prompt("u64:")]
        val_u64: u64,

        #[prompt("i8:")]
        val_i8: i8,

        #[prompt("i16:")]
        val_i16: i16,

        #[prompt("i32:")]
        val_i32: i32,

        #[prompt("i64:")]
        val_i64: i64,

        #[prompt("f32:")]
        val_f32: f32,

        #[prompt("f64:")]
        val_f64: f64,
    }

    let _has_wizard_method: fn() -> NumericTypes = NumericTypes::wizard;
}

#[test]
fn enum_with_named_fields() {
    #[derive(Debug, Wizard)]
    enum Payment {
        Cash,
        CreditCard {
            #[prompt("Card number:")]
            number: String,

            #[prompt("CVV:")]
            #[mask]
            cvv: String,
        },
    }

    let _has_wizard_method: fn() -> Payment = Payment::wizard;
    let _has_wizard_with_message: fn(&str) -> Payment = Payment::wizard_with_message;
}

#[test]
fn deeply_nested_wizards() {
    #[derive(Debug, Wizard)]
    struct Level3 {
        #[prompt("Level 3 value:")]
        value: String,
    }

    #[derive(Debug, Wizard)]
    struct Level2 {
        #[prompt("Level 2 value:")]
        value: String,

        #[prompt]
        level3: Level3,
    }

    #[derive(Debug, Wizard)]
    struct Level1 {
        #[prompt("Level 1 value:")]
        value: String,

        #[prompt]
        level2: Level2,
    }

    let _has_wizard_method: fn() -> Level1 = Level1::wizard;
}

// This test demonstrates the expected usage pattern with simulated input.
// Note: requestty library doesn't easily support stdin mocking in unit tests,
// so this is more of a documentation/integration test example.
// To actually test with input, run:
//   echo -e "John Doe\n30\ny" | cargo test --test wizard_derive user_input_example -- --ignored --nocapture
#[test]
#[ignore] // Ignored by default since it requires manual stdin input
fn user_input_example() {
    #[derive(Debug, Wizard, PartialEq)]
    struct UserProfile {
        #[prompt("What is your name?")]
        name: String,

        #[prompt("How old are you?")]
        age: u32,

        #[prompt("Do you like Rust?")]
        likes_rust: bool,
    }

    // This would read from stdin when run interactively:
    // Expected input format:
    //   - Name: "John Doe"
    //   - Age: 30
    //   - Likes Rust: y (for yes) or n (for no)

    let profile = UserProfile::wizard();

    println!("Created profile: {:#?}", profile);

    // Verify the profile was created
    assert!(!profile.name.is_empty());
}

#[test]
#[ignore]
fn enum_input_example() {
    #[derive(Debug, Wizard)]
    enum PaymentMethod {
        Cash,
        Card {
            #[prompt("Card number:")]
            number: String,

            #[prompt("CVV:")]
            #[mask]
            cvv: String,
        },
    }

    // Expected input:
    //   - Select variant: "Card" (or use arrow keys)
    //   - Card number: "1234-5678-9012-3456"
    //   - CVV: "123"

    let payment = PaymentMethod::wizard();

    println!("Payment method selected: {:#?}", payment);
}
