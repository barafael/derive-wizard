use derive_wizard::Wizard;

/// This example demonstrates how defaults work in the egui backend.
/// The egui backend shows default values as placeholder text (hint text) in text fields.
/// When using wizard_with_defaults(), the current values are used as defaults.
#[derive(Debug, Clone, Wizard)]
struct AppSettings {
    #[prompt("Application name:")]
    app_name: String,

    #[prompt("Port number (default: 8080):")]
    #[min(1024)]
    #[max(65535)]
    port: i32,

    #[prompt("Max connections (default: 100):")]
    #[min(1)]
    #[max(10000)]
    max_connections: i32,

    #[prompt("Timeout in seconds (default: 30.0):")]
    #[min(0.1)]
    #[max(300.0)]
    timeout: f64,

    #[prompt("Enable debug mode:")]
    debug_mode: bool,

    #[prompt("Log level:")]
    log_level: String,
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 450.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Defaults Demo - egui",
        options,
        Box::new(|_cc| Ok(Box::new(DefaultsDemoApp::new()))),
    )
}

struct DefaultsDemoApp {
    backend: Option<derive_wizard::EguiBackend>,
    current_settings: Option<AppSettings>,
    mode: AppMode,
}

enum AppMode {
    FirstRun,
    Editing,
    Display,
}

impl DefaultsDemoApp {
    fn new() -> Self {
        // Create initial settings with some default values
        let initial_settings = AppSettings {
            app_name: "My Application".to_string(),
            port: 8080,
            max_connections: 100,
            timeout: 30.0,
            debug_mode: false,
            log_level: "info".to_string(),
        };

        Self {
            backend: None,
            current_settings: Some(initial_settings),
            mode: AppMode::Display,
        }
    }

    fn start_editing(&mut self) {
        if let Some(settings) = &self.current_settings {
            // Create an interview with current values as defaults
            let mut interview = AppSettings::interview();

            // Manually set defaults from current settings
            if let Some(derive_wizard::interview::Section::Sequence(seq)) =
                interview.sections.get_mut(0)
            {
                use derive_wizard::question::QuestionKind;

                for question in &mut seq.sequence {
                    match question.kind() {
                        QuestionKind::Input(q) => {
                            if question.name() == "app_name" {
                                let mut q = q.clone();
                                q.default = Some(settings.app_name.clone());
                                *question = derive_wizard::question::Question::new(
                                    question.id().map(|s| s.to_string()),
                                    question.name().to_string(),
                                    question.prompt().to_string(),
                                    QuestionKind::Input(q),
                                );
                            } else if question.name() == "log_level" {
                                let mut q = q.clone();
                                q.default = Some(settings.log_level.clone());
                                *question = derive_wizard::question::Question::new(
                                    question.id().map(|s| s.to_string()),
                                    question.name().to_string(),
                                    question.prompt().to_string(),
                                    QuestionKind::Input(q),
                                );
                            }
                        }
                        QuestionKind::Int(q) => {
                            let default = match question.name() {
                                "port" => Some(settings.port as i64),
                                "max_connections" => Some(settings.max_connections as i64),
                                _ => None,
                            };
                            if let Some(default) = default {
                                let mut q = q.clone();
                                q.default = Some(default);
                                *question = derive_wizard::question::Question::new(
                                    question.id().map(|s| s.to_string()),
                                    question.name().to_string(),
                                    question.prompt().to_string(),
                                    QuestionKind::Int(q),
                                );
                            }
                        }
                        QuestionKind::Float(q) => {
                            if question.name() == "timeout" {
                                let mut q = q.clone();
                                q.default = Some(settings.timeout);
                                *question = derive_wizard::question::Question::new(
                                    question.id().map(|s| s.to_string()),
                                    question.name().to_string(),
                                    question.prompt().to_string(),
                                    QuestionKind::Float(q),
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }

            self.backend = Some(derive_wizard::EguiBackend::new(interview));
            self.mode = AppMode::Editing;
        }
    }

    fn start_fresh(&mut self) {
        let interview = AppSettings::interview();
        self.backend = Some(derive_wizard::EguiBackend::new(interview));
        self.mode = AppMode::FirstRun;
    }
}

impl eframe::App for DefaultsDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.mode {
            AppMode::FirstRun | AppMode::Editing => {
                if let Some(backend) = &mut self.backend {
                    let completed = backend.show(ctx);

                    if completed {
                        if let Some(Ok(answers)) = backend.result() {
                            match AppSettings::from_answers(answers) {
                                Ok(settings) => {
                                    println!("Settings saved:");
                                    println!("{:#?}", settings);
                                    self.current_settings = Some(settings);
                                    self.backend = None;
                                    self.mode = AppMode::Display;
                                }
                                Err(e) => {
                                    eprintln!("Error building settings: {}", e);
                                }
                            }
                        }
                    }
                }
            }
            AppMode::Display => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if let Some(settings) = &self.current_settings {
                        ui.heading("Current Application Settings");
                        ui.separator();
                        ui.add_space(10.0);

                        egui::Grid::new("settings_grid")
                            .num_columns(2)
                            .spacing([40.0, 8.0])
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("Application Name:");
                                ui.label(&settings.app_name);
                                ui.end_row();

                                ui.label("Port:");
                                ui.label(settings.port.to_string());
                                ui.end_row();

                                ui.label("Max Connections:");
                                ui.label(settings.max_connections.to_string());
                                ui.end_row();

                                ui.label("Timeout:");
                                ui.label(format!("{:.1}s", settings.timeout));
                                ui.end_row();

                                ui.label("Debug Mode:");
                                ui.label(if settings.debug_mode { "Enabled" } else { "Disabled" });
                                ui.end_row();

                                ui.label("Log Level:");
                                ui.label(&settings.log_level);
                                ui.end_row();
                            });

                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(10.0);

                        ui.vertical(|ui| {
                            if ui.button("Edit Settings").clicked() {
                                self.start_editing();
                            }

                            ui.add_space(5.0);

                            if ui.button("Create New Configuration").clicked() {
                                self.start_fresh();
                            }
                        });

                        ui.add_space(15.0);
                        ui.separator();
                        ui.add_space(5.0);

                        ui.label("About this demo:");
                        ui.add_space(5.0);
                        ui.label("• String fields show default values as placeholder text (hint text)");
                        ui.label("• Int and Float fields display their min/max constraints as drag values");
                        ui.label("• If you leave a text field empty, it uses the placeholder as the default");
                        ui.label("• Bool fields are shown as checkboxes (default: false)");
                        ui.label("• Click 'Edit Settings' to see current values as placeholders");
                        ui.label("• Click 'Create New' to start fresh with no defaults");
                    }
                });
            }
        }
    }
}
