pub mod interview;

pub mod suggested_answer;
pub use suggested_answer::SuggestedAnswer;

mod assumed_answer;
pub use assumed_answer::AssumedAnswer;

/// The key used to store the selected enum variant index in answers.
/// This constant ensures consistency between the macro-generated code and backends.
pub const SELECTED_ALTERNATIVE_KEY: &str = "selected_alternative";
