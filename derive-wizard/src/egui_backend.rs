#[cfg(feature = "egui-backend")]
use crate::backend::{AnswerValue, Answers, BackendError, InterviewBackend};
use crate::interview::{Interview, Section, Sequence};
use crate::question::{Question, QuestionKind};

/// State for the interview
#[derive(Debug, Clone)]
struct InterviewState {
    input_buffers: std::collections::HashMap<String, String>,
    selected_alternatives: std::collections::HashMap<String, usize>,
    validation_errors: std::collections::HashMap<String, String>,
}

impl InterviewState {
    fn new() -> Self {
        Self {
            input_buffers: std::collections::HashMap::new(),
            selected_alternatives: std::collections::HashMap::new(),
            validation_errors: std::collections::HashMap::new(),
        }
    }

    fn get_or_init_buffer(&mut self, key: &str) -> &mut String {
        self.input_buffers
            .entry(key.to_string())
            .or_insert_with(String::new)
    }
}

/// egui-based interview backend
pub struct EguiBackend {
    interview: Interview,
    state: InterviewState,
    completed: bool,
    result: Option<Result<Answers, BackendError>>,
}

impl EguiBackend {
    pub fn new(interview: Interview) -> Self {
        Self {
            interview,
            state: InterviewState::new(),
            completed: false,
            result: None,
        }
    }

