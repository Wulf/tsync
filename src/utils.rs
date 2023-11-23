use quote::ToTokens;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Expr, ExprPath, MetaNameValue, Token};

pub(crate) static RENAME_RULES: &[(&str, convert_case::Case)] = &[
    ("lowercase", convert_case::Case::Lower),
    ("UPPERCASE", convert_case::Case::Upper),
    ("PascalCase", convert_case::Case::Pascal),
    ("camelCase", convert_case::Case::Camel),
    ("snake_case", convert_case::Case::Snake),
    ("SCREAMING_SNAKE_CASE", convert_case::Case::ScreamingSnake),
    ("kebab-case", convert_case::Case::Kebab),
    // ("SCREAMING-KEBAB-CASE", _), // not supported by convert_case
];

pub fn has_attribute(needle: &str, attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(|attr| {
        attr.path()
            .segments
            .iter()
            .any(|segment| segment.ident == needle)
    })
}

fn check_expression_is_path(expr: Expr) -> Option<ExprPath> {
    match expr {
        Expr::Path(expr_path) if !expr_path.path.segments.is_empty() => Some(expr_path),
        _ => None,
    }
}

fn check_token(token: proc_macro2::TokenTree, arg: &str) -> Option<String> {
    // this detects the '(...)' part in #[serde(rename_all = "UPPERCASE", tag = "type")]
    // we can use this to get the value of a particular argument
    // or to see if it exists at all

    // make sure the delimiter is what we're expecting
    let proc_macro2::TokenTree::Group(group) = token
    else
    {
        return None;
    };

    // this detects the '(...)' part in #[serde(rename_all = "UPPERCASE", tag = "type")]
    // we can use this to get the value of a particular argument
    // or to see if it exists at all

    // make sure the delimiter is what we're expecting
    if group.delimiter() != proc_macro2::Delimiter::Parenthesis {
        return None;
    }

    match ::syn::parse::Parser::parse2(
        Punctuated::<MetaNameValue, Token![,]>::parse_terminated,
        group.stream(),
    ) {
        Ok(name_value_pairs) => {
            name_value_pairs
                .into_iter()
                .find(|nvp| nvp.path.is_ident(arg))
                .map(|nvp| nvp.value.to_token_stream().to_string())
                // removes quotes around the value
                .map(|value| value[1..value.len() - 1].to_owned())
        }
        Err(_) => {
            Parser::parse2(
                Punctuated::<Expr, Token![,]>::parse_terminated,
                group.stream(),
            )
            .map_or(None, |comma_seperated_values| {
                comma_seperated_values
                    .into_iter()
                    .map_while(check_expression_is_path)
                    .any(|expr_path| expr_path.path.segments[0].ident.to_string().eq(arg))
                    .then_some(arg.to_owned())
            })
        }
    }
}

/// Get the value matching an attribute and argument combination
///
/// For #[serde(tag = "type")], get_attribute_arg("serde", "tag", attributes) will return Some("type")
/// For #[derive(Serialize_repr)], get_attribute_arg("derive", "Serialize_repr", attributes) will return Some("Serialize_repr")
pub fn get_attribute_arg(needle: &str, arg: &str, attributes: &[syn::Attribute]) -> Option<String> {
    // check if attribute list contains the argument we are interested in
    // TODO: don't use a for loop here or iterator here
    get_attribute(needle, attributes).and_then(|attr| {
        attr.meta
            .to_token_stream()
            .into_iter()
            .filter_map(|token| check_token(token, arg))
            .next()
    })
}

/// Check has an attribute arg.
pub fn has_attribute_arg(needle: &str, arg: &str, attributes: &[syn::Attribute]) -> bool {
    get_attribute_arg(needle, arg, attributes).is_some()
}

fn check_token_tree(tt: proc_macro2::TokenTree) -> Option<String> {
    let proc_macro2::TokenTree::Literal(comment) = tt else { return None; };
    let c = comment.to_string();
    Some(c[1..c.len() - 1].trim().to_owned())
}

fn check_attribute(attr: &syn::Attribute) -> Vec<String> {
    let syn::Meta::NameValue(ref nv) = attr.meta else { return Vec::default(); };
    nv.value
        .to_token_stream()
        .into_iter()
        .filter_map(check_token_tree)
        .collect::<Vec<String>>()
}

/// Get the doc string comments from the syn::attributes
/// note: the compiler transforms doc comments into attributes
/// see: https://docs.rs/syn/2.0.28/syn/struct.Attribute.html#doc-comments
pub fn get_comments(mut attributes: Vec<syn::Attribute>) -> Vec<String> {
    attributes.retain(|x| x.path().segments.iter().any(|seg| seg.ident == "doc"));

    attributes
        .iter()
        .flat_map(check_attribute)
        .collect::<Vec<String>>()
}

pub fn build_indentation(indentation_amount: i8) -> String {
    (0..indentation_amount).map(|_| ' ').collect()
}

pub fn extract_struct_generics(s: syn::Generics) -> String {
    let out: Vec<String> = s
        .params
        .into_iter()
        .filter_map(|gp| {
            if let syn::GenericParam::Type(ty) = gp {
                Some(ty)
            } else {
                None
            }
        })
        .map(|ty| ty.ident.to_string())
        .collect();

    out.is_empty()
        .then(Default::default)
        .unwrap_or(format!("<{}>", out.join(", ")))
}

/// Get the attribute matching needle name.
pub fn get_attribute<'a>(
    needle: &'a str,
    attributes: &'a [syn::Attribute],
) -> Option<&'a syn::Attribute> {
    // if multiple attributes pass the conditions
    // we still want to return the last
    attributes.iter().rev().find(|attr| {
        attr.meta
            .path()
            .segments
            .iter()
            .any(|segment| segment.ident == needle)
    })
}

pub(crate) fn parse_serde_case(val: impl Into<Option<String>>) -> Option<convert_case::Case> {
    val.into().and_then(|x| {
        RENAME_RULES
            .iter()
            .find(|(name, _)| name == &x)
            .map(|(_, rule)| *rule)
    })
}
