use derive_wizard::Wizard;

#[derive(Debug, Wizard)]
enum PaymentMethod {
    Cash,
    Card {
        #[prompt("Card number:")]
        number: String,
        
        #[prompt("CVV:")]
        #[mask]
        cvv: String,
    },
}

fn main() {
    let payment = PaymentMethod::wizard();
    match payment {
        PaymentMethod::Cash => println!("RESULT: Cash"),
        PaymentMethod::Card { number, cvv } => {
            println!("RESULT: Card number={}, cvv_len={}", number, cvv.len());
        }
    }
}
