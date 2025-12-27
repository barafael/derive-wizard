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

fn main() {
    println!("=== User Profile Wizard - egui Demo ===\n");

    // Use the egui backend
    let backend = derive_wizard::EguiBackend::new()
        .with_title("User Profile Wizard")
        .with_window_size([400.0, 300.0]);

    let profile = UserProfile::wizard_with_backend(&backend);

    println!("\n=== Profile Created ===");
    println!("{:#?}", profile);
}
