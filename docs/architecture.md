# Elicitor Architecture

This document describes the architecture of the **elicitor** crate (formerly known as `derive-wizard`).

A "survey" is a structured collection of questions. It's presentation-agnostic.
It can be viewed as a
sequential interview or a
fill-in form or just
data for generating a document.

## Usage Example

```rust
use elicitor::{Survey, SurveyBuilder};

#[derive(Survey, Debug)]
#[prelude("A journey begins...!")]
#[epilogue("Good luck.")]
struct MySurvey {
    #[ask("What is your name?")]
    #[validate("is_valid_name")]
    name: String,

    #[ask("What's the secret passphrase?")]
    #[mask]
    passphrase: String,

    #[ask("How old are you?")]
    #[min(18)]
    #[max(233)]
    age: u32,

    #[ask("What is your role?")]
    role: Role,

    #[ask("Pick your inventory:")]
    #[multiselect]
    #[validate("is_within_starting_budget")]
    inventory: Vec<Item>,
}

#[derive(Survey, Debug)]
enum Role {
    Streetfighter,
    Mage,
    Archer,
    Thief,
    Other(#[ask("What then?!")] String)
}

#[derive(Survey, Debug)]
enum Item {
    #[ask("Sword (value: 80)")]
    Sword,

    #[ask("Shield (value: 50)")]
    Shield,

    #[ask("Potion (value: 20)")]
    Potion,

    #[ask("Scroll (value: 10)")]
    Scroll,

    #[ask("Chewing Gum (value: 2 * quantity)")]
    ChewingGum {
        flavor: String,
        #[ask("How many?")]
        quantity: u32,
    },
}

impl Item {
    fn value(&self) -> u32 {
        match self {
            Item::Sword => 80,
            Item::Shield => 50,
            Item::Potion => 20,
            Item::Scroll => 10,
            Item::ChewingGum { flavor: _, quantity } => 2 * quantity,
        }
    }
}

fn is_valid_name(
    value: &elicitor::ResponseValue,
    _responses: &elicitor::Responses,
    _path: &elicitor::ResponsePath,
) -> Result<(), String> {
    let name = value.as_string().unwrap_or("");
    if name.len() > 2 && name.len() < 100 {
        Ok(())
    } else {
        Err("Name must be between 3 and 99 characters".to_string())
    }
}

fn is_within_starting_budget(
    value: &elicitor::ResponseValue,
    _responses: &elicitor::Responses,
    _path: &elicitor::ResponsePath,
) -> Result<(), String> {
    // For Vec<Item>, validation happens after reconstruction
    // This is a placeholder - real validation would check the collected items
    Ok(())
}

fn main() {
    // Run the survey with the requestty backend
    let survey_result = MySurvey::builder()
        .run(elicitor_wizard_requestty::RequesttyWizard::new())
        .unwrap();

    println!("{:#?}", survey_result);
}
```

### Who-is-who

| Element         | Name                |
|-----------------|---------------------|
| Crate           | `elicitor`          |
| Derive macro    | `#[derive(Survey)]` |
| Trait           | `Survey`            |
| Data structure  | `SurveyDefinition`  |
| Individual item | `Question`          |
| Item variants   | `QuestionKind`      |
| Collected data  | `Responses`         |
| Builder         | `TypeBuilder`       |
| Backend trait   | `SurveyBackend`     |

### Proc-macro Attributes

| Attribute                    | Purpose                                        |
|------------------------------|------------------------------------------------|
| `#[ask("...")]`              | The prompt text shown to the user              |
| `#[mask]`                    | Hide input (for passwords)                     |
| `#[multiline]`               | Open text editor / show textarea               |
| `#[validate("fn")]`          | Custom validation function                     |
| `#[validate_fields("fn")]`   | Propagate validator to all numeric child fields|
| `#[min(n)]` / `#[max(n)]`    | Numeric bounds                                 |
| `#[multiselect]`             | Enable multi-select for `Vec<Enum>` fields     |
| `#[prelude("...")]`          | Message before survey starts                   |
| `#[epilogue("...")]`         | Message after survey completes                 |

