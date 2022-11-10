use syn::{NestedMeta, __private::ToTokens};

pub fn has_attribute(needle: &str, attributes: &Vec<syn::Attribute>) -> bool {
    attributes.iter().any(|attr| {
        attr.path
            .segments
            .iter()
            .any(|segment| segment.ident.to_string() == needle)
    })
}

/// Get the value matching an attribute and argument combination
pub fn get_attribute_arg(
    needle: &str,
    arg: &str,
    attributes: &Vec<syn::Attribute>,
) -> Option<String> {
    // if multiple attributes pass the conditions
    // we still want to return the last
    for attr in attributes.iter().rev() {
        // check if correct attribute
        if attr
            .path
            .segments
            .iter()
            .any(|segment| segment.ident.to_string() == needle)
        {
            // check if attribute list contains the argument we are interested in
            if let Ok(syn::Meta::List(args)) = attr.parse_meta() {
                // accept the literal following the argument we want
                for subs in args.nested {
                    match subs {
                        NestedMeta::Meta(syn::Meta::NameValue(meta)) => {
                            // check if the meta refers to the argument we want
                            if meta
                                .path
                                .get_ident()
                                .filter(|x| &x.to_string() == arg)
                                .is_some()
                            {
                                if let syn::Lit::Str(out) = meta.lit {
                                    return Some(out.value());
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    None
}

/// Get the doc string comments from the syn::attributes
pub fn get_comments(attributes: Vec<syn::Attribute>) -> Vec<String> {
    let mut comments: Vec<String> = vec![];

    for attribute in attributes {
        let mut is_doc = false;
        for segment in attribute.path.segments {
            if segment.ident.to_string() == "doc" {
                is_doc = true;
                break;
            }
        }

        if is_doc {
            for token in attribute.tokens {
                match token {
                    syn::__private::quote::__private::TokenTree::Literal(comment) => {
                        let comment = comment.to_string();
                        let comment = comment[1..comment.len() - 1].trim();
                        comments.push(comment.to_string());
                    }
                    _ => { /* Do nothing */ }
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
        match generic_param {
            syn::GenericParam::Type(ty) => generic_params.push(ty.ident.to_string()),
            _ => {}
        }
    }

    if generic_params.len() == 0 {
        "".to_string()
    } else {
        format!("<{list}>", list = generic_params.join(", "))
    }
}
