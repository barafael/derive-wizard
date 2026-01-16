//! LaTeX backend for derive-survey: generates fillable PDF forms from SurveyDefinition.

use derive_survey::SurveyDefinition;

/// Generate a LaTeX document (as a String) for a fillable form from a SurveyDefinition.
pub fn to_latex_form(survey: &SurveyDefinition) -> String {
    let mut latex = String::new();
    latex.push_str("\\documentclass{article}\n");
    latex.push_str("\\usepackage[pdftex,unicode]{hyperref}\n");
    latex.push_str("\\usepackage{geometry}\n");
    latex.push_str("\\hypersetup{pdfborder=0 0 0}\n");
    latex.push_str("\\geometry{margin=1in}\n");
    latex.push_str("\\begin{document}\n");
    if let Some(prelude) = &survey.prelude {
        latex.push_str(&format!("\n{}\\bigskip\n", prelude));
    }

    latex.push_str("\\begin{Form}\n\n");
    for q in &survey.questions {
        latex.push_str(&render_question(q));
        latex.push_str("\\bigskip\n");
    }

    latex.push_str("\\end{Form}\n");
    if let Some(epilogue) = &survey.epilogue {
        latex.push_str(&format!("\\bigskip\n{}\n", epilogue));
    }
    latex.push_str("\\end{document}");
    latex
}

fn render_question(q: &derive_survey::Question) -> String {
    use derive_survey::QuestionKind;
    let mut s = String::new();
    let ask = q.ask().replace("&", "\\&");
    s.push_str(&ask);
    match q.kind() {
        QuestionKind::Input(_) => {
            let field_name = q.path().as_str().replace("_", "\\_");
            s.push_str("\\\\\n\\TextField[name=");
            s.push_str(&field_name);
            s.push_str(",width=3in]{}\n\\medskip\n");
        }
        QuestionKind::Int(_) => {
            let field_name = q.path().as_str().replace("_", "\\_");
            s.push_str("\\\\\n\\TextField[name=");
            s.push_str(&field_name);
            s.push_str(",width=1in]{}\n");
            s.push_str("% (integer only)\n\\medskip\n");
        }
        QuestionKind::Confirm(_) => {
            let field_name = q.path().as_str().replace("_", "\\_");
            s.push_str("\\\\\n\\CheckBox[name=");
            s.push_str(&field_name);
            s.push_str("]{} Yes\n\\medskip\n");
        }
        QuestionKind::OneOf(oneof) => {
            let field_name = q.path().as_str().replace("_", "\\_");
            let options: Vec<String> = oneof.variants.iter().map(|v| v.name.clone()).collect();
            s.push_str("\\\\\n\\ChoiceMenu[popdown,name=");
            s.push_str(&field_name);
            s.push_str(",width=2in]{}{");
            s.push_str(&options.join(","));
            s.push_str("}\n\\medskip\n");
        }
        QuestionKind::AnyOf(anyof) => {
            for variant in &anyof.variants {
                let checkbox_name =
                    format!("{}_{}", q.path().as_str(), variant.name.replace(' ', "_"))
                        .replace("_", "\\_");
                s.push_str("\\\\\n\\CheckBox[name=");
                s.push_str(&checkbox_name);
                s.push_str("]{} ");
                s.push_str(&variant.name);
                s.push_str("\n");
            }
            s.push_str("\\medskip\n");
        }
        QuestionKind::AllOf(allof) => {
            for sub in &allof.questions {
                s.push_str(&render_question(sub));
            }
        }
        QuestionKind::Multiline(_) => {
            let field_name = q.path().as_str().replace("_", "\\_");
            s.push_str("\\\\\n\\TextField[name=");
            s.push_str(&field_name);
            s.push_str(",multiline=true,width=3in,height=1in]{}\n\\medskip\n");
        }
        QuestionKind::Unit => {
            s.push_str("% [Unit] No data to collect\n\\medskip\n");
        }
        QuestionKind::Masked(_) => {
            let field_name = q.path().as_str().replace("_", "\\_");
            s.push_str("\\\\\n\\TextField[name=");
            s.push_str(&field_name);
            s.push_str(",width=3in]{}\n\\medskip\n");
            s.push_str("% (not masked in PDF)\n\\medskip\n");
        }
        QuestionKind::Float(_) => {
            s.push_str("% [Float] Floating-point input not supported in LaTeX\n\\medskip\n");
        }
        QuestionKind::List(_) => {
            s.push_str("% [List] List input not supported in LaTeX\n\\medskip\n");
        }
    }
    s.push_str("\n");
    s
}