## Two Interaction Models

The design supports two fundamentally different interaction paradigms:

### Sequential (Wizard-style)

**Backends:** `elicitor-wizard-requestty`, `elicitor-wizard-dialoguer`, `elicitor-wizard-ratatui`

**Characteristics:**

- One question at a time
- User answers, then moves to next
- Linear flow, no back-navigation
- Validation per-field before proceeding
- Natural for CLI prompts

### Form-style

**Backends:** `elicitor-form-ratatui`, `elicitor-form-egui`

**Characteristics:**

- All fields visible simultaneously
- User can fill in any order
- Jump between fields freely
- Validation as user types, submit only possible once inputs are valid
- All validation errors are shown immediately and simultaneously
- Inter-field conditions (such as "passwords entered must match") are validated as user types
- Natural for GUIs and TUIs

## Crate Structure

### Core Crates

Users only need to depend on `elicitor` for access to `#[derive(Survey)]`.
The macro crate cannot export types, so the split is necessary.
The main crate re-exports everything so generated code works without users adding `elicitor-macro` and `elicitor-types` manually.

The main crate does NOT include any backend implementations (except a private `TestBackend` for testing).

```
elicitor/
├── elicitor/           # Main crate - re-exports types and macro
├── elicitor-types/     # Core data structures and traits
├── elicitor-macro/     # Procedural macro implementation
```

### Backend Crates

Pattern: `elicitor-{style}-{library}`

**Wizard-style backends:**

```
elicitor-wizard-requestty     # CLI prompts via requestty
elicitor-wizard-dialoguer     # CLI prompts via dialoguer
elicitor-wizard-ratatui       # TUI wizard with step-by-step flow
```

**Form-style backends:**

```
elicitor-form-ratatui         # TUI form with field navigation
elicitor-form-egui            # GUI form via egui
```

### Document Generator Crates

Document generators transform a `SurveyDefinition` into a document format.
They depend on `elicitor` but don't implement `SurveyBackend` (they're not interactive).

```
elicitor-doc-latex            # Generates LaTeX markup
elicitor-doc-html             # Generates HTML forms
```

```rust
// Example usage
use elicitor::Survey;
use elicitor_doc_html::to_html;

let html = to_html::<MySurvey>(Some("Registration Form"));
std::fs::write("form.html", html)?;
```

### Backend Autonomy

Backends decide how to present a `SurveyDefinition`.

```rust
pub trait SurveyBackend {
    type Error: Into<anyhow::Error>;

    fn collect(
        &self,
        definition: &SurveyDefinition,
        validate: &dyn Fn(&ResponseValue, &Responses, &ResponsePath) -> Result<(), String>,
    ) -> Result<Responses, Self::Error>;
}
```

A wizard backend iterates through questions sequentially. A form backend renders all fields at once.
The trait doesn't care — it just takes a `SurveyDefinition` and returns `Responses`.

Each backend crate:

1. Depends on `elicitor` for the `SurveyDefinition` structure and `SurveyBackend` trait
2. Decides independently how to present the survey to the user
3. Can implement `SurveyBackend` however it sees fit
4. Is responsible for its own dependencies (ratatui, egui, etc.)

## Dependency Graph

### User Application

The user depends on `elicitor` for the proc-macro, and on a selected backend crate.

```
user-app/
├── Cargo.toml
│   └── [dependencies]
│       ├── elicitor = "0.6"
│       ├── elicitor-wizard-requestty = "0.6"  # Example backend choice
│       └── elicitor-doc-html = "0.6"          # Optional document generator
```

### Internal Structure

```
elicitor
├── Cargo.toml
│   └── [dependencies]
│       ├── elicitor-types = "0.6"
│       └── elicitor-macro = "0.6"
│
├── lib.rs
│   ├── pub use elicitor_types::*;        // Re-export all types
│   ├── pub use elicitor_macro::Survey;   // Re-export #[derive(Survey)]
│   └── mod test_backend;                 // TestBackend (private, for testing)
│
└── (dependencies)
    │
    ├─ elicitor-types/
    │  └── lib.rs
    │      ├── struct SurveyDefinition { ... }
    │      ├── struct Question { ... }
    │      ├── enum QuestionKind { ... }
    │      ├── struct Responses { ... }
    │      ├── struct ResponsePath { ... }
    │      ├── enum ResponseValue { ... }
    │      ├── trait Survey { ... }
    │      └── trait SurveyBackend { ... }
    │
    └─ elicitor-macro/
       └── lib.rs
           └── #[proc_macro_derive(Survey, attributes(...))]
```

### Key Dependency Rules

1. **elicitor-types**: No dependencies on other elicitor crates. Pure data structures and traits.

2. **elicitor-macro**: Depends on elicitor-types. Generates code using types that will be re-exported by elicitor.

3. **elicitor** (facade): Re-exports everything from types and macro. Single dependency for users.

4. **Backend crates**: Depend on elicitor. Each implements SurveyBackend trait. No interdependencies between backends.

5. **Document crates**: Depend on elicitor. Transform SurveyDefinition into documents. Do NOT implement SurveyBackend.

## Core Types

### SurveyDefinition

The top-level structure containing all questions and metadata:

```rust
pub struct SurveyDefinition {
    pub prelude: Option<String>,
    pub questions: Vec<Question>,
    pub epilogue: Option<String>,
}
```

### ResponsePath

Typed paths for response values:

```rust
pub struct ResponsePath {
    segments: Vec<String>,
}

impl ResponsePath {
    pub fn new(name: impl Into<String>) -> Self;
    pub fn empty() -> Self;
    pub fn child(&self, name: impl Into<String>) -> Self;
    pub fn segments(&self) -> &[String];
    pub fn as_str(&self) -> String;  // Dot-separated
}
```

### ResponseValue

```rust
pub enum ResponseValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    ChosenVariant(usize),        // For OneOf (single enum selection)
    ChosenVariants(Vec<usize>),  // For AnyOf (multi-select)
    StringList(Vec<String>),     // For Vec<String>
    IntList(Vec<i64>),           // For Vec<i32>, Vec<i64>, etc.
    FloatList(Vec<f64>),         // For Vec<f32>, Vec<f64>
}
```

### Responses

```rust
pub struct Responses {
    values: HashMap<ResponsePath, ResponseValue>,
}

impl Responses {
    pub fn get(&self, path: &ResponsePath) -> Option<&ResponseValue>;
    pub fn insert(&mut self, path: ResponsePath, value: ResponseValue);
    pub fn filter_prefix(&self, prefix: &ResponsePath) -> Self;
    pub fn has_value(&self, path: &ResponsePath) -> bool;

    // Typed accessors
    pub fn get_string(&self, path: &ResponsePath) -> Result<&str, ResponseError>;
    pub fn get_int(&self, path: &ResponsePath) -> Result<i64, ResponseError>;
    pub fn get_float(&self, path: &ResponsePath) -> Result<f64, ResponseError>;
    pub fn get_bool(&self, path: &ResponsePath) -> Result<bool, ResponseError>;
    pub fn get_chosen_variant(&self, path: &ResponsePath) -> Result<usize, ResponseError>;
    pub fn get_chosen_variants(&self, path: &ResponsePath) -> Result<&[usize], ResponseError>;
    pub fn get_string_list(&self, path: &ResponsePath) -> Result<&[String], ResponseError>;
    pub fn get_int_list(&self, path: &ResponsePath) -> Result<&[i64], ResponseError>;
    pub fn get_float_list(&self, path: &ResponsePath) -> Result<&[f64], ResponseError>;
}
```

### Question

```rust
pub struct Question {
    path: ResponsePath,
    ask: String,
    kind: QuestionKind,
    default: DefaultValue,
}

pub enum DefaultValue {
    None,
    Suggested(ResponseValue),  // Pre-filled, user can modify
    Assumed(ResponseValue),    // Question skipped, value used directly
}
```

### QuestionKind

```rust
pub enum QuestionKind {
    Unit,                       // No data (unit variants)
    Input(InputQuestion),       // Single-line text
    Multiline(MultilineQuestion),
    Masked(MaskedQuestion),     // Password input
    Int(IntQuestion),           // Integer with min/max
    Float(FloatQuestion),       // Float with min/max
    Confirm(ConfirmQuestion),   // Yes/no
    List(ListQuestion),         // Vec<primitive>
    AnyOf(AnyOfQuestion),       // Multi-select (Vec<Enum>)
    AllOf(AllOfQuestion),       // Group of questions (nested struct)
    OneOf(OneOfQuestion),       // Choose one variant (enum)
}
```

### Survey Trait

```rust
pub trait Survey: Sized {
    fn survey() -> SurveyDefinition;
    fn from_responses(responses: &Responses) -> Self;
    fn validate_field(
        value: &ResponseValue,
        responses: &Responses,
        path: &ResponsePath,
    ) -> Result<(), String>;
    fn validate_all(responses: &Responses) -> HashMap<ResponsePath, String>;
}
```

### SurveyBackend Trait

```rust
pub trait SurveyBackend {
    type Error: Into<anyhow::Error>;

    fn collect(
        &self,
        definition: &SurveyDefinition,
        validate: &dyn Fn(&ResponseValue, &Responses, &ResponsePath) -> Result<(), String>,
    ) -> Result<Responses, Self::Error>;
}
```

## Generated Code

The `#[derive(Survey)]` macro generates:

1. **`Survey` trait implementation** with `survey()`, `from_responses()`, `validate_field()`, `validate_all()`

2. **`TypeBuilder`** struct with:
   - `suggest_fieldname()` methods for each field
   - `assume_fieldname()` methods for each field
   - `with_suggestions(&instance)` to bulk populate from existing value
   - `run(backend)` to execute the survey

3. **`TypeValidationContext`** struct providing typed access to sibling fields during validation

4. **Static `get_fieldname()` methods** on the type for quick field access from validators

5. **Compile-time validator checks** ensuring validator functions have correct signatures

## Validation

### Field-level Validation

```rust
fn validate_email(
    value: &ResponseValue,
    responses: &Responses,
    path: &ResponsePath,
) -> Result<(), String> {
    let email = value.as_string().unwrap_or("");
    if email.contains('@') {
        Ok(())
    } else {
        Err("Invalid email address".to_string())
    }
}

#[derive(Survey)]
struct User {
    #[ask("Email:")]
    #[validate("validate_email")]
    email: String,
}
```

### Composite Validation

```rust
fn validate_passwords_match(responses: &Responses) -> HashMap<ResponsePath, String> {
    let mut errors = HashMap::new();
    let pw = responses.get_string(&ResponsePath::new("password"));
    let confirm = responses.get_string(&ResponsePath::new("password_confirm"));

    if let (Ok(pw), Ok(confirm)) = (pw, confirm) {
        if pw != confirm {
            errors.insert(
                ResponsePath::new("password_confirm"),
                "Passwords do not match".into(),
            );
        }
    }
    errors
}

#[derive(Survey)]
#[validate("validate_passwords_match")]
struct AccountSetup {
    #[ask("Password:")]
    #[mask]
    password: String,

    #[ask("Confirm password:")]
    #[mask]
    password_confirm: String,
}
```

### Propagated Validators

```rust
fn validate_positive(
    value: &ResponseValue,
    _responses: &Responses,
    _path: &ResponsePath,
) -> Result<(), String> {
    if let Some(n) = value.as_int() {
        if n > 0 { Ok(()) } else { Err("Must be positive".into()) }
    } else {
        Ok(())
    }
}

#[derive(Survey)]
#[validate_fields("validate_positive")]  // Applied to all numeric fields
struct Dimensions {
    #[ask("Width:")]
    width: i32,
    #[ask("Height:")]
    height: i32,
    #[ask("Depth:")]
    depth: i32,
}
```