    /// Show the interview UI and return true if completed
    pub fn show(&mut self, ctx: &egui::Context) -> bool {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Interview Wizard");
            ui.separator();

            if self.completed {
                ui.label("Interview completed!");
                return;
            }

            // Show all sections in a scrollable area
            egui::ScrollArea::vertical().show(ui, |ui| {
                let sections: Vec<_> = self.interview.sections.clone();
                for (section_idx, section) in sections.iter().enumerate() {
                    self.show_section(ui, section, section_idx);
                    ui.add_space(15.0);
                }

                ui.separator();
                ui.add_space(10.0);

                // Submit button at the bottom
                if ui.button("Submit").clicked() {
                    if self.validate_and_save_all() {
                        self.completed = true;
                    }
                }

                // Show validation errors
                if !self.state.validation_errors.is_empty() {
                    ui.add_space(10.0);
                    ui.colored_label(egui::Color32::RED, "Please fix the following errors:");
                    for (field, error) in &self.state.validation_errors {
                        ui.colored_label(egui::Color32::RED, format!("  • {}: {}", field, error));
                    }
                }
            });
        });

        self.completed
    }

    fn show_section(&mut self, ui: &mut egui::Ui, section: &Section, section_idx: usize) {
        match section {
            Section::Empty => {
                // Nothing to render
            }
            Section::Sequence(seq) => {
                self.show_sequence(ui, seq);
            }
            Section::Alternatives(default_idx, alternatives) => {
                self.show_alternatives(ui, *default_idx, alternatives, section_idx);
            }
        }
    }

    fn show_sequence(&mut self, ui: &mut egui::Ui, sequence: &Sequence) {
        for question in &sequence.sequence {
            self.show_question(ui, question);
            ui.add_space(8.0);
        }
    }

    fn show_alternatives(
        &mut self,
        ui: &mut egui::Ui,
        default_idx: usize,
        alternatives: &[crate::interview::Alternative],
        section_idx: usize,
    ) {
        let alt_key = format!("section_{}", section_idx);

        ui.label("Select an option:");
        ui.add_space(5.0);

        let selected = self
            .state
            .selected_alternatives
            .get(&alt_key)
            .copied()
            .unwrap_or(default_idx);

        for (idx, alternative) in alternatives.iter().enumerate() {
            if ui.radio(selected == idx, &alternative.name).clicked() {
                self.state
                    .selected_alternatives
                    .insert(alt_key.clone(), idx);
            }
        }

        ui.add_space(10.0);

        // Show follow-up questions for selected alternative (only if not empty)
        if let Some(alt) = alternatives.get(selected) {
            if !matches!(alt.section, Section::Empty) {
                ui.group(|ui| {
                    ui.label(format!("Details for '{}':", alt.name));
                    ui.add_space(5.0);
                    self.show_section(ui, &alt.section, section_idx * 1000 + selected);
                });
            }
        }
    }

    fn show_question(&mut self, ui: &mut egui::Ui, question: &Question) {
        let id = question.id().unwrap_or(question.name());

        ui.horizontal(|ui| {
            ui.label(question.prompt());

            // Show error indicator if there's a validation error
            if self.state.validation_errors.contains_key(id) {
                ui.colored_label(egui::Color32::RED, "⚠");
            }
        });
        ui.add_space(3.0);

        match question.kind() {
            QuestionKind::Input(input_q) => {
                let buffer = self.state.get_or_init_buffer(id);

                let mut text_edit = egui::TextEdit::singleline(buffer);

                // Use default as placeholder text if available
                if let Some(default) = &input_q.default {
                    text_edit = text_edit.hint_text(default);
                }

                ui.add(text_edit);
            }
            QuestionKind::Multiline(multiline_q) => {
                let buffer = self.state.get_or_init_buffer(id);

                let mut text_edit = egui::TextEdit::multiline(buffer);

                // Use default as placeholder text if available
                if let Some(default) = &multiline_q.default {
                    text_edit = text_edit.hint_text(default);
                }

                ui.add(text_edit);
            }
            QuestionKind::Masked(_masked_q) => {
                let buffer = self.state.get_or_init_buffer(id);
                ui.add(egui::TextEdit::singleline(buffer).password(true));
            }
            QuestionKind::Int(int_q) => {
                let buffer = self.state.get_or_init_buffer(id);

                // Parse current value or use default
                let mut value = if buffer.is_empty() {
                    int_q.default.unwrap_or(0)
                } else {
                    buffer.parse::<i64>().unwrap_or(0)
                };

                let mut drag = egui::DragValue::new(&mut value).speed(1.0);

                if let Some(min) = int_q.min {
                    drag = drag.range(min..=int_q.max.unwrap_or(i64::MAX));
                } else if let Some(max) = int_q.max {
                    drag = drag.range(i64::MIN..=max);
                }

                if ui.add(drag).changed() {
                    *buffer = value.to_string();
                }
            }
            QuestionKind::Float(float_q) => {
                let buffer = self.state.get_or_init_buffer(id);

                // Parse current value or use default
                let mut value = if buffer.is_empty() {
                    float_q.default.unwrap_or(0.0)
                } else {
                    buffer.parse::<f64>().unwrap_or(0.0)
                };

                let mut drag = egui::DragValue::new(&mut value).speed(0.1);

                if let Some(min) = float_q.min {
                    drag = drag.range(min..=float_q.max.unwrap_or(f64::MAX));
                } else if let Some(max) = float_q.max {
                    drag = drag.range(f64::MIN..=max);
                }

                if ui.add(drag).changed() {
                    *buffer = value.to_string();
                }
            }
            QuestionKind::Confirm(confirm_q) => {
                let buffer = self.state.get_or_init_buffer(id);

                if buffer.is_empty() {
                    *buffer = confirm_q.default.to_string();
                }

                let mut value = buffer == "true";
                ui.checkbox(&mut value, "Yes");
                *buffer = value.to_string();
            }
            QuestionKind::Nested(_) => {
                ui.colored_label(
                    egui::Color32::RED,
                    "Error: Nested questions should be inlined",
                );
            }
        }
    }

    fn validate_and_save_all(&mut self) -> bool {
        self.state.validation_errors.clear();
        let mut answers = Answers::new();
        let mut all_valid = true;

        // Validate and save all questions from all sections
        let sections = self.interview.sections.clone();
        for (section_idx, section) in sections.iter().enumerate() {
            if !self.validate_section(section, section_idx, &mut answers) {
                all_valid = false;
            }
        }

        if all_valid {
            self.result = Some(Ok(answers));
        }

        all_valid
    }

    fn validate_section(
        &mut self,
        section: &Section,
        section_idx: usize,
        answers: &mut Answers,
    ) -> bool {
        match section {
            Section::Empty => true,
            Section::Sequence(seq) => {
                let mut valid = true;
                for question in &seq.sequence {
                    if !self.validate_and_save_question(question, answers) {
                        valid = false;
                    }
                }
                valid
            }
            Section::Alternatives(default_idx, alternatives) => {
                let alt_key = format!("section_{}", section_idx);
                let selected = self
                    .state
                    .selected_alternatives
                    .get(&alt_key)
                    .copied()
                    .unwrap_or(*default_idx);

                // Store the selected alternative name
                if let Some(alt) = alternatives.get(selected) {
                    answers.insert(
                        "selected_alternative".to_string(),
                        AnswerValue::String(alt.name.clone()),
                    );

                    // Validate the follow-up section
                    self.validate_section(&alt.section, section_idx * 1000 + selected, answers)
                } else {
                    true
                }
            }
        }
    }

    fn validate_and_save_question(&mut self, question: &Question, answers: &mut Answers) -> bool {
        let id = question.id().unwrap_or(question.name());
        let buffer = self.state.get_or_init_buffer(id).clone();

        match question.kind() {
            QuestionKind::Input(input_q) => {
                // Use default if buffer is empty
                let value = if buffer.is_empty() {
                    input_q.default.clone().unwrap_or_default()
                } else {
                    buffer
                };
                answers.insert(id.to_string(), AnswerValue::String(value));
                true
            }
            QuestionKind::Multiline(multiline_q) => {
                // Use default if buffer is empty
                let value = if buffer.is_empty() {
                    multiline_q.default.clone().unwrap_or_default()
                } else {
                    buffer
                };
                answers.insert(id.to_string(), AnswerValue::String(value));
                true
            }
            QuestionKind::Masked(_) => {
                answers.insert(id.to_string(), AnswerValue::String(buffer));
                true
            }
            QuestionKind::Int(int_q) => {
                // Use default if buffer is empty
                if buffer.is_empty() {
                    let val = int_q.default.unwrap_or(0);
                    answers.insert(id.to_string(), AnswerValue::Int(val));
                    self.state.validation_errors.remove(id);
                    true
                } else {
                    match buffer.parse::<i64>() {
                        Ok(val) => {
                            answers.insert(id.to_string(), AnswerValue::Int(val));
                            self.state.validation_errors.remove(id);
                            true
                        }
                        Err(_) => {
                            self.state
                                .validation_errors
                                .insert(id.to_string(), "Please enter a valid integer".to_string());
                            false
                        }
                    }
                }
            }
            QuestionKind::Float(float_q) => {
                // Use default if buffer is empty
                if buffer.is_empty() {
                    let val = float_q.default.unwrap_or(0.0);
                    answers.insert(id.to_string(), AnswerValue::Float(val));
                    self.state.validation_errors.remove(id);
                    true
                } else {
                    match buffer.parse::<f64>() {
                        Ok(val) => {
                            answers.insert(id.to_string(), AnswerValue::Float(val));
                            self.state.validation_errors.remove(id);
                            true
                        }
                        Err(_) => {
                            self.state.validation_errors.insert(
                                id.to_string(),
                                "Please enter a valid decimal number".to_string(),
                            );
                            false
                        }
                    }
                }
            }
            QuestionKind::Confirm(_) => {
                let val = buffer == "true";
                answers.insert(id.to_string(), AnswerValue::Bool(val));
                true
            }
            QuestionKind::Nested(_) => false,
        }
    }

    /// Get the result if the interview is completed
    pub fn result(&self) -> Option<&Result<Answers, BackendError>> {
        self.result.as_ref()
    }
}

impl InterviewBackend for EguiBackend {
    fn execute(&self, _interview: &Interview) -> Result<Answers, BackendError> {
        // For egui, we can't block and execute synchronously
        // This should be called after the UI completes
        Err(BackendError::Custom(
            "EguiBackend::execute should not be called directly. Use show() method instead."
                .to_string(),
        ))
    }
}
