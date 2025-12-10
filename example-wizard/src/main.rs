use derive_wizard::Wizard;

#[derive(Wizard)]
struct Config {
    #[prompt("Enter the server address:")]
    server: String,

    #[prompt("Enter the port number:")]
    port: u16,
}

fn main() {
    let config = Config::wizard();
}
