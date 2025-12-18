use derive_wizard::Wizard;

#[derive(Debug, Wizard)]
struct UserProfile {
    #[prompt("Enter your username:")]
    #[description("Choose a unique username (alphanumeric characters only)")]
    username: String,

    #[prompt("Enter your email:")]
    email: String,

    #[prompt("Are you over 18?")]
    #[description("This information is required for age verification")]
    is_adult: bool,
}

fn main() {
    let profile = UserProfile::wizard();
    println!("User Profile: {profile:#?}");
}
