use derive_wizard::Wizard;

#[derive(Debug, Wizard)]
enum Gender {
    Male,
    Female,
    Other(#[prompt("Please specify:")] String),
}

#[derive(Debug, Wizard)]
struct UserProfile {
    #[prompt("Enter your name:")]
    name: String,

    #[prompt("Enter your age:")]
    #[min(0)]
    #[max(150)]
    age: i32,

    #[prompt("Enter your height (in meters):")]
    #[min(0.3)]
    #[max(3.0)]
    height: f64,

    #[prompt("Enter your email:")]
    email: String,

    #[prompt("Do you agree to the terms?")]
    agree: bool,

    #[prompt("Select your gender:")]
    gender: Gender,
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Interview Wizard - egui Demo",
        options,
        Box::new(|_cc| Ok(Box::new(WizardApp::new()))),
    )
}

struct WizardApp {
    backend: derive_wizard::EguiBackend,
    final_result: Option<UserProfile>,
}

impl WizardApp {
    fn new() -> Self {
        let interview = UserProfile::interview();
        Self {
            backend: derive_wizard::EguiBackend::new(interview),
            final_result: None,
        }
    }
}

impl eframe::App for WizardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let completed = self.backend.show(ctx);

        if completed && self.final_result.is_none() {
            if let Some(Ok(answers)) = self.backend.result() {
                match UserProfile::from_answers(answers) {
                    Ok(profile) => {
                        println!("Interview completed!");
                        println!("{:#?}", profile);
                        self.final_result = Some(profile);
                    }
                    Err(e) => {
                        eprintln!("Error building profile: {}", e);
                    }
                }
            }
        }

        if let Some(ref profile) = self.final_result {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Interview Complete!");
                ui.separator();
                ui.label(format!("Name: {}", profile.name));
                ui.label(format!("Age: {}", profile.age));
                ui.label(format!("Height: {:.2}m", profile.height));
                ui.label(format!("Email: {}", profile.email));
                ui.label(format!("Agreed: {}", profile.agree));
                ui.label(format!("Gender: {:?}", profile.gender));
            });
        }
    }
}
