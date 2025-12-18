# Testing Terminal Applications with PTY

Since `derive-wizard` generates interactive CLI wizards, standard stdin piping doesn't work for automated tests. Terminal libraries require proper TTY handling for automated testing.

## ✅ WORKING SOLUTION: Use Dialoguer

**We found a working solution!** The `dialoguer` library has excellent PTY support and works perfectly with automated tests.

See **[PTY_WORKAROUND_SOLUTION.md](PTY_WORKAROUND_SOLUTION.md)** for the complete working implementation.

### Quick Summary

- ✅ All PTY tests pass reliably with `dialoguer`
- ✅ 3 automated integration tests working
- ✅ Clean API similar to requestty
- ✅ Easy migration path

```bash
$ cargo test --test dialoguer_pty_test
running 3 tests
test test_dialoguer_with_different_values ... ok
test test_dialoguer_with_false_boolean ... ok
test test_dialoguer_with_pty ... ok

test result: ok. 3 passed; 0 failed; 0 ignored
```

## The Original Problem with Requestty

## The Problem

```rust
// This DOESN'T work:
let output = Command::new("my-wizard")
    .stdin(piped_input)  // ❌ requestty needs a real terminal
    .output();
```

Error: `"The cursor position could not be read within a normal duration"`

## Current Limitation ⚠️

**Unfortunately, `requestty` doesn't work reliably in PTY-based automated tests either.**

The library performs terminal capability checks (like cursor position detection) that fail even with pseudo-terminals created by libraries like `expectrl` or `rexpect`. The generated wizard code calls `.unwrap()` on `requestty::prompt_one()`, which panics when these terminal checks fail.

### What We Tried

We implemented PTY-based tests using `expectrl`:

```rust
use expectrl::{spawn, Expect, Regex};

#[test]
#[ignore] // Doesn't work due to requestty limitations
fn test_wizard() -> Result<(), Box<dyn std::error::Error>> {
    let mut session = spawn("cargo run --example test_user_input")?;
    session.expect(Regex("What is your name"))?;
    session.send_line("Alice")?;
    // ...
}
```

**Result**: Tests fail because `requestty` panics when it can't detect cursor position in the PTY environment.

## Current Testing Strategy

For now, testing `derive-wizard` applications involves:

1. **Compilation tests** - Verify that the `Wizard` trait derives correctly:

   ```rust
   #[test]
   fn test_wizard_compiles() {
       #[derive(Wizard)]
       struct User {
           #[prompt("Name:")]
           name: String,
       }
       let _has_wizard: fn() -> User = User::wizard;
   }
   ```

2. **Manual interactive testing** - Run examples manually:

   ```bash
   cargo run --example test_user_input
   cargo run --example test_enum_input
   ```

3. **PTY tests marked as ignored** - We have PTY test examples in `tests/pty_tests.rs` that demonstrate how automated testing WOULD work, but they're marked as `#[ignore]` since they don't currently work with `requestty`.

## Future Solutions

If you need automated testing for wizard-based applications, consider:

1. **Switch to a different prompting library** that supports stdin mocking or has better PTY support (e.g., `dialoguer` with feature flags)

2. **Wrap wizard calls** to make them testable:

   ```rust
   trait WizardRunner {
       fn run_wizard<T: Wizard>() -> T;
   }
   
   // Real implementation
   struct InteractiveWizard;
   impl WizardRunner for InteractiveWizard {
       fn run_wizard<T: Wizard>() -> T {
           T::wizard()
       }
   }
   
   // Test implementation with mocked data
   struct MockWizard<T>(T);
   impl<T: Wizard> WizardRunner for MockWizard<T> {
       fn run_wizard<U: Wizard>() -> U {
           // Return pre-constructed mock data
       }
   }
   ```

3. **Contribute to `requestty`** to add better test support or PTY compatibility

4. **Use environment variable detection** to skip interactive prompts in test mode

## Reference: PTY Libraries (For Future Use)

Below are examples of how PTY testing would work if `requestty` supported it. These are kept as reference documentation.

## Option 1: Using `rexpect` (Unix-only, Expect-like API)

### Setup

Add to `Cargo.toml`:

```toml
[dev-dependencies]
rexpect = "0.5"
```

### Example Test

```rust
use rexpect::spawn;

#[test]
fn test_wizard_with_pty() {
    // Spawn with PTY (timeout in ms)
    let mut p = spawn("cargo run --example test_user_input", Some(5000))
        .expect("Failed to spawn");

    // Expect prompt and respond
    p.exp_string("What is your name?").unwrap();
    p.send_line("Alice").unwrap();

    p.exp_string("How old are you?").unwrap();
    p.send_line("30").unwrap();

    p.exp_string("Do you like Rust?").unwrap();
    p.send_line("y").unwrap();

    // Verify output
    p.exp_string("RESULT: name=Alice, age=30, likes_rust=true").unwrap();
}
```

