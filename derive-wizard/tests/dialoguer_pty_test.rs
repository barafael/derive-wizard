use expectrl::{Expect, Regex, spawn};
use std::path::Path;

#[test]
fn test_dialoguer_with_pty() {
    // Build the example first
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Could not get workspace root");

    let example_path = workspace_root.join("target/debug/examples/test_dialoguer");

    // Ensure it's built
    let build_status = std::process::Command::new("cargo")
        .args(&[
            "build",
            "--example",
            "test_dialoguer",
            "--features",
            "dialoguer",
        ])
        .current_dir(workspace_root)
        .status()
        .expect("Failed to build example");

    assert!(build_status.success(), "Failed to build example");

    // Spawn the process with PTY
    let mut session = spawn(example_path.to_str().unwrap()).expect("Failed to spawn process");

    // Wait for "Enter your name" prompt
    session
        .expect(Regex("Enter your name"))
        .expect("Did not find name prompt");

    // Send name
    session.send_line("Alice").expect("Failed to send name");

    // Wait for age prompt
    session
        .expect(Regex("Enter your age"))
        .expect("Did not find age prompt");

    // Send age
    session.send_line("30").expect("Failed to send age");

    // Wait for Rust question
    session
        .expect(Regex("Do you like Rust"))
        .expect("Did not find Rust question");

    // Send yes (dialoguer accepts y/n)
    session.send_line("y").expect("Failed to send answer");

    // Check the final output
    session
        .expect(Regex("RESULT: name=Alice, age=30, likes_rust=true"))
        .expect("Did not find expected result");

    println!("✅ Dialoguer PTY test passed!");
}

#[test]
fn test_dialoguer_with_false_boolean() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Could not get workspace root");

    let example_path = workspace_root.join("target/debug/examples/test_dialoguer");

    let mut session = spawn(example_path.to_str().unwrap()).expect("Failed to spawn process");

    session.expect(Regex("Enter your name")).unwrap();
    session.send_line("Bob").unwrap();

    session.expect(Regex("Enter your age")).unwrap();
    session.send_line("25").unwrap();

    session.expect(Regex("Do you like Rust")).unwrap();
    session.send_line("n").unwrap();

    session
        .expect(Regex("RESULT: name=Bob, age=25, likes_rust=false"))
        .unwrap();
}

#[test]
fn test_dialoguer_with_different_values() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Could not get workspace root");

    let example_path = workspace_root.join("target/debug/examples/test_dialoguer");

    let mut session = spawn(example_path.to_str().unwrap()).expect("Failed to spawn process");

    session.expect(Regex("Enter your name")).unwrap();
    session.send_line("Charlie Brown").unwrap();

    session.expect(Regex("Enter your age")).unwrap();
    session.send_line("42").unwrap();

    session.expect(Regex("Do you like Rust")).unwrap();
    session.send_line("yes").unwrap();

    session
        .expect(Regex("RESULT: name=Charlie Brown, age=42, likes_rust=true"))
        .unwrap();
}
