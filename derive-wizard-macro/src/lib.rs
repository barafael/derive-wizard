use proc_macro::TokenStream;

#[proc_macro_derive(Wizard, attributes(prompt))]
pub fn wizard_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
