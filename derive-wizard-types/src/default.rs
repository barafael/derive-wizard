/// Represents a suggested value that can be set on a question.
pub enum SuggestedAnswer {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl From<String> for SuggestedAnswer {
    fn from(v: String) -> Self {
        SuggestedAnswer::String(v)
    }
}

impl From<i64> for SuggestedAnswer {
    fn from(v: i64) -> Self {
        SuggestedAnswer::Int(v)
    }
}

impl From<f64> for SuggestedAnswer {
    fn from(v: f64) -> Self {
        SuggestedAnswer::Float(v)
    }
}

impl From<bool> for SuggestedAnswer {
    fn from(v: bool) -> Self {
        SuggestedAnswer::Bool(v)
    }
}

/// Represents an assumed value that skips the question entirely.
#[derive(Debug, Clone)]
pub enum AssumedAnswer {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl From<String> for AssumedAnswer {
    fn from(v: String) -> Self {
        AssumedAnswer::String(v)
    }
}

impl From<i64> for AssumedAnswer {
    fn from(v: i64) -> Self {
        AssumedAnswer::Int(v)
    }
}

impl From<f64> for AssumedAnswer {
    fn from(v: f64) -> Self {
        AssumedAnswer::Float(v)
    }
}

impl From<bool> for AssumedAnswer {
    fn from(v: bool) -> Self {
        AssumedAnswer::Bool(v)
    }
}
