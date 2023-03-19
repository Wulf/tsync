use syn::{Attribute, NestedMeta, __private::ToTokens};

pub fn has_attribute(needle: &str, attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(|attr| {
        attr.path
            .segments
            .iter()
            .any(|segment| segment.ident == needle)
    })
}

/// Get the value matching an attribute and argument combination
pub fn get_attribute_arg(
    needle: &str,
    arg: &str,
    attributes: &[syn::Attribute],
) -> Option<String> {
    if let Some(attr) = get_attribute(needle, attributes) {
        // check if attribute list contains the argument we are interested in
        if let Ok(syn::Meta::List(args)) = attr.parse_meta() {
            // accept the literal following the argument we want
            for subs in args.nested {
                if let NestedMeta::Meta(syn::Meta::NameValue(meta)) = subs {
                    // check if the meta refers to the argument we want
                    if meta
                        .path
                        .get_ident()
                        .filter(|x| *x == arg)
                        .is_some()
                    {
                        if let syn::Lit::Str(out) = meta.lit {
                            return Some(out.value());
                        }
                    }
                }
            }
        }
    }
    None
}

/// Check has an attribute arg.
pub fn has_attribute_arg(needle: &str, arg: &str, attributes: &[syn::Attribute]) -> bool {
    if let Some(attr) = get_attribute(needle, attributes) {
        // check if attribute list contains the argument we are interested in
        if let Ok(syn::Meta::List(args)) = attr.parse_meta() {
            // accept the literal following the argument we want
            for subs in args.nested {
                if let NestedMeta::Meta(meta) = subs {
                    // check if the meta refers to the argument we want
                    if meta.to_token_stream().to_string() == arg {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Get the doc string comments from the syn::attributes
pub fn get_comments(attributes: Vec<syn::Attribute>) -> Vec<String> {
    let mut comments: Vec<String> = vec![];

    for attribute in attributes {
        let mut is_doc = false;
        for segment in attribute.path.segments {
            if segment.ident == "doc" {
                is_doc = true;
                break;
            }
        }

        if is_doc {
            for token in attribute.tokens {
                if let proc_macro2::TokenTree::Literal(comment) = token {
                    let comment = comment.to_string();
                    let comment = comment[1..comment.len() - 1].trim();
                    comments.push(comment.to_string());
                }
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
            .path
            .segments
            .iter()
            .any(|segment| segment.ident == needle)
        {
            return Some(attr.clone());
        }
    }
    None
}
