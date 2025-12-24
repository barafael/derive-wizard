#![doc = include_str!("../../README.md")]

pub use derive_wizard_macro::*;
pub use requestty::{Answer, Answers, ExpandItem, ListItem, Question, prompt_one};

pub trait Wizard: Sized {
    fn wizard() -> Self;

    fn wizard_with_message(message: &str) -> Self {
        let _ = message;
        Self::wizard()
    }

    fn wizard_with_defaults(self) -> Self {
        self
    }
}

/// Simplified trait that defines the essential prompting operations needed by the Wizard macro.
///
/// This trait abstracts over different CLI prompting libraries (requestty, dialoguer, etc.)
/// by focusing on the actual operations rather than specific types.
pub trait WizardPromptingBackend {
    /// Prompt for a string input
    fn prompt_string(
        message: &str,
        default: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>>;

    /// Prompt for a boolean confirmation
    fn prompt_bool(
        message: &str,
        default: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    /// Prompt for an integer
    fn prompt_int(message: &str, default: Option<i64>) -> Result<i64, Box<dyn std::error::Error>>;

    /// Prompt for a float
    fn prompt_float(message: &str, default: Option<f64>)
    -> Result<f64, Box<dyn std::error::Error>>;

    /// Prompt for a password (masked input)
    fn prompt_password(message: &str) -> Result<String, Box<dyn std::error::Error>>;

    /// Prompt using an editor
    fn prompt_editor(
        message: &str,
        default: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

/// Implementation of the wizard prompting backend using requestty.
///
/// This struct provides the concrete implementation for interactive
/// CLI prompts using the requestty crate.
pub struct RequesttyWizard;

impl WizardPromptingBackend for RequesttyWizard {
    fn prompt_string(
        message: &str,
        default: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut question = Question::input("input").message(message);
        if let Some(d) = default {
            question = question.default(d);
        }
        let answer = prompt_one(question.build())?;
        answer
            .try_into_string()
            .map_err(|_| "Failed to convert answer to string".into())
    }

    fn prompt_bool(
        message: &str,
        default: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut question = Question::confirm("confirm").message(message);
        if let Some(d) = default {
            question = question.default(d);
        }
        let answer = prompt_one(question.build())?;
        answer
            .try_into_bool()
            .map_err(|_| "Failed to convert answer to bool".into())
    }

    fn prompt_int(message: &str, default: Option<i64>) -> Result<i64, Box<dyn std::error::Error>> {
        let mut question = Question::int("int").message(message);
        if let Some(d) = default {
            question = question.default(d);
        }
        let answer = prompt_one(question.build())?;
        answer
            .try_into_int()
            .map_err(|_| "Failed to convert answer to int".into())
    }

    fn prompt_float(
        message: &str,
        default: Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut question = Question::float("float").message(message);
        if let Some(d) = default {
            question = question.default(d);
        }
        let answer = prompt_one(question.build())?;
        answer
            .try_into_float()
            .map_err(|_| "Failed to convert answer to float".into())
    }

    fn prompt_password(message: &str) -> Result<String, Box<dyn std::error::Error>> {
        let question = Question::password("password").message(message).build();
        let answer = prompt_one(question)?;
        answer
            .try_into_string()
            .map_err(|_| "Failed to convert answer to string".into())
    }

    fn prompt_editor(
        message: &str,
        default: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut question = Question::editor("editor").message(message);
        if let Some(d) = default {
            question = question.default(d);
        }
        let answer = prompt_one(question.build())?;
        answer
            .try_into_string()
            .map_err(|_| "Failed to convert answer to string".into())
    }
}

/// Implementation of the wizard prompting backend using dialoguer.
///
/// This struct provides the concrete implementation for interactive
/// CLI prompts using the dialoguer crate.
pub struct DialoguerWizard;

impl WizardPromptingBackend for DialoguerWizard {
    fn prompt_string(
        message: &str,
        default: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        use dialoguer::Input;
        let mut input = Input::<String>::new().with_prompt(message);
        if let Some(d) = default {
            input = input.default(d);
        }
        Ok(input.interact_text()?)
    }

    fn prompt_bool(
        message: &str,
        default: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        use dialoguer::Confirm;
        let mut confirm = Confirm::new().with_prompt(message);
        if let Some(d) = default {
            confirm = confirm.default(d);
        }
        Ok(confirm.interact()?)
    }

    fn prompt_int(message: &str, default: Option<i64>) -> Result<i64, Box<dyn std::error::Error>> {
        use dialoguer::Input;
        let mut input = Input::<i64>::new().with_prompt(message);
        if let Some(d) = default {
            input = input.default(d);
        }
        Ok(input.interact_text()?)
    }

    fn prompt_float(
        message: &str,
        default: Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        use dialoguer::Input;
        let mut input = Input::<f64>::new().with_prompt(message);
        if let Some(d) = default {
            input = input.default(d);
        }
        Ok(input.interact_text()?)
    }

    fn prompt_password(message: &str) -> Result<String, Box<dyn std::error::Error>> {
        use dialoguer::Password;
        Ok(Password::new().with_prompt(message).interact()?)
    }

    fn prompt_editor(
        _message: &str,
        default: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        use dialoguer::Editor;
        let content = Editor::new()
            .require_save(true)
            .edit(&default.unwrap_or_default())?
            .ok_or("Editor was cancelled")?;
        Ok(content)
    }
}
