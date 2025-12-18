use proc_macro2::TokenStream;
use quote::quote;
use syn::{Meta, parse_macro_input};

/// Helper function to check if a type is promptable (has a known conversion from Answer)
fn is_promptable_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                let ident = &segment.ident;
                let ident_str = ident.to_string();
                matches!(
                    ident_str.as_str(),
                    "String"
                        | "bool"
                        | "u8"
                        | "u16"
                        | "u32"
                        | "u64"
                        | "u128"
                        | "usize"
                        | "i8"
                        | "i16"
                        | "i32"
                        | "i64"
                        | "i128"
                        | "isize"
                        | "f32"
                        | "f64"
                        | "char"
                        | "PathBuf"
                )
            } else {
                false
            }
        }
        _ => false,
    }
}

#[proc_macro_derive(Wizard, attributes(prompt, mask, editor))]
pub fn wizard_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let ast = implement_wizard(&input);
    proc_macro::TokenStream::from(ast)
}

fn implement_wizard(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    match input.data {
        syn::Data::Struct(ref data_struct) => implement_struct_wizard(name, data_struct),
        syn::Data::Enum(ref data_enum) => implement_enum_wizard(name, data_enum),
        _ => syn::Error::new_spanned(name, "Wizard can only be derived for structs and enums")
            .to_compile_error(),
    }
}

