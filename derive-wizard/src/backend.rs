use crate::interview::{Interview, Section};
use std::collections::HashMap;

/// Represents the answers collected from an interview
#[derive(Debug, Clone, Default)]
pub struct Answers {
    values: HashMap<String, AnswerValue>,
}

/// A single answer value
#[derive(Debug, Clone)]
pub enum AnswerValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Nested(Box<Answers>),
}

impl Answers {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: AnswerValue) {
        self.values.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&AnswerValue> {
        self.values.get(key)
    }

    pub fn merge(&mut self, other: Answers) {
        self.values.extend(other.values);
    }

    pub fn as_string(&self, key: &str) -> Result<String, AnswerError> {
        match self.get(key) {
            Some(AnswerValue::String(s)) => Ok(s.clone()),
            Some(_) => Err(AnswerError::TypeMismatch {
                key: key.to_string(),
                expected: "String",
            }),
            None => Err(AnswerError::MissingKey(key.to_string())),
        }
    }

    pub fn as_int(&self, key: &str) -> Result<i64, AnswerError> {
        match self.get(key) {
            Some(AnswerValue::Int(i)) => Ok(*i),
            Some(_) => Err(AnswerError::TypeMismatch {
                key: key.to_string(),
                expected: "Int",
            }),
            None => Err(AnswerError::MissingKey(key.to_string())),
        }
    }

    pub fn as_float(&self, key: &str) -> Result<f64, AnswerError> {
        match self.get(key) {
            Some(AnswerValue::Float(f)) => Ok(*f),
            Some(_) => Err(AnswerError::TypeMismatch {
                key: key.to_string(),
                expected: "Float",
            }),
            None => Err(AnswerError::MissingKey(key.to_string())),
        }
    }

    pub fn as_bool(&self, key: &str) -> Result<bool, AnswerError> {
        match self.get(key) {
            Some(AnswerValue::Bool(b)) => Ok(*b),
            Some(_) => Err(AnswerError::TypeMismatch {
                key: key.to_string(),
                expected: "Bool",
            }),
            None => Err(AnswerError::MissingKey(key.to_string())),
        }
    }

    pub fn as_nested(&self, key: &str) -> Result<&Answers, AnswerError> {
        match self.get(key) {
            Some(AnswerValue::Nested(nested)) => Ok(nested),
            Some(_) => Err(AnswerError::TypeMismatch {
                key: key.to_string(),
                expected: "Nested",
            }),
            None => Err(AnswerError::MissingKey(key.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AnswerError {
    #[error("Missing key: {0}")]
    MissingKey(String),

    #[error("Type mismatch for key '{key}': expected {expected}")]
    TypeMismatch { key: String, expected: &'static str },
}

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("Answer error: {0}")]
    Answer(#[from] AnswerError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Backend-specific error: {0}")]
    Custom(String),
}

/// Trait for interview execution backends
pub trait InterviewBackend {
    /// Execute an interview and return the collected answers
    fn execute(&self, interview: &Interview) -> Result<Answers, BackendError>;

    /// Execute a single section (optional, has default implementation)
    fn execute_section(&self, section: &Section) -> Result<Answers, BackendError> {
        // Default implementation - subclasses can override
        let _ = section;
        Err(BackendError::Custom(
            "execute_section not implemented".to_string(),
        ))
    }
}

/// Test backend that returns predefined answers
#[derive(Debug, Default)]
pub struct TestBackend {
    answers: Answers,
}

impl TestBackend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_string(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.answers
            .insert(key.into(), AnswerValue::String(value.into()));
        self
    }

    pub fn with_int(mut self, key: impl Into<String>, value: i64) -> Self {
        self.answers.insert(key.into(), AnswerValue::Int(value));
        self
    }

    pub fn with_float(mut self, key: impl Into<String>, value: f64) -> Self {
        self.answers.insert(key.into(), AnswerValue::Float(value));
        self
    }

    pub fn with_bool(mut self, key: impl Into<String>, value: bool) -> Self {
        self.answers.insert(key.into(), AnswerValue::Bool(value));
        self
    }
}

impl InterviewBackend for TestBackend {
    fn execute(&self, _interview: &Interview) -> Result<Answers, BackendError> {
        // Simply return the predefined answers
        Ok(self.answers.clone())
    }
}
