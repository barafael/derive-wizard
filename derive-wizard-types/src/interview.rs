use crate::question::Question;

/// A sequence of sections, which contain questions.
#[derive(Debug, Clone)]
pub struct Interview {
    pub sections: Vec<Section>,
}

/// A section may be either questions in sequence or a single-choice selection question.
#[derive(Debug, Clone)]
pub enum Section {
    Empty,

    /// A sequence of questions.
    Sequence(Sequence),

    /// A single-choice selection from a list.
    Alternatives(usize, Vec<Alternative>),
}

/// A sequence of questions.
#[derive(Debug, Clone)]
pub struct Sequence {
    /// The list of questions to ask in sequence.
    pub sequence: Vec<Question>,
}

/// Single-choice alternatives.
#[derive(Debug, Clone)]
pub struct Alternative {
    /// The name of the alternative.
    pub name: String,

    /// The question to ask.
    pub section: Section,
}
