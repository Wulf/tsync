#[derive(Debug)]
pub struct TsType {
    pub ts_type: String,
    pub is_optional: bool,
}

impl From<String> for TsType {
    fn from(ts_type: String) -> Self {
        Self {
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

/// Returns Err(()) when no match is found
fn try_match_ident_str(ident: &str) -> Result<String, ()> {
    match ident {
        "i8" => Ok("number".to_owned()),
        "u8" => Ok("number".to_owned()),
        "i16" => Ok("number".to_owned()),
        "u16" => Ok("number".to_owned()),
        "i32" => Ok("number".to_owned()),
        "u32" => Ok("number".to_owned()),
        "i64" => Ok("number".to_owned()),
        "u64" => Ok("number".to_owned()),
        "i128" => Ok("number".to_owned()),
        "u128" => Ok("number".to_owned()),
        "isize" => Ok("number".to_owned()),
        "usize" => Ok("number".to_owned()),
        "f32" => Ok("number".to_owned()),
        "f64" => Ok("number".to_owned()),
        "bool" => Ok("boolean".to_owned()),
        "char" => Ok("string".to_owned()),
        "str" => Ok("string".to_owned()),
        "String" => Ok("string".to_owned()),
        "NaiveDateTime" => Ok("Date".to_owned()),
        "DateTime" => Ok("Date".to_owned()),
        "Uuid" => Ok("string".to_owned()),
        _ => Err(()),
    }
}

/// Returns Err(()) when no match is found
fn try_match_with_args(ident: &str, args: &syn::PathArguments) -> Result<TsType, ()> {
    match ident {
        "Cow" => Ok(match &args {
            syn::PathArguments::AngleBracketed(angle_bracketed_argument) => {
                let Some(arg) = angle_bracketed_argument
                    .args
                    .iter()
                    .find(|arg| matches!(arg, syn::GenericArgument::Type(_)))
                else {
                    return Ok("unknown".to_owned().into());
                };

                convert_generic(arg).ts_type.into()
            }
            _ => "unknown".to_owned().into(),
        }),
        "Option" => Ok(TsType {
            is_optional: true,
            ts_type: match &args {
                syn::PathArguments::Parenthesized(parenthesized_argument) => {
                    format!("{:?}", parenthesized_argument)
                }
                syn::PathArguments::AngleBracketed(angle_bracketed_argument) => {
                    convert_generic(angle_bracketed_argument.args.first().unwrap()).ts_type
                }
                _ => "unknown".to_owned(),
            },
        }),
        "Vec" => Ok(match &args {
            syn::PathArguments::Parenthesized(parenthesized_argument) => {
                format!("{:?}", parenthesized_argument).into()
            }
            syn::PathArguments::AngleBracketed(angle_bracketed_argument) => format!(
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
            )
            .into(),
            _ => "unknown".to_owned().into(),
        }),
        "HashMap" => Ok(match &args {
            syn::PathArguments::Parenthesized(parenthesized_argument) => {
                format!("{:?}", parenthesized_argument).into()
            }
            syn::PathArguments::AngleBracketed(angle_bracketed_argument) => format!(
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
            )
            .into(),
            _ => "unknown".to_owned().into(),
        }),
        _ => Err(()),
    }
}

pub fn extract_custom_type(segment: &syn::PathSegment) -> Result<TsType, ()> {
    let ident = segment.ident.to_string();
    let args = &segment.arguments;

    match args {
        syn::PathArguments::None => Ok(ident.into()),
        syn::PathArguments::AngleBracketed(angle_bracketed_argument) => {
            let args = angle_bracketed_argument
                .args
                .iter()
                .map(|arg| match convert_generic(arg) {
                    TsType {
                        is_optional: true,
                        ts_type,
                    } => format!("{} | undefined", ts_type),
                    TsType {
                        is_optional: false,
                        ts_type,
                    } => ts_type,
                })
                .collect::<Vec<String>>()
                .join(", ");

            Ok(format!("{}<{}>", ident, args).into())
        }
        syn::PathArguments::Parenthesized(_parenthesized_argument) => {
            Err(()) // tuples are not supported yet
        }
    }
}

pub fn convert_type(ty: &syn::Type) -> TsType {
    match ty {
        syn::Type::Reference(p) => convert_type(&p.elem),
        syn::Type::Path(p) => {
            let segment = p.path.segments.last().unwrap();
            let identifier = segment.ident.to_string();

            if let Ok(ts_type) = try_match_ident_str(&identifier) {
                ts_type.into()
            } else if let Ok(ts_type) = try_match_with_args(&identifier, &segment.arguments) {
                ts_type
            } else if let Ok(ts_type) = extract_custom_type(segment) {
                ts_type
            } else {
                "unknown".to_owned().into()
            }
        }
        syn::Type::Tuple(t) => {
            let types = t
                .elems
                .iter()
                .map(convert_type)
                .map(|ty| {
                    if ty.is_optional {
                        format!("{} | undefined", ty.ts_type)
                    } else {
                        ty.ts_type
                    }
                })
                .collect::<Vec<String>>()
                .join(", ");

            TsType {
                ts_type: format!("[{types}]"),
                is_optional: false,
            }
        }
        _ => "unknown".to_owned().into(),
    }
}