fn implement_struct_wizard(name: &syn::Ident, data_struct: &syn::DataStruct) -> TokenStream {
    let mut fields = Vec::new();

    for field in &data_struct.fields {
        let mut prompt_attr = None;
        let mut has_mask = false;
        let mut has_editor = false;

        for attr in &field.attrs {
            if attr.path().is_ident("prompt") {
                prompt_attr = Some(attr.clone());
            } else if attr.path().is_ident("mask") {
                has_mask = true;
            } else if attr.path().is_ident("editor") {
                has_editor = true;
            }
        }

        if let Some(prompt) = prompt_attr {
            // Check for mutually exclusive attributes
            if has_mask && has_editor {
                return syn::Error::new_spanned(
                    field,
                    "Cannot use both #[mask] and #[editor] on the same field. They are mutually exclusive.",
                )
                .to_compile_error();
            }
            fields.push((field.clone(), Some(prompt), has_mask, has_editor));
        } else {
            return syn::Error::new_spanned(field, "Missing required #[prompt(\"...\")] attribute")
                .to_compile_error();
        }
    }

    let mut identifiers = Vec::new();
    let mut wizard_messages = Vec::new();
    for (field, attribute, has_mask, has_editor) in fields {
        let field_ident = field.ident.clone().unwrap();

        let attribute = attribute.unwrap();

        // Check if this is a wizard field by checking if the type is a known promptable type
        let is_wizard_field = !is_promptable_type(&field.ty);

        if is_wizard_field {
            // Extract optional message from #[wizard] or #[wizard("message")]
            let wizard_message = match &attribute.meta {
                Meta::List(meta_list) => Some(meta_list.tokens.clone()),
                Meta::Path(_) => None,
                _ => None,
            };
            wizard_messages.push((field_ident.clone(), wizard_message));
            identifiers.push((field_ident, None, field.ty));
            continue;
        }

        let prompt_attribute = attribute;

        // Parse the prompt attribute to extract the prompt string
        let prompt_text = match &prompt_attribute.meta {
            Meta::List(meta_list) => meta_list.tokens.clone(),
            _ => {
                return syn::Error::new_spanned(prompt_attribute, "Expected #[prompt(\"...\")]")
                    .to_compile_error();
            }
        };

        let field_name = field_ident.to_string();

        // Determine question type - priority: editor > mask > type inference
        let question_type = infer_question_type(&field.ty, has_mask, has_editor);
        let question =
            quote::quote! { Question::#question_type(#field_name).message(#prompt_text).build() };
        identifiers.push((field_ident, Some(question), field.ty));
    }

    let questions = identifiers
        .iter()
        .filter_map(|(ident, q, _)| q.as_ref().map(|q| quote::quote! {let #ident = #q;}))
        .collect::<TokenStream>();

    let prompts = identifiers
        .iter()
        .map(|(ident, q, t)| {
            if q.is_none() {
                // This is a #[wizard] field - call Type::wizard() directly
                match t {
                    syn::Type::Path(type_path) => {
                        let type_ident = &type_path.path.segments.last().unwrap().ident;

                        // Find the wizard message for this field
                        wizard_messages
                            .iter()
                            .find(|(msg_ident, _)| msg_ident == ident)
                            .and_then(|(_, msg)| msg.as_ref())
                            .map(|msg| {
                                quote::quote! {
                                    let #ident = #type_ident::wizard_with_message(#msg);
                                }
                            })
                            .unwrap_or_else(|| {
                                quote::quote! {
                                    let #ident = #type_ident::wizard();
                                }
                            })
                    }
                    _ => syn::Error::new_spanned(t, "Nested wizard fields must be named types")
                        .to_compile_error(),
                }
            } else {
                // Regular field - use prompt_one
                let into = infer_into(t);
                quote::quote! {
                    let #ident = prompt_one(#ident).unwrap()
                        #into;
                }
            }
        })
        .collect::<TokenStream>();

    let target = identifiers
        .iter()
        .map(|(ident, _, _)| {
            quote::quote! {
                #ident,
            }
        })
        .collect::<TokenStream>();

    let code = quote::quote! {
        impl Wizard for #name {
            fn wizard() -> Self {
                use derive_wizard::Question;
                use derive_wizard::prompt_one;

                #questions

                #prompts

                let result = Self {
                    #target
                };

                result
            }
        }
    };

    code
}

fn implement_enum_wizard(name: &syn::Ident, data_enum: &syn::DataEnum) -> TokenStream {
    // Create a list of variant names for selection
    let variant_names: Vec<String> = data_enum
        .variants
        .iter()
        .map(|v| v.ident.to_string())
        .collect();

    // Generate match arms for each variant
    let match_arms = data_enum.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let variant_name = variant_ident.to_string();

        match &variant.fields {
            syn::Fields::Named(fields) => {
                // Process named fields similar to struct
                let mut field_code = Vec::new();
                let mut field_idents = Vec::new();

                for field in &fields.named {
                    let field_ident = field.ident.as_ref().unwrap();
                    field_idents.push(field_ident);

                    let mut prompt_attr = None;
                    let mut has_mask = false;
                    let mut has_editor = false;
                    let mut has_wizard = false;

                    for attr in &field.attrs {
                        if attr.path().is_ident("prompt") {
                            prompt_attr = Some(attr.clone());
                        } else if attr.path().is_ident("mask") {
                            has_mask = true;
                        } else if attr.path().is_ident("editor") {
                            has_editor = true;
                        } else if attr.path().is_ident("wizard") {
                            has_wizard = true;
                        }
                    }

                    if has_wizard {
                        // Call nested wizard
                        if prompt_attr.is_some() || has_mask || has_editor {
                            return syn::Error::new_spanned(
                                field,
                                "#[wizard] attribute cannot be combined with #[prompt], #[mask], or #[editor]",
                            )
                            .to_compile_error();
                        }

                        match &field.ty {
                            syn::Type::Path(type_path) => {
                                let type_ident = &type_path.path.segments.last().unwrap().ident;
                                field_code.push(quote::quote! {
                                    let #field_ident = #type_ident::wizard();
                                });
                            }
                            _ => {
                                return syn::Error::new_spanned(
                                    &field.ty,
                                    "#[wizard] attribute can only be used on named types",
                                )
                                .to_compile_error();
                            }
                        }
                    } else if let Some(prompt) = prompt_attr {
                        if has_mask && has_editor {
                            return syn::Error::new_spanned(
                                field,
                                "Cannot use both #[mask] and #[editor] on the same field. They are mutually exclusive.",
                            )
                            .to_compile_error();
                        }

                        let prompt_text = match &prompt.meta {
                            Meta::List(meta_list) => meta_list.tokens.clone(),
                            _ => {
                                return syn::Error::new_spanned(prompt, "Expected #[prompt(\"...\")]")
                                    .to_compile_error();
                            }
                        };

                        let field_name = field_ident.to_string();
                        let question_type = infer_question_type(&field.ty, has_mask, has_editor);
                        let into = infer_into(&field.ty);

                        field_code.push(quote::quote! {
                            let #field_ident = Question::#question_type(#field_name).message(#prompt_text).build();
                            let #field_ident = prompt_one(#field_ident).unwrap()
                                #into;
                        });
                    } else {
                        return syn::Error::new_spanned(
                            field,
                            "Missing required #[prompt(\"...\")] or #[wizard] attribute",
                        )
                        .to_compile_error();
                    }
                }

                let construction = quote::quote! {
                    #name::#variant_ident {
                        #(#field_idents),*
                    }
                };

                quote::quote! {
                    #variant_name => {
                        #(#field_code)*
                        #construction
                    }
                }
            }
            syn::Fields::Unnamed(fields) => {
                // Process unnamed fields (tuple variants)
                let mut field_code = Vec::new();
                let mut field_idents = Vec::new();

                for (i, field) in fields.unnamed.iter().enumerate() {
                    let field_ident = syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site());
                    field_idents.push(field_ident.clone());

                    let mut prompt_attr = None;
                    let mut has_mask = false;
                    let mut has_editor = false;
                    let mut has_wizard = false;

                    for attr in &field.attrs {
                        if attr.path().is_ident("prompt") {
                            prompt_attr = Some(attr.clone());
                        } else if attr.path().is_ident("mask") {
                            has_mask = true;
                        } else if attr.path().is_ident("editor") {
                            has_editor = true;
                        } else if attr.path().is_ident("wizard") {
                            has_wizard = true;
                        }
                    }

                    if has_wizard {
                        if prompt_attr.is_some() || has_mask || has_editor {
                            return syn::Error::new_spanned(
                                field,
                                "#[wizard] attribute cannot be combined with #[prompt], #[mask], or #[editor]",
                            )
                            .to_compile_error();
                        }

                        match &field.ty {
                            syn::Type::Path(type_path) => {
                                let type_ident = &type_path.path.segments.last().unwrap().ident;
                                field_code.push(quote::quote! {
                                    let #field_ident = #type_ident::wizard();
                                });
                            }
                            _ => {
                                return syn::Error::new_spanned(
                                    &field.ty,
                                    "#[wizard] attribute can only be used on named types",
                                )
                                .to_compile_error();
                            }
                        }
                    } else if let Some(prompt) = prompt_attr {
                        if has_mask && has_editor {
                            return syn::Error::new_spanned(
                                field,
                                "Cannot use both #[mask] and #[editor] on the same field. They are mutually exclusive.",
                            )
                            .to_compile_error();
                        }

                        let prompt_text = match &prompt.meta {
                            Meta::List(meta_list) => meta_list.tokens.clone(),
                            _ => {
                                return syn::Error::new_spanned(prompt, "Expected #[prompt(\"...\")]")
                                    .to_compile_error();
                            }
                        };

                        let field_name = format!("{} field {}", variant_name, i);
                        let question_type = infer_question_type(&field.ty, has_mask, has_editor);
                        let into = infer_into(&field.ty);

                        field_code.push(quote::quote! {
                            let #field_ident = Question::#question_type(#field_name).message(#prompt_text).build();
                            let #field_ident = prompt_one(#field_ident).unwrap()
                                #into;
                        });
                    } else {
                        return syn::Error::new_spanned(
                            field,
                            "Missing required #[prompt(\"...\")] or #[wizard] attribute",
                        )
                        .to_compile_error();
                    }
                }

                quote::quote! {
                    #variant_name => {
                        #(#field_code)*
                        #name::#variant_ident(#(#field_idents),*)
                    }
                }
            }
            syn::Fields::Unit => {
                // Unit variant - no fields to prompt for
                quote::quote! {
                    #variant_name => #name::#variant_ident
                }
            }
        }
    });

    let variant_list_items = variant_names.iter().map(|name| {
        quote::quote! {
            #name
        }
    });

    quote::quote! {
        impl Wizard for #name {
            fn wizard() -> Self {
                Self::wizard_with_message("Select variant:")
            }

            fn wizard_with_message(message: &str) -> Self {
                use derive_wizard::{Question, prompt_one};

                let variant_question = Question::select("variant")
                    .message(message)
                    .choices(vec![#(#variant_list_items),*])
                    .build();

                let selected_variant = prompt_one(variant_question)
                    .unwrap()
                    .try_into_list_item()
                    .unwrap();

                let variant_name = selected_variant.text;

                match variant_name.as_str() {
                    #(#match_arms,)*
                    _ => unreachable!()
                }
            }
        }
    }
}

