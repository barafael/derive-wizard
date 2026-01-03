use derive_wizard::Wizard;

#[derive(Wizard, Debug)]
enum AccountType {
    Free,
    Premium {
        #[prompt("Select payment method")]
        payment: PaymentMethod,
    },
}

#[derive(Wizard, Debug)]
enum PaymentMethod {
    CreditCard,
    PayPal,
}

fn main() {
    let interview = AccountType::interview();
    println!("{:#?}", interview);
}
