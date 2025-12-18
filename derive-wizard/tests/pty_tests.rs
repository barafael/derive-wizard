// Automated PTY-based tests for wizard functionality
//
// NOTE: These tests currently don't work reliably because `requestty` has issues
// running in PTY environments. The library performs terminal capability checks
// (like cursor position detection) that fail in automated PTY sessions.
//
// These tests are marked as #[ignore] and serve as documentation for how PTY
// testing WOULD work if requestty supported it better.
//
// To run interactively: cargo run --example test_user_input

use expectrl::{Expect, Regex, spawn};
use std::time::Duration;

#[test]
#[ignore = "requestty doesn't work reliably in PTY - see TESTING_WITH_PTY.md"]
fn test_user_input_with_pty() -> Result<(), Box<dyn std::error::Error>> {
    // Build from workspace root
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();

    let output = std::process::Command::new("cargo")
        .current_dir(workspace_root)
        .args(&["build", "--example", "test_user_input"])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Failed to compile example: {:?}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    // Run the compiled binary with PTY
    let binary_path = workspace_root.join("target/debug/examples/test_user_input");
    let mut session = spawn(binary_path.to_str().unwrap())?;
    session.set_expect_timeout(Some(Duration::from_secs(10)));

    // Wait for the first prompt and send input
    session.expect(Regex("What is your name"))?;
    session.send_line("Alice")?;

    // Wait for age prompt and respond
    session.expect(Regex("How old are you"))?;
    session.send_line("30")?;

    // Wait for boolean prompt and respond
    session.expect(Regex("Do you like Rust"))?;
    session.send_line("y")?;

    // Verify the output contains our expected result
    session.expect(Regex("RESULT: name=Alice, age=30, likes_rust=true"))?;

    Ok(())
}

#[test]
#[ignore = "requestty doesn't work reliably in PTY"]
fn test_enum_cash_variant() -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("cargo")
        .args(&["build", "--example", "test_enum_input"])
        .output()?;

    let mut session = spawn("target/debug/examples/test_enum_input")?;
    session.set_expect_timeout(Some(Duration::from_secs(10)));

    // Wait for variant selection prompt
    session.expect(Regex("Select variant"))?;

    // Send "Cash" selection
    session.send_line("Cash")?;

    // Verify the output
    session.expect(Regex("RESULT: Cash"))?;

    Ok(())
}

#[test]
#[ignore = "requestty doesn't work reliably in PTY"]
fn test_enum_card_variant() -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("cargo")
        .args(&["build", "--example", "test_enum_input"])
        .output()?;

    let mut session = spawn("target/debug/examples/test_enum_input")?;
    session.set_expect_timeout(Some(Duration::from_secs(10)));

    // Select Card variant
    session.expect(Regex("Select variant"))?;
    session.send_line("Card")?;

    // Enter card number
    session.expect(Regex("Card number"))?;
    session.send_line("1234-5678-9012-3456")?;

    // Enter CVV (masked field)
    session.expect(Regex("CVV"))?;
    session.send_line("123")?;

    // Verify the output
    session.expect(Regex("RESULT: Card number=1234-5678-9012-3456"))?;
    session.expect(Regex("cvv_len=3"))?;

    Ok(())
}

// Additional test for different input values
#[test]
#[ignore = "requestty doesn't work reliably in PTY"]
fn test_user_input_with_different_values() -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("cargo")
        .args(&["build", "--example", "test_user_input"])
        .output()?;

    let mut session = spawn("target/debug/examples/test_user_input")?;
    session.set_expect_timeout(Some(Duration::from_secs(10)));

    session.expect(Regex("What is your name"))?;
    session.send_line("Bob")?;

    session.expect(Regex("How old are you"))?;
    session.send_line("25")?;

    session.expect(Regex("Do you like Rust"))?;
    session.send_line("y")?;

    session.expect(Regex("RESULT: name=Bob, age=25, likes_rust=true"))?;

    Ok(())
}

#[test]
#[ignore = "requestty doesn't work reliably in PTY"]
fn test_user_input_boolean_no() -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("cargo")
        .args(&["build", "--example", "test_user_input"])
        .output()?;

    let mut session = spawn("target/debug/examples/test_user_input")?;
    session.set_expect_timeout(Some(Duration::from_secs(10)));

    session.expect(Regex("What is your name"))?;
    session.send_line("Charlie")?;

    session.expect(Regex("How old are you"))?;
    session.send_line("35")?;

    session.expect(Regex("Do you like Rust"))?;
    session.send_line("n")?;

    session.expect(Regex("RESULT: name=Charlie, age=35, likes_rust=false"))?;

    Ok(())
}