fn infer_question_type(ty: &syn::Type, has_mask: bool, has_editor: bool) -> TokenStream {
    if has_editor {
        return quote! { editor };
    }
    if has_mask {
        return quote! { password };
    }

    match ty {
        syn::Type::Path(type_path) => {
            let type_str = type_path
                .path
                .segments
                .iter()
                .map(|seg| seg.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");

            match type_str.as_str() {
                "PathBuf" => quote! { input },
                "String" => quote! { input },
                "bool" => quote! { confirm },
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => quote! { int },
                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => quote! { int },
                "f32" | "f64" => quote! { float },
                "ListItem" => quote! { select },
                "ExpandItem" => quote! { expand },
                _ if type_str.starts_with("Vec") => quote! { multi_select },
                _ => quote! { input },
            }
        }
        _ => quote! { input },
    }
}

fn infer_into(typ: &syn::Type) -> TokenStream {
    match typ {
        syn::Type::Path(type_path) => {
            let type_str = type_path
                .path
                .segments
                .iter()
                .map(|seg| seg.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");

            match type_str.as_str() {
                "PathBuf" => quote! { .try_into_string().map(PathBuf::from).unwrap() },
                "String" => quote! { .try_into_string().unwrap() },
                "bool" => quote! { .try_into_bool().unwrap() },
                ty @ ("i8" | "i16" | "i32" | "i64" | "isize") => {
                    let id = syn::Ident::new(ty, proc_macro2::Span::call_site());
                    quote! { .try_into_int().unwrap() as #id }
                }
                ty @ ("u8" | "u16" | "u32" | "u64" | "usize") => {
                    let id = syn::Ident::new(ty, proc_macro2::Span::call_site());
                    quote! { .try_into_int().unwrap() as #id }
                }
                ty @ ("f32" | "f64") => {
                    let id = syn::Ident::new(ty, proc_macro2::Span::call_site());
                    quote! { .try_into_float().unwrap() as #id }
                }
                "ListItem" => quote! { .try_into_list_item().unwrap() },
                "ExpandItem" => quote! { .try_into_expand_item().unwrap() },
                _ if type_str.starts_with("Vec") => {
                    quote! { .try_into_list_items().unwrap() }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}
