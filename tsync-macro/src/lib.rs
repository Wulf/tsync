use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// document this attribute
#[proc_macro_attribute]
pub fn tsync(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);

    // Remove tsync decorators from struct fields
    if let syn::Data::Struct(ref mut data) = input.data {
        if let syn::Fields::Named(ref mut fields) = data.fields {
            for field in fields.named.iter_mut() {
                // Remove all tsync attributes
                field.attrs.retain(|attr| !attr.path().is_ident("tsync"));
            }
        }
    }

    if let syn::Data::Enum(ref mut data) = input.data {
        for variant in data.variants.iter_mut() {
            // Remove all tsync attributes
            variant.attrs.retain(|attr| !attr.path().is_ident("tsync"));
        }
    }

    // Return the struct without the field-level tsync attributes
    quote! {
        #input
    }
    .into()
}
