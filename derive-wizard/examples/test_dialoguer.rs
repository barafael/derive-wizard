use derive_wizard::dialoguer_backend::*;

struct UserProfile {
    name: String,
    age: u32,
    likes_rust: bool,
}

impl DialoguerWizard for UserProfile {
    fn wizard_dialoguer() -> Self {
        let name = prompt_string("Enter your name");
        let age = prompt_number("Enter your age");
        let likes_rust = prompt_bool("Do you like Rust?");

        Self {
            name,
            age,
            likes_rust,
        }
    }
}

fn main() {
    let profile = UserProfile::wizard_dialoguer();
    println!(
        "RESULT: name={}, age={}, likes_rust={}",
        profile.name, profile.age, profile.likes_rust
    );
}
