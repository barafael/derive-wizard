#[derive(Debug, Clone)]
pub struct Question {
    /// The unique identifier for the question.
    id: Option<String>,

    /// The field name.
    name: String,

    /// The prompt message to display.
    prompt: String,

    kind: QuestionKind,
}

impl Question {
    /// Create a new question with the given id, name, prompt, and kind.
    pub fn new(id: Option<String>, name: String, prompt: String, kind: QuestionKind) -> Self {
        Self {
            id,
            name,
            prompt,
            kind,
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    pub fn kind(&self) -> &QuestionKind {
        &self.kind
    }

    pub fn kind_mut(&mut self) -> &mut QuestionKind {
        &mut self.kind
    }

    /// Set the default value for this question based on its kind.
    /// Returns true if the default was set, false if the question kind doesn't support defaults.
    pub fn set_default(&mut self, value: impl Into<QuestionDefault>) -> bool {
        match (&mut self.kind, value.into()) {
            (QuestionKind::Input(q), QuestionDefault::String(v)) => {
                q.default = Some(v);
                true
            }
            (QuestionKind::Multiline(q), QuestionDefault::String(v)) => {
                q.default = Some(v);
                true
            }
            (QuestionKind::Int(q), QuestionDefault::Int(v)) => {
                q.default = Some(v);
                true
            }
            (QuestionKind::Float(q), QuestionDefault::Float(v)) => {
                q.default = Some(v);
                true
            }
            (QuestionKind::Confirm(q), QuestionDefault::Bool(v)) => {
                q.default = v;
                true
            }
            _ => false,
        }
    }
}

/// Represents a default value that can be set on a question.
pub enum QuestionDefault {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl From<String> for QuestionDefault {
    fn from(v: String) -> Self {
        QuestionDefault::String(v)
    }
}

impl From<i64> for QuestionDefault {
    fn from(v: i64) -> Self {
        QuestionDefault::Int(v)
    }
}

impl From<f64> for QuestionDefault {
    fn from(v: f64) -> Self {
        QuestionDefault::Float(v)
    }
}

impl From<bool> for QuestionDefault {
    fn from(v: bool) -> Self {
        QuestionDefault::Bool(v)
    }
}

/// Possible question kinds which a wizard may ask.
#[derive(Debug, Clone)]
pub enum QuestionKind {
    /// A text input question for string values.
    Input(InputQuestion),

    /// A multi-line text input.
    Multiline(MultilineQuestion),

    /// A password/masked input question.
    Masked(MaskedQuestion),

    /// A number input question (integers).
    Int(IntQuestion),

    /// A number input question (floating point).
    Float(FloatQuestion),

    /// A yes/no confirmation question.
    Confirm(ConfirmQuestion),

    /// A nested interview (for composite types that implement Wizard).
    Nested(NestedQuestion),
}

/// Configuration for a nested interview question.
#[derive(Debug, Clone)]
pub struct NestedQuestion {
    /// The type path of the nested wizard type.
    pub type_path: String,
}

/// Configuration for a text input question.
#[derive(Debug, Clone)]
pub struct InputQuestion {
    /// Optional default value.
    pub default: Option<String>,

    /// Validation function to call on each keystroke.
    pub validate_on_key: Option<String>,

    /// Validation function to call on submission.
    pub validate_on_submit: Option<String>,
}

/// Configuration for a multi-line text editor question.
#[derive(Debug, Clone)]
pub struct MultilineQuestion {
    /// Optional default value.
    pub default: Option<String>,

    /// Validation function to call on each keystroke.
    pub validate_on_key: Option<String>,

    /// Validation function to call on submission.
    pub validate_on_submit: Option<String>,
}

/// Configuration for a password/masked input question.
#[derive(Debug, Clone)]
pub struct MaskedQuestion {
    /// The masking character (default: '*').
    pub mask: Option<char>,

    /// Validation function to call on each keystroke.
    pub validate_on_key: Option<String>,

    /// Validation function to call on submission.
    pub validate_on_submit: Option<String>,
}

/// Configuration for an integer input question.
#[derive(Debug, Clone)]
pub struct IntQuestion {
    /// Optional default value
    pub default: Option<i64>,

    /// Optional minimum value
    pub min: Option<i64>,

    /// Optional maximum value
    pub max: Option<i64>,

    /// Validation function to call on each keystroke.
    pub validate_on_key: Option<String>,

    /// Validation function to call on submission.
    pub validate_on_submit: Option<String>,
}

/// Configuration for a floating-point input question.
#[derive(Debug, Clone)]
pub struct FloatQuestion {
    /// Optional default value.
    pub default: Option<f64>,

    /// Optional minimum value
    pub min: Option<f64>,

    /// Optional maximum value
    pub max: Option<f64>,

    /// Validation function to call on each keystroke.
    pub validate_on_key: Option<String>,

    /// Validation function to call on submission.
    pub validate_on_submit: Option<String>,
}

/// Configuration for a yes/no confirmation question.
#[derive(Debug, Clone)]
pub struct ConfirmQuestion {
    /// Default value (true for yes, false for no)
    pub default: bool,
}
