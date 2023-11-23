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

pub fn convert_type(ty: &syn::Type) -> TsType {
    match ty {
        syn::Type::Reference(p) => convert_type(&p.elem),
        syn::Type::Path(p) => {
            let segment = p.path.segments.last().unwrap();
            let ident = &segment.ident;
            let arguments = &segment.arguments;
            let identifier = ident.to_string();
            match identifier.as_str() {
                "i8" => "number".to_owned().into(),
                "u8" => "number".to_owned().into(),
                "i16" => "number".to_owned().into(),
                "u16" => "number".to_owned().into(),
                "i32" => "number".to_owned().into(),
                "u32" => "number".to_owned().into(),
                "i64" => "number".to_owned().into(),
                "u64" => "number".to_owned().into(),
                "i128" => "number".to_owned().into(),
                "u128" => "number".to_owned().into(),
                "isize" => "number".to_owned().into(),
                "usize" => "number".to_owned().into(),
                "f32" => "number".to_owned().into(),
                "f64" => "number".to_owned().into(),
                "bool" => "boolean".to_owned().into(),
                "char" => "string".to_owned().into(),
                "str" => "string".to_owned().into(),
                "String" => "string".to_owned().into(),
                "NaiveDateTime" => "Date".to_owned().into(),
                "DateTime" => "Date".to_owned().into(),
                "Uuid" => "string".to_owned().into(),
                x if x.contains("Cow") => "string".to_owned().into(),
                "Option" => TsType {
                    is_optional: true,
                    ts_type: match arguments {
                        syn::PathArguments::Parenthesized(parenthesized_argument) => {
                            format!("{:?}", parenthesized_argument)
                        }
                        syn::PathArguments::AngleBracketed(anglebracketed_argument) => {
                            convert_generic(anglebracketed_argument.args.first().unwrap()).ts_type
                        }
                        _ => "unknown".to_owned(),
                    },
                },
                "Vec" => match arguments {
                    syn::PathArguments::Parenthesized(parenthesized_argument) => {
                        format!("{:?}", parenthesized_argument)
                    }
                    syn::PathArguments::AngleBracketed(anglebracketed_argument) => {
                        format!(
                            "Array<{}>",
                            match convert_generic(anglebracketed_argument.args.first().unwrap()) {
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
                    }
                    _ => "unknown".to_owned(),
                }
                .into(),
                "HashMap" => match arguments {
                    syn::PathArguments::Parenthesized(parenthesized_argument) => {
                        format!("{:?}", parenthesized_argument)
                    }
                    syn::PathArguments::AngleBracketed(anglebracketed_argument) => {
                        format!(
                            "Record<{}>",
                            anglebracketed_argument
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
                    }
                    _ => "unknown".to_owned(),
                }
                .into(),
                _ => identifier.to_string().into(),
            }
        }
        _ => "unknown".to_owned().into(),
    }
}
