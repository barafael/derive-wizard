use proc_macro2::TokenStream;
use syn::{Meta, parse_macro_input};

#[proc_macro_derive(Wizard, attributes(prompt))]
pub fn wizard_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let ast = implement_wizard(&input);
    proc_macro::TokenStream::from(ast)
}

fn implement_wizard(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    let mut fields = Vec::new();

    match input.data {
        syn::Data::Struct(ref data_struct) => {
            for field in &data_struct.fields {
                for attr in &field.attrs {
                    if attr.path().is_ident("prompt") {
                        fields.push((field.clone(), attr.clone()));
                    }
                }
            }
        }
        _ => {
            return syn::Error::new_spanned(name, "Wizard can only be derived for structs")
                .to_compile_error();
        }
    }

    let mut identifiers = Vec::new();
    for (field, attribute) in fields {
        // Parse the attribute to extract the prompt string
        let prompt_text = match &attribute.meta {
            Meta::List(meta_list) => meta_list.tokens.clone(),
            _ => {
                return syn::Error::new_spanned(attribute, "Expected #[prompt(\"...\")]")
                    .to_compile_error();
            }
        };

        let field_ident = field.ident.clone().unwrap();
        let field_name = field_ident.to_string();
        let question = quote::quote! { Question::input(#field_name).message(#prompt_text).build() };
        identifiers.push((field_ident, question));
    }

    let questions = identifiers
        .iter()
        .map(|(ident, q)| quote::quote! {let #ident = #q;})
        .collect::<TokenStream>();

    let prompts = identifiers
        .iter()
        .map(|(ident, _)| {
            quote::quote! {
                let #ident = prompt_one(#ident).unwrap();
            }
        })
        .collect::<TokenStream>();

    let code = quote::quote! {
        use derive_wizard::Question;
        use derive_wizard::prompt_one;

        impl Wizard for #name {
            fn wizard() -> Self {
                #questions

                #prompts

                unimplemented!()
            }
        }
    };

    code.into()
}
