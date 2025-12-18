# PTY Testing Workaround: Using Dialoguer

## Problem Summary

The `requestty` library doesn't work in PTY (pseudo-terminal) environments because it performs terminal capability checks (cursor position detection) that fail in automated testing scenarios.

## Solution: Use Dialoguer Instead

The `dialoguer` crate has excellent PTY support and works perfectly with automated testing using `expectrl`.

### ✅ Verified Working

All tests pass reliably:

```bash
$ cargo test --test dialoguer_pty_test
running 3 tests
test test_dialoguer_with_different_values ... ok
test test_dialoguer_with_false_boolean ... ok
test test_dialoguer_with_pty ... ok

test result: ok. 3 passed; 0 failed; 0 ignored
```

## Implementation

### 1. Add Dialoguer as Optional Dependency

```toml
[dependencies]
dialoguer = { version = "0.12.0", optional = true }

[features]
dialoguer = ["dep:dialoguer"]
```

### 2. Create Dialoguer Backend Module

See [src/dialoguer_backend.rs](derive-wizard/src/dialoguer_backend.rs) for the implementation.

Key functions:

- `prompt_string(message: &str) -> String`
- `prompt_bool(message: &str) -> bool`
- `prompt_number<T>(message: &str) -> T`

### 3. Write PTY Tests

```rust
use expectrl::{spawn, Regex, Expect};

#[test]
fn test_with_pty() {
    let mut session = spawn("./target/debug/examples/my_wizard")
        .expect("Failed to spawn process");
    
    // Wait for prompt and send input
    session.expect(Regex("Enter your name")).unwrap();
    session.send_line("Alice").unwrap();
    
    // Verify output
    session.expect(Regex("RESULT: name=Alice")).unwrap();
}
```

## Advantages of Dialoguer

1. **PTY Support**: Works perfectly in automated testing environments
2. **Modern API**: Clean, simple interface
3. **Feature Rich**: Supports all common input types
4. **Active Maintenance**: Well-maintained crate
5. **Test Utilities**: Built-in support for testing via `console` crate

## Comparison

| Feature | requestty | dialoguer |
|---------|-----------|-----------|
| PTY Support | ❌ Fails | ✅ Works |
| String Input | ✅ | ✅ |
| Number Input | ✅ | ✅ |
| Boolean Input | ✅ | ✅ |
| Select Menus | ✅ | ✅ |
| Testing | ❌ Difficult | ✅ Easy |

## Migration Path

For users who need automated testing:

1. Enable the `dialoguer` feature
2. Implement `DialoguerWizard` trait instead of (or in addition to) `Wizard`
3. Use the dialoguer backend functions
4. Write PTY tests using `expectrl`

## Example

See:

- [examples/test_dialoguer.rs](derive-wizard/examples/test_dialoguer.rs) - Implementation
- [tests/dialoguer_pty_test.rs](derive-wizard/tests/dialoguer_pty_test.rs) - PTY tests

## Future Considerations

### Option 1: Make Dialoguer the Default

Switch from `requestty` to `dialoguer` as the primary backend since it has better testing support.

### Option 2: Dual Backend Support

Support both libraries via feature flags, letting users choose:

```toml
[features]
default = ["requestty"]
requestty = ["dep:requestty"]
dialoguer = ["dep:dialoguer"]
```

### Option 3: Proc Macro Code Generation

Generate different code based on feature flags so the derive macro works with either backend.

## Recommendation

**Use dialoguer for projects that need automated testing.** It's a superior choice for CLI tools that require PTY-based integration tests.