### Pros

- Simple Expect-like API
- Good for scripted interactions

### Cons

- Unix/Linux only (uses Unix PTY)
- Not suitable for Windows

## Option 2: Using `expectrl` (Cross-platform, Modern)

### Setup

Add to `Cargo.toml`:

```toml
[dev-dependencies]
expectrl = "0.7"
```

### Example Test

```rust
use expectrl::{spawn, Regex};
use std::time::Duration;

#[test]
fn test_wizard_with_expectrl() -> Result<(), Box<dyn std::error::Error>> {
    let mut session = spawn("cargo run --example test_user_input")?;
    session.set_expect_timeout(Some(Duration::from_secs(10)));

    // Use regex patterns for more flexible matching
    session.expect(Regex("What is your name"))?;
    session.send_line("Bob")?;

    session.expect(Regex("How old are you"))?;
    session.send_line("25")?;

    session.expect(Regex("Do you like Rust"))?;
    session.send_line("y")?;

    session.expect(Regex("RESULT: name=Bob"))?;

    Ok(())
}
```

### Pros

- Cross-platform (works on Windows, macOS, Linux)
- Modern async-friendly API
- Better error handling

### Cons

- Slightly more complex API
- Heavier dependency

## Option 3: Using `portable-pty` (Low-level)

For full control, use `portable-pty` to create PTYs manually:

```toml
[dev-dependencies]
portable-pty = "0.8"
```

```rust
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{BufRead, BufReader, Write};

#[test]
fn test_with_portable_pty() {
    let pty_system = native_pty_system();
    let pair = pty_system.openpty(PtySize::default()).unwrap();
    
    let mut cmd = CommandBuilder::new("cargo");
    cmd.args(&["run", "--example", "test_user_input"]);
    
    let mut child = pair.slave.spawn_command(cmd).unwrap();
    
    let mut reader = pair.master.try_clone_reader().unwrap();
    let mut writer = pair.master.take_writer().unwrap();
    
    // Read until prompt, write response, etc.
    // (More manual control but more work)
}
```

## Recommended Approach

For `derive-wizard` tests, I recommend **`expectrl`** because:

1. ✅ Cross-platform (important for CI/CD)
2. ✅ Active maintenance
3. ✅ Good balance of ease-of-use and power
4. ✅ Works well with `requestty`

## Running PTY Tests

```bash
# Add expectrl to dev-dependencies
cargo add --dev expectrl

# Run the PTY tests
cargo test --test pty_tests

# Run a specific test
cargo test --test pty_tests test_with_expectrl
```

## Example: Complete Working Test

```rust
// tests/pty_integration.rs
use expectrl::{spawn, Regex};
use std::time::Duration;

#[test]
fn test_complete_wizard_flow() -> Result<(), Box<dyn std::error::Error>> {
    let mut p = spawn("cargo run --example test_enum_input")?;
    p.set_expect_timeout(Some(Duration::from_secs(10)));

    // Variant selection
    p.expect(Regex("Select variant"))?;
    p.send_line("Card")?;

    // Card details
    p.expect(Regex("Card number"))?;
    p.send_line("1234-5678-9012-3456")?;

    p.expect(Regex("CVV"))?;
    p.send_line("123")?;

    // Verify result
    p.expect(Regex("RESULT: Card number=1234-5678-9012-3456"))?;
    p.expect(Regex("cvv_len=3"))?;

    Ok(())
}
```

## Alternative: Manual Testing Scripts

If you don't want PTY dependencies, create test scripts:

```bash
#!/bin/bash
# test_wizard.sh

expect <<EOF
spawn cargo run --example test_user_input
expect "What is your name?"
send "Alice\r"
expect "How old are you?"
send "30\r"
expect "Do you like Rust?"
send "y\r"
expect "RESULT: name=Alice"
EOF
```

Run with: `expect test_wizard.sh`

## CI/CD Considerations

For GitHub Actions or other CI:

```yaml
- name: Install expect (for PTY tests)
  run: sudo apt-get install -y expect

- name: Run PTY tests
  run: cargo test --test pty_tests
```

Or just use `expectrl` which doesn't need system `expect`.

## Summary

| Approach | Cross-Platform | Ease of Use | Best For |
|----------|---------------|-------------|----------|
| `rexpect` | ❌ Unix only | ⭐⭐⭐ | Simple Linux tests |
| `expectrl` | ✅ Yes | ⭐⭐ | Production tests |
| `portable-pty` | ✅ Yes | ⭐ | Custom needs |
| Shell `expect` | ❌ Unix only | ⭐⭐⭐ | Quick scripts |

**Recommendation: Use `expectrl` for automated tests in `derive-wizard`.**
