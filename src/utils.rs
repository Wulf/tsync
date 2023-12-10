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

/// Ensures that a parsed expression is a valid Rust module-like path and the
/// path contains valid segments.
///
/// # Example
/// ```rust
/// let expression: syn::Result<syn::Expr> = syn::parse_str("std::mem::replace");
/// assert!(expression.is_ok());
/// let exp_path = expression.unwrap();
/// let is_path = match exp_path {
///     syn::Expr::Path(path_expression) if !path_expression.path.segments.is_empty() => Some(path_expression),
///     _ => None,
/// };
/// assert!(is_path.is_some());
/// let path = is_path.unwrap();
/// // Segments are syn::PathSegment objects, where their `ident`s are
/// // syn::Ident objects, representing the text between `::` separators.
/// assert_eq!(path.path.segments.len(), 3);
/// ```
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
    let proc_macro2::TokenTree::Group(group) = token
    else
    {
        return None;
    };

    // Make sure the delimiter is what we're expecting, otherwise return right away.
    if group.delimiter() != proc_macro2::Delimiter::Parenthesis {
        return None;
    }

    // First check to see if the group is a `MetaNameValue`, (.e.g `feature = "nightly"`)
    match Parser::parse2(Punctuated::<MetaNameValue, Token![,]>::parse_terminated, group.stream()) {
        Ok(name_value_pairs) => {
            // If so move the pairs into an iterator
            name_value_pairs
                .into_iter()
                // checking that the `path` component is of length 1 equal to the given arg.
                .find(|nvp| nvp.path.is_ident(arg))
                // If it is, get the `value` component, ("nightly" from the example above).
                .map(|nvp| nvp.value.to_token_stream().to_string())
                // Then remove the literal quotes around the value.
                .map(|value| value[1..value.len() - 1].to_owned())
        }
        Err(_) => {
            // Otherwise, check to see if the group is a `Expr` of `Punctuated<_, P>` attributes,
            // separated by `P`, `Token![,]` in this case.
            // (.e.g `default, skip_serializing`)
            Parser::parse2(Punctuated::<Expr, Token![,]>::parse_terminated, group.stream())
                // If the expression cannot be parsed, return None
                .map_or(None, |comma_seperated_values| {
                    // Otherwise move the pairs into an iterator
                    comma_seperated_values
                        .into_iter()
                        // Checking each is a `ExprPath`, object, yielding elements while the method
                        // returns true.
                        .map_while(check_expression_is_path)
                        // Check if any yielded paths equal `arg`
                        .any(|expr_path| expr_path.path.segments[0].ident.to_string().eq(arg))
                        // If so, return `Some(arg)`, otherwise `None`.
                        .then_some(arg.to_owned())
            })
        }
    }
}

/// Get the value matching an attribute and argument combination
///
/// For #[serde(tag = "type")], get_attribute_arg("serde", "tag", attributes) will return Some("type")
/// For #[derive(Serialize_repr)], get_attribute_arg("derive", "Serialize_repr", attributes) will
/// return Some("Serialize_repr")
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


/// Checks if a [`proc_macro2::TokenTree`] is a literal character ('a'), string ("hello"),
/// number (2.3), etc. If so, trim it to retain only the comment body, returning `Some(comment)`,
/// otherwise returns `None`.
///
/// Given an attribute like `#[doc = "Single line doc comments"]`, only `Single line doc comments`
/// should be returned.
fn check_doc_tokens(tt: proc_macro2::TokenTree) -> Option<String> {
    let proc_macro2::TokenTree::Literal(comment) = tt else { return None; };
    let c = comment.to_string();
    Some(c[1..c.len() - 1].trim().to_owned())
}

/// Checks if an attribute's [`syn::Meta`] property is a name-value pair, like
/// `doc = "Single line doc comments"`. if so, continues to check that the value
/// (.e.g. "Single line doc comments") is a valid [`proc_macro2::TokenTree::Literal`],
/// if so, add it to the collection of comment strings.
fn check_doc_attribute(attr: &syn::Attribute) -> Vec<String> {
    // Check if the attribute's meta is a NameValue, otherwise return
    // right away.
    let syn::Meta::NameValue(ref nv) = attr.meta else { return Default::default(); };

    // Convert the value to a token stream, then iterate it, collecting
    // only valid comment string.
    nv.value
        .to_token_stream()
        .into_iter()
        .filter_map(check_doc_tokens)
        .collect::<Vec<String>>()
}

/// Get the doc string comments from the syn::attributes
/// note: the compiler transforms doc comments into attributes
/// see: https://docs.rs/syn/2.0.28/syn/struct.Attribute.html#doc-comments
pub fn get_comments(mut attributes: Vec<syn::Attribute>) -> Vec<String> {
    // Retains only attributes that have segments equal to "doc".
    // (.e.g. #[doc = "Single line doc comments"])
    attributes.retain(|x| x.path().segments.iter().any(|seg| seg.ident == "doc"));

    attributes
        .iter()
        .flat_map(check_doc_attribute)
        .collect::<Vec<String>>()
}

/// Generate a string filled with `indentation_amount` white-space
/// literal `chars`.
///
/// # Example
/// ```rust
/// for i in 0..64 {
///     let indentations: String = (0..i).map(|_| '\u{0020}').collect();
///     assert_eq!(indentations.len(), i);
/// }
/// ```
pub fn build_indentation(indentation_amount: i8) -> String {
    // Change from empty whitespace char to Unicode whitespace
    // representation for a bit more clarity.
    (0..indentation_amount).map(|_| '\u{0020}').collect()
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
    attributes.iter()
        // Reverse the iterator to check the last attribute first.
        .rev()
        // From the `find` documentation:
        // "find() is short-circuiting;
        // in other words, it will stop processing as soon as the closure returns true."
        .find(|attr| {
            // Checks if any segments in the iterator equal `needle`. Returns
            // true if a match is found, otherwise false.
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