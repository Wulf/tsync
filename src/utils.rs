use quote::ToTokens;
use syn::{punctuated::Punctuated, Attribute, ExprPath, MetaNameValue, Token};

pub fn has_attribute(needle: &str, attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(|attr| {
        attr.path()
            .segments
            .iter()
            .any(|segment| segment.ident == needle)
    })
}

/// Get the value matching an attribute and argument combination
///
/// For #[serde(tag = "type")], get_attribute_arg("serde", "tag", attributes) will return Some("type")
/// For #[derive(Serialize_repr)], get_attribute_arg("derive", "Serialize_repr", attributes) will return Some("Serialize_repr")
pub fn get_attribute_arg(needle: &str, arg: &str, attributes: &[syn::Attribute]) -> Option<String> {
    if let Some(attr) = get_attribute(needle, attributes) {
        // check if attribute list contains the argument we are interested in
        let mut found = false;
        let mut value = String::new();

        // TODO: don't use a for loop here or iterator here
        let tokens = attr.meta.to_token_stream().into_iter();
        for token in tokens {
            if let proc_macro2::TokenTree::Ident(ident) = token {
                // this detects the 'serde' part in #[serde(rename_all = "UPPERCASE")]
                // we use get_attribute to make sure we've gotten the right attribute,
                // hence, we'll ignore it here
            } else if let proc_macro2::TokenTree::Group(group) = token {
                // this detects the '(...)' part in #[serde(rename_all = "UPPERCASE", tag = "type")]
                // we can use this to get the value of a particular argument
                // or to see if it exists at all

                // make sure the delimiter is what we're expecting
                if group.delimiter() != proc_macro2::Delimiter::Parenthesis {
                    continue;
                }

                let name_value_pairs = ::syn::parse::Parser::parse2(
                    Punctuated::<MetaNameValue, Token![,]>::parse_terminated,
                    group.stream(),
                );

                if name_value_pairs.is_err() {
                    let comma_seperated_values = ::syn::parse::Parser::parse2(
                        Punctuated::<syn::Expr, Token![,]>::parse_terminated,
                        group.stream(),
                    );

                    if comma_seperated_values.is_err() {
                        continue;
                    }

                    let comma_seperated_values = comma_seperated_values.unwrap();

                    for comma_seperated_value in comma_seperated_values {
                        match comma_seperated_value {
                            syn::Expr::Path(expr_path) => {
                                let segments = expr_path.path.segments;

                                if segments.is_empty() {
                                    continue;
                                }

                                if segments[0].ident.to_string().eq(arg) {
                                    found = true;
                                    value = String::from(arg);

                                    break;
                                }
                            }
                            _ => continue,
                        }
                    }

                    continue;
                }

                let name_value_pairs = name_value_pairs.unwrap();

                for name_value_pair in name_value_pairs {
                    if name_value_pair.path.is_ident(arg) {
                        found = true;
                        value = name_value_pair.value.to_token_stream().to_string();
                        // removes quotes around the value
                        value = value[1..value.len() - 1].to_string();

                        break;
                    }
                }
            }
        }

        if found {
            return Some(value);
        } else {
            return None;
        }
    }
    None
}

/// Check has an attribute arg.
pub fn has_attribute_arg(needle: &str, arg: &str, attributes: &[syn::Attribute]) -> bool {
    get_attribute_arg(needle, arg, attributes).is_some()
}

/// Get the doc string comments from the syn::attributes
/// note: the compiler transforms doc comments into attributes
/// see: https://docs.rs/syn/2.0.28/syn/struct.Attribute.html#doc-comments
pub fn get_comments(attributes: Vec<syn::Attribute>) -> Vec<String> {
    let mut comments: Vec<String> = vec![];

    for attribute in attributes {
        let mut is_doc = false;
        for segment in attribute.path().segments.clone() {
            if segment.ident == "doc" {
                is_doc = true;
                break;
            }
        }

        if is_doc {
            match attribute.meta {
                syn::Meta::NameValue(name_value) => {
                    let comment = name_value.value.to_token_stream();

                    for token in comment.into_iter() {
                        if let proc_macro2::TokenTree::Literal(comment) = token {
                            let comment = comment.to_string();
                            let comment = comment[1..comment.len() - 1].trim();
                            comments.push(comment.to_string());
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    comments
}

pub fn build_indentation(indentation_amount: i8) -> String {
    let mut indent = "".to_string();
    for _ in 0..indentation_amount {
        indent.push(' ');
    }
    indent
}

pub fn extract_struct_generics(s: syn::Generics) -> String {
    let mut generic_params: Vec<String> = vec![];

    for generic_param in s.params {
        if let syn::GenericParam::Type(ty) = generic_param {
            generic_params.push(ty.ident.to_string());
        }
    }

    if generic_params.is_empty() {
        "".to_string()
    } else {
        format!("<{list}>", list = generic_params.join(", "))
    }
}

/// Get the attribute matching needle name.
pub fn get_attribute(needle: &str, attributes: &[syn::Attribute]) -> Option<Attribute> {
    // if multiple attributes pass the conditions
    // we still want to return the last
    for attr in attributes.iter().rev() {
        // check if correct attribute
        if attr
            .meta
            .path()
            .segments
            .iter()
            .any(|segment| segment.ident == needle)
        {
            return Some(attr.clone());
        }
    }
    None
}
