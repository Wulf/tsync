use syn::PathArguments;

#[derive(Debug)]
pub struct TsType {
    pub ts_type: String,
    pub is_optional: bool,
}

impl From<String> for TsType {
    fn from(ts_type: String) -> TsType {
        TsType {
            ts_type,
            is_optional: false,
        }
    }
}

fn convert_generic(gen_ty: &syn::GenericArgument) -> TsType {
    match gen_ty {
        syn::GenericArgument::Type(ty) => convert_type(ty),
        _ => "unknown".to_string().into(),
    }
}

fn check_cow(cow: &str) -> String {
    if !cow.contains("Cow<") {
        return cow.to_owned();
    }

    if let Some(comma_pos) = cow.chars().enumerate().find(|(_, c)| c == &',').map(|(i, _)| i) {
        let cow = cow[comma_pos + 1..].trim();
        if let Some(c) = cow.strip_suffix('>') {
            return try_match_ident_str(c);
        }
    }

    cow.to_owned()
}

fn try_match_ident_str(ident: &str) -> String {
    match ident {
        "i8" => "number".to_owned(),
        "u8" => "number".to_owned(),
        "i16" => "number".to_owned(),
        "u16" => "number".to_owned(),
        "i32" => "number".to_owned(),
        "u32" => "number".to_owned(),
        "i64" => "number".to_owned(),
        "u64" => "number".to_owned(),
        "i128" => "number".to_owned(),
        "u128" => "number".to_owned(),
        "isize" => "number".to_owned(),
        "usize" => "number".to_owned(),
        "f32" => "number".to_owned(),
        "f64" => "number".to_owned(),
        "bool" => "boolean".to_owned(),
        "char" => "string".to_owned(),
        "str" => "string".to_owned(),
        "String" => "string".to_owned(),
        "NaiveDateTime" => "Date".to_owned(),
        "DateTime" => "Date".to_owned(),
        "Uuid" => "string".to_owned(),
        x if x.contains("Cow") => check_cow(x),
        _ => ident.to_owned(),
    }
}

fn try_match_with_args(ident: &str, args: &PathArguments) -> TsType {
    match ident {
        "Option" => TsType {
            is_optional: true,
            ts_type: match &args {
                PathArguments::Parenthesized(parenthesized_argument) => {
                    format!("{:?}", parenthesized_argument)
                }
                PathArguments::AngleBracketed(angle_bracketed_argument) => {
                    convert_generic(angle_bracketed_argument.args.first().unwrap()).ts_type
                }
                _ => "unknown".to_owned(),
            },
        },
        "Vec" => match &args {
            PathArguments::Parenthesized(parenthesized_argument) => {
                format!("{:?}", parenthesized_argument).into()
            }
            PathArguments::AngleBracketed(angle_bracketed_argument) => {
                format!(
                    "Array<{}>",
                    match convert_generic(angle_bracketed_argument.args.first().unwrap()) {
                        TsType {
                            is_optional: true,
                            ts_type,
                        } => format!("{} | undefined", ts_type),
                        TsType {
                            is_optional: false,
                            ts_type,
                        } => ts_type,
                    }
                ).into()
            }
            _ => "unknown".to_owned().into(),
        },
        "HashMap" => match &args {
            PathArguments::Parenthesized(parenthesized_argument) => {
                format!("{:?}", parenthesized_argument).into()
            }
            PathArguments::AngleBracketed(angle_bracketed_argument) => {
                format!(
                    "Record<{}>",
                    angle_bracketed_argument
                        .args
                        .iter()
                        .map(|arg| {
                            match convert_generic(arg) {
                                TsType {
                                    is_optional: true,
                                    ts_type,
                                } => format!("{} | undefined", ts_type),
                                TsType {
                                    is_optional: false,
                                    ts_type,
                                } => ts_type,
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(", ")
                ).into()
            }
            _ => "unknown".to_owned().into(),
        },
        _ => ident.to_owned().into()
    }
}

const COMPLEX_TYPES: [&str; 3usize] = ["Option", "Vec", "HashMap"];


pub fn convert_type(ty: &syn::Type) -> TsType {
    match ty {
        syn::Type::Reference(p) => convert_type(&p.elem),
        syn::Type::Path(p) => {
            let segment = p.path.segments.last().unwrap();
            let identifier = segment.ident.to_string();
            if COMPLEX_TYPES.contains(&identifier.as_str()) {
                try_match_with_args(&identifier, &segment.arguments)
            } else {
                try_match_ident_str(&identifier).into()
            }
        }
        _ => "unknown".to_owned().into(),
    }
}