# Derive Wizard Architecture

## Overview

Derive Wizard is a Rust library that automatically generates interactive forms/wizards from Rust data structures using procedural macros. It transforms type definitions into interviews that can be executed across multiple backend UIs (CLI, TUI, GUI).

**Core Philosophy**: "Types as Schemas" - Your Rust struct/enum definitions serve as both the data model AND the form schema.

## System Architecture

### Three-Crate Design

The project is organized into three main crates with clear separation of concerns:

```
derive-wizard/
├── derive-wizard-types/     # Core data structures (no proc-macro deps)
├── derive-wizard-macro/     # Procedural macro implementation
└── derive-wizard/           # Main library + runtime + backends
```

**Rationale for separation**:
1. **derive-wizard-types**: Contains serializable data structures (`Interview`, `Question`, etc.) that can be used without macro dependencies. This allows for:
   - Programmatic interview construction
   - Interview serialization/inspection
   - Lighter dependency tree for consumers who only need types
   
2. **derive-wizard-macro**: Isolates the proc-macro implementation. Proc-macros must be in separate crates and have heavy compile-time dependencies (syn, quote). This separation keeps the macro logic focused.

3. **derive-wizard**: Integrates everything - re-exports the macro, provides the `Wizard` trait, implements backends, and offers runtime functionality.

### Compilation Flow

```
User Code with #[derive(Wizard)]
            ↓
derive-wizard-macro (compile-time)
            ↓
    Generated Rust code implementing:
    - Wizard::interview() -> Interview
    - Wizard::from_answers(Answers) -> Self
    - Wizard::validate_field(...) -> Result<(), String>
            ↓
    Runtime execution via Backend
            ↓
        User's struct/enum instance
```

## Core Abstractions

### 1. The `Wizard` Trait

```rust
pub trait Wizard: Sized {
    fn interview() -> Interview;
    fn interview_with_suggestions(&self) -> Interview;
    fn from_answers(answers: &Answers) -> Result<Self, BackendError>;
    fn validate_field(field: &str, value: &str, answers: &Answers) -> Result<(), String>;
}
```

This trait is the central abstraction. Every type that derives `Wizard` gets:
- **interview()**: Static method returning the form structure
- **interview_with_suggestions()**: Instance method that pre-fills the form with values from `self`
- **from_answers()**: Bidirectional transformation from answers back to typed data
- **validate_field()**: Centralized validation dispatch

### 2. Interview Data Model

The `Interview` structure represents a form:

```
Interview
├── sections: Vec<Question>      # Top-level questions
├── prelude: Option<String>      # Intro text
└── epilogue: Option<String>     # Outro text

Question
├── id: Option<String>           # Unique identifier
├── name: String                 # Field name (used as key in answers)
├── prompt: String               # User-facing question text
├── kind: QuestionKind           # What type of question
└── assumed: Option<AssumedAnswer>  # Pre-answer that skips the question

QuestionKind (enum)
├── Input(InputQuestion)         # String input
├── Multiline(MultilineQuestion) # Multi-line text
├── Masked(MaskedQuestion)       # Password-style input
├── Int(IntQuestion)             # Integer input with min/max
├── Float(FloatQuestion)         # Float input with min/max
├── Confirm(ConfirmQuestion)     # Yes/No boolean
├── MultiSelect(MultiSelectQuestion)  # Multiple choice
├── Sequence(Vec<Question>)      # Nested questions (for structs)
└── Alternative(usize, Vec<Question>)  # Variant choice (for enums)
```

**Design Decision: Sequence vs Alternative**
- `Sequence`: Represents nested struct fields or all questions in a section. ALL questions are asked.
- `Alternative`: Represents enum variants. Only ONE variant's questions are asked based on user selection.

### 3. Answer Storage

```rust
pub type Answers = HashMap<String, AnswerValue>;

pub enum AnswerValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    IntList(Vec<i64>),      // For multi-select
    Nested(HashMap<String, AnswerValue>),  // Not currently used; nesting is flat
}
```

Answers use **flat dot-notation keys** for nested fields:
```
"name" -> "Alice"
"address.street" -> "123 Main St"
"address.city" -> "Portland"
"payment.CreditCard.number" -> "4111..."
```

**Key naming conventions**:
- Struct fields: Direct field name or `parent.field` for nesting
- Enum variant selection: `SELECTED_ALTERNATIVE_KEY` (constant = "selected_alternative")
- Tuple fields: `field_0`, `field_1`, etc.

### 4. Backend Abstraction

```rust
pub trait InterviewBackend {
    fn execute(&self, interview: &Interview) -> Result<Answers, BackendError>;
    
    fn execute_with_validator(
        &self,
        interview: &Interview,
        validator: ValidatorFn<'_>,
    ) -> Result<Answers, BackendError>;
}
```

Backends are **pluggable UI implementations**. Each backend:
1. Takes an `Interview` structure
2. Presents it to the user in its own UI style
3. Collects answers
4. Returns an `Answers` hashmap

**Available Backends**:
- `RequesttyBackend` (default): Terminal prompts using requestty
- `DialoguerBackend`: Terminal prompts using dialoguer
- `RatatuiBackend`: Full TUI interface with ratatui
- `EguiBackend`: Native GUI windows with egui
- `TestBackend`: In-memory backend for testing

## Macro Code Generation Strategy

### Phase 1: Compile-Time Analysis

The macro (`derive-wizard-macro/src/lib.rs`) performs extensive compile-time checking:

1. **Type Classification**: Determines if a field is:
   - Builtin (String, i64, bool, PathBuf, etc.)
   - Nested Wizard (non-builtin with `#[prompt]`)
   - Vec\<Enum\> (multi-select question)

2. **Attribute Extraction**: Parses field attributes:
   - `#[prompt("text")]`: Question text (required for nested wizards)
   - `#[mask]`: Password-style input
   - `#[multiline]`: Multi-line text editor
   - `#[validate("func_name")]`: Custom validator function
   - `#[min(n)]` / `#[max(n)]`: Numeric bounds
   - `#[prelude("text")]` / `#[epilogue("text")]`: Interview-level messages

3. **Compile-Time Validation**:
   - Missing `#[prompt]` on non-builtin types → compile error
   - Invalid validator function signature → compile error via const assertion
   - Ensures users don't accidentally create runtime failures

### Phase 2: Code Generation

The macro generates three key implementations:

#### A. `interview()` Method

This is **hybrid compile-time/runtime generation**:

**For simple fields** (builtins): Questions are fully constructed at compile time:
```rust
Question::new(
    Some("age".to_string()),
    "age".to_string(),
    "Enter age:".to_string(),
    QuestionKind::Int(IntQuestion { default: None, min: None, max: None, validate: None })
)
```

**For nested Wizard fields**: Placeholder sequences are expanded at runtime:
```rust
{
    let mut nested_interview = <Address as Wizard>::interview();
    // Prefix all nested question names with "address."
    for question in &mut nested_interview.sections {
        let old_name = question.name().to_string();
        let new_name = format!("address.{}", old_name);
        // Update question with prefixed name...
    }
    nested_interview.sections
}
```

**Why hybrid?** We can't know nested Wizard structures at compile time (they might be in other crates), so we defer to runtime `Wizard::interview()` calls and apply name prefixing.

**For Vec\<Enum\> fields** (multi-select):
```rust
{
    let inner_interview = <PaymentMethod as Wizard>::interview();
    // Extract variant names from enum's Alternative questions
    let options: Vec<String> = /* extract variant names */;
    QuestionKind::MultiSelect(MultiSelectQuestion { options, defaults: vec![] })
}
```

#### B. `from_answers()` Method

Generates deserialization code that mirrors the type structure:

**For structs**:
```rust
Ok(Person {
    name: answers.as_string("name")?,
    age: answers.as_int("age")? as i32,
    address: {
        let mut nested_answers = Answers::default();
        for (key, value) in answers.iter() {
            if let Some(stripped) = key.strip_prefix("address.") {
                nested_answers.insert(stripped.to_string(), value.clone());
            }
        }
        <Address as Wizard>::from_answers(&nested_answers)?
    },
})
```

**For enums**:
```rust
let selected = answers.as_int(SELECTED_ALTERNATIVE_KEY)?;
match selected {
    0 => Ok(Transport::Car { /* fields */ }),
    1 => Ok(Transport::Bike),
    _ => Err(BackendError::ExecutionError(format!("Unknown variant: {}", selected)))
}
```

**For Vec\<Enum\>** (multi-select results):
```rust
{
    let indices = answers.as_int_list("payment_methods")?;
    indices.into_iter()
        .map(|idx| {
            let mut temp_answers = Answers::default();
            temp_answers.insert(SELECTED_ALTERNATIVE_KEY.to_string(), AnswerValue::Int(idx));
            <PaymentMethod as Wizard>::from_answers(&temp_answers)
        })
        .collect::<Result<Vec<_>, _>>()?
}
```

#### C. `validate_field()` Method

Generates a dispatcher that routes field validation to user-defined functions:

```rust
fn validate_field(field: &str, value: &str, answers: &Answers) -> Result<(), String> {
    // Check for nested wizard delegation
    if let Some(nested_field) = field.strip_prefix("address.") {
        return <Address as Wizard>::validate_field(nested_field, value, answers);
    }
    
    // Direct field validators
    match field {
        "email" => validate_email(value, answers),
        "age" => validate_adult(value, answers),
        _ => Ok(()),
    }
}
```

Validators have signature: `fn(&str, &Answers) -> Result<(), String>`
- First param: The input value as string
- Second param: All answers collected so far (for cross-field validation)
- Return: `Ok(())` or `Err(message)`

## Advanced Features

### 1. Nested Structures

Nesting is handled via **name prefixing** and **recursive trait calls**:

```rust
#[derive(Wizard)]
struct Person {
    name: String,
    #[prompt("Enter address details:")]
    address: Address,  // Address must also derive Wizard
}

#[derive(Wizard)]
struct Address {
    street: String,
    city: String,
}
```

At runtime, questions become:
- `"name"` → "Enter name:"
- `"address.street"` → "Enter street:"
- `"address.city"` → "Enter city:"

The recursive call to `Address::interview()` happens in `Person::interview()`, and all nested question names get prefixed.

### 2. Enum Support

Enums are modeled as **Alternative questions**:

```rust
#[derive(Wizard)]
enum Transport {
    Car { make: String, model: String },
    Bike,
    Walk,
}
```

Generates:
1. A top-level Sequence question named "alternatives"
2. Each variant becomes an Alternative with its own questions
3. User selects variant first, then only that variant's fields are asked
4. The selected index is stored under `SELECTED_ALTERNATIVE_KEY`

**Nested enums** (enum fields in structs):
```rust
#[derive(Wizard)]
struct Trip {
    destination: String,
    #[prompt("How will you travel?")]
    transport: Transport,
}
```

The `Transport` enum's alternatives get prefixed with `transport.alternatives`, and the selection is stored as `transport.selected_alternative`.

### 3. Multi-Select (Vec\<Enum\>)

```rust
#[derive(Wizard)]
enum Feature { GPS, Bluetooth, Camera }

#[derive(Wizard)]
struct Phone {
    #[prompt("Select features:")]
    features: Vec<Feature>,
}
```

This generates:
- A `MultiSelect` question with options extracted from `Feature`'s variants
- User can select multiple indices: `[0, 2]` → GPS and Camera
- `from_answers()` constructs `Vec<Feature>` by calling `Feature::from_answers()` for each selected index

### 4. Builder Pattern API

The `WizardBuilder` provides a fluent API for customization:

```rust
Person::wizard_builder()
    .suggest_field("name", "Alice")           // Pre-fill but still ask
    .assume_field("country", "USA")           // Skip question entirely
    .with_backend(EguiBackend::new())
    .build()?
```

**Suggestions vs Assumptions**:
- **Suggestion**: Pre-fills the default value, question is still shown
- **Assumption**: Skips the question entirely, uses the assumed value

The builder uses `find_question_by_path()` to locate nested questions and apply suggestions/assumptions.

### 5. Validation System

**Two levels of validation**:

1. **Built-in validation** (min/max for numbers):
   ```rust
   #[derive(Wizard)]
   struct Config {
       #[min(1)]
       #[max(100)]
       port: i32,
   }
   ```

2. **Custom validators**:
   ```rust
   fn validate_email(value: &str, _answers: &Answers) -> Result<(), String> {
       if value.contains('@') {
           Ok(())
       } else {
           Err("Invalid email".to_string())
       }
   }
   
   #[derive(Wizard)]
   struct User {
       #[validate("validate_email")]
       email: String,
   }
   ```

Validators are:
- Checked at compile time via const assertions
- Called by backends during question prompting
- Can access all previous answers for cross-field validation

### 6. Typst Form Generation

With the `typst-form` feature:

```rust
let typst_markup = Person::to_typst_form(Some("Registration Form"));
std::fs::write("form.typ", typst_markup)?;
```

Generates a printable PDF form (non-interactive) via Typst markup language. This demonstrates the flexibility of the `Interview` data structure - it can target multiple output formats.

## Key Design Patterns

### 1. Procedural Macro as Code Generator

The macro doesn't implement runtime behavior; it **generates code** that uses the runtime library. This keeps the macro focused on AST transformation.

### 2. Type-Driven Design

Field types determine question kinds:
- `String` → Input
- `bool` → Confirm
- `i32` → Int
- Custom type → Sequence (nested)
- `Vec<Enum>` → MultiSelect

Attributes override/refine this mapping (`#[mask]`, `#[multiline]`).

### 3. Flat Answer Storage with Hierarchical Names

Rather than nested hashmaps (`Nested(HashMap)`), we use flat storage with dot-separated keys. This simplifies:
- Serialization
- Debugging (can print all answers as flat key-value pairs)
- Backend implementation (no recursive structure to handle)

### 4. Backend Polymorphism

The `InterviewBackend` trait allows runtime backend selection without recompiling the wizard definition. Same `Interview` can render in CLI, TUI, or GUI.

### 5. Compile-Time Safety

Heavy use of compile-time checks:
- Type system ensures only valid field types
- `compile_error!` for missing attributes
- Const assertions for validator signatures
- No silent failures at runtime

## Questions & Areas for Expansion

### Questions:

1. **Conditional Questions**: Do you plan to support conditional logic (e.g., "only ask question B if answer A is 'yes'")? Currently, assumptions can skip questions, but there's no built-in "if-then" flow.

2. **Async Validators**: Validators are currently synchronous. Are there plans for async validation (e.g., checking if a username is available via API)?

3. **Custom Question Types**: Is there a plugin system planned for custom question types beyond the built-in ones? E.g., date pickers, file uploads, etc.

4. **Answer Persistence**: Is there a plan for saving/loading `Answers` to disk (e.g., for interview sessions that can be resumed)?

5. **Error Recovery**: If `from_answers()` fails midway through (validation error, type conversion), is there a recovery mechanism or does the entire interview need to restart?

6. **Localization (i18n)**: How would you approach multi-language support for prompts and error messages?

### Areas That Could Use More Detail:

1. **Field Path Resolution Algorithm**: How does `find_question_by_path()` handle ambiguous cases? E.g., if you have both `"user.name"` as a single question name AND a nested structure `user { name }`, which takes precedence?

2. **Backend Implementation Guidelines**: What's the contract for implementing a new backend? Are there common pitfalls? Is there a test suite that backends should pass?

3. **Performance Characteristics**: For deeply nested structures or very large enums, what are the performance implications of the runtime interview expansion?

4. **Derive Macro Expansion Order**: With nested Wizards, what's the compilation order guarantee? Can you have circular dependencies?

5. **Migration/Versioning**: If a struct's fields change (add/remove/rename), how do you handle old serialized `Answers`? Is there a versioning story?

6. **Alternative Question Rendering**: The `Alternative` stores a `usize` index. How do backends present this to users? Is it always "Select variant: [Car, Bike, Walk]" or can it be customized?

## Current Architectural Strengths

1. **Separation of Concerns**: Clean split between data model (types), schema (macro), and presentation (backends)
2. **Extensibility**: New backends don't require changes to core library
3. **Type Safety**: Leverages Rust's type system for compile-time guarantees
4. **Minimal Runtime Dependencies**: The types crate is dependency-light
5. **Dogfooding**: The library uses standard Rust patterns (traits, derives, builders)

## Current Architectural Limitations

1. **No Dynamic Schemas**: Interviews are derived from static types; can't easily create dynamic forms from runtime data
2. **Limited Control Flow**: No built-in support for conditional questions, branching, or loops
3. **Flat Validation**: Validators can't easily return structured errors (e.g., multiple errors per field)
4. **Answer Type Coupling**: `AnswerValue` is tightly coupled to supported question types; extending requires modifying the enum
5. **Backend Feature Flags**: Having multiple backend features creates a large feature matrix; might want a backend registry pattern instead

Would you like me to expand on any particular section or explore specific architectural decisions in more depth?