## Suggestions and Assumptions

### Suggestions

Pre-fill values that users can modify:

```rust
let config: ServerConfig = ServerConfig::builder()
    .suggest_host("localhost")
    .suggest_port(8080)
    .run(backend)?;
```

### Assumptions

Skip questions entirely with pre-set values:

```rust
let config: ServerConfig = ServerConfig::builder()
    .assume_host("localhost")  // User won't be asked
    .run(backend)?;
```

### Bulk Suggestions

```rust
let existing = ServerConfig { host: "localhost".into(), port: 8080 };
let config: ServerConfig = ServerConfig::builder()
    .with_suggestions(&existing)
    .run(backend)?;
```

## Design Decisions and Scope

### Conditional Questions

Conditional logic ("only ask B if A is true") is handled through Rust's type system rather than a dedicated conditional question mechanism:

- **Enums with `OneOf`**: Model mutually exclusive paths naturally
- **Assumptions**: Skip questions programmatically based on prior knowledge
- **Nested surveys**: Group related questions that should appear together

This approach keeps the question model simple while leveraging Rust's expressiveness. Complex conditional flows are better modeled as enum variants than as runtime predicates on questions.

### Async Validators

Validators are synchronous by design. Async validation (e.g., checking username availability via API) is out of scope for the core crate because:

- It would complicate the `SurveyBackend` trait significantly
- Different backends have different async runtimes (tokio, async-std, blocking)
- Network validation during form fill creates UX challenges (latency, error handling)

**Workaround**: Perform async validation after survey completion, prompting the user to re-run if needed.

### Answer Persistence

Saving/loading `Responses` to disk for resumable surveys is out of scope. The `Responses` type is a simple `HashMap<ResponsePath, ResponseValue>` and can be serialized with serde if needed, but:

- Session management varies by application
- Storage backends vary (file, database, cloud)
- Versioning/migration is application-specific

Applications needing persistence should serialize `Responses` themselves.

### Internationalization (i18n)

Prompt strings are static `&str` values. Full i18n support is possible but not built-in:

**Approach 1**: Use a translation macro

```rust
#[derive(Survey)]
struct Greeting {
    #[ask(t!("greeting.name_prompt"))]  // Expands at compile time
    name: String,
}
```

**Approach 2**: Generate surveys programmatically

```rust
fn localized_survey(locale: &str) -> SurveyDefinition {
    let mut def = MyType::survey();
    for q in &mut def.questions {
        q.set_prompt(translate(q.path().as_str(), locale));
    }
    def
}
```

**Approach 3**: Use assumptions for known values and only prompt in the user's language for remaining fields.

The crate provides the hooks (`Question::set_prompt()`, mutable access to `SurveyDefinition`) but leaves i18n strategy to applications.

## Error Handling

| Error Category          | Handling                  | Visible to Caller? |
|-------------------------|---------------------------|--------------------|
| Validation              | Backend retry loop        | No                 |
| Response reconstruction | Ruled out by construction | No                 |
| Cancellation            | User exits early          | Yes                |
| Backend failure         | I/O, UI crash             | Yes                |

Backends return `Result<Responses, Self::Error>`. The builder converts this to `anyhow::Error` for convenience.

## Summary

Elicitor is presentation-agnostic. The derive macro generates a `SurveyDefinition` data structure. What consumers do with that structure is up to them:

- **Wizard backends** iterate through questions sequentially
- **Form backends** render all questions at once
- **Document crates** generate static documents

The crate naming convention communicates purpose:

- `elicitor-wizard-{library}` — sequential prompt backends
- `elicitor-form-{library}` — simultaneous form backends
- `elicitor-doc-{format}` — document generators
