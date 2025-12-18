use derive_wizard::Wizard;

#[derive(Debug, Wizard)]
struct UserProfile {
    #[prompt("What is your name?")]
    name: String,

    #[prompt("How old are you?")]
    age: u32,

    #[prompt("Do you like Rust?")]
    likes_rust: bool,
}

fn main() {
    let profile = UserProfile::wizard();
    println!("RESULT: name={}, age={}, likes_rust={}", profile.name, profile.age, profile.likes_rust);
}
