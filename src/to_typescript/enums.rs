use crate::{utils, BuildState};
use convert_case::{Case, Casing};
use syn::__private::ToTokens;

static RENAME_RULES: &[(&str, convert_case::Case)] = &[
    ("lowercase", Case::Lower),
    ("UPPERCASE", Case::Upper),
    ("PascalCase", Case::Pascal),
    ("camelCase", Case::Camel),
    ("snake_case", Case::Snake),
    ("SCREAMING_SNAKE_CASE", Case::ScreamingSnake),
    ("kebab-case", Case::Kebab),
    // ("SCREAMING-KEBAB-CASE", _), // not supported by convert_case
];

/// Conversion of Rust Enum to Typescript using external tagging as per https://serde.rs/enum-representations.html
/// however conversion will adhere to the `serde` `tag` such that enums are intenrally tagged
/// (while the other forms such as adjacent tagging aren't supported).
/// `renameAll` attributes for the name of the tag will also be adhered to.
impl super::ToTypescript for syn::ItemEnum {
    fn convert_to_ts(self, state: &mut BuildState, debug: bool) {
        // check we don't have any tuple structs that could mess things up.
        // if we do ignore this struct
        for variant in self.variants.iter() {
            for f in variant.fields.iter() {
                if f.ident.is_none() {
                    if debug {
                        println!("#[tsync] failed for enum {}", self.ident.to_string());
                    }
                    return;
                }
            }
        }

        state.types.push('\n');

        let comments = utils::get_comments(self.clone().attrs);
        let casing = utils::get_attribute_arg("serde", "renameAll", &self.attrs);
        let casing = to_enum_case(casing);

        let is_single = !self.variants.iter().any(|x| x.fields.len() > 0);
        state.write_comments(&comments, 0);

        if is_single {
            if utils::has_attribute_arg("derive", "Serialize_repr", &self.attrs) {
                make_numeric_enum(self, state, casing)
            } else {
                make_enum(self, state, casing)
            }
        } else {
            if let Some(tag_name) = utils::get_attribute_arg("serde", "tag", &self.attrs) {
                make_variant(tag_name, self, state, casing)
            } else {
                make_externally_tagged_variant(self, state, casing)
            }
        }
    }
}

/// This convert an all unit enums to a union of const strings in Typescript.
/// It will ignore any discriminants.  
fn make_enum(exported_struct: syn::ItemEnum, state: &mut BuildState, casing: Option<Case>) {
    state.types.push_str(&format!(
        "type {interface_name} =\n{space}",
        interface_name = exported_struct.ident.to_string(),
        space = utils::build_indentation(1)
    ));

    for variant in exported_struct.variants {
        let field_name = if let Some(casing) = casing {
            variant.ident.to_string().to_case(casing)
        } else {
            variant.ident.to_string()
        };
        state.types.push_str(&format!(" | \"{}\"", field_name));
    }

    state.types.push_str(";\n");
}

/// Numeric enums. These will be converted using enum syntax
/// ```ignore
/// enum Foo {
///     Bar,            // 0
///     Baz = 123,      // 123
///     Quux,           // 124
/// }
/// enum Animal {
///     Dog,
///     Cat,
/// }
/// ``` to the following
/// ```ignore
/// enum Foo {
///    Bar = 0,          
///    Baz = 123,     
///    Quux = 124,           
/// }
/// enum Animal {
///    Dog = 0,
///    Cat = 1,
/// }
/// ```
///
fn make_numeric_enum(exported_struct: syn::ItemEnum, state: &mut BuildState, casing: Option<Case>) {
    state.types.push_str(&format!(
        "enum {interface_name} {{",
        interface_name = exported_struct.ident.to_string()
    ));

    let mut num = 0;

    for variant in exported_struct.variants {
        state.types.push('\n');
        let field_name = if let Some(casing) = casing {
            variant.ident.to_string().to_case(casing)
        } else {
            variant.ident.to_string()
        };
        if let Some((_, disc)) = variant.discriminant {
            if let Ok(new_disc) = disc.to_token_stream().to_string().parse::<i32>() {
                num = new_disc;
            }
        }
        state
            .types
            .push_str(&format!("  {} = {},", field_name, num));
        num += 1;
    }

    state.types.push_str("\n}\n");
}

/// Conversion of Rust Enum to Typescript using internal tagging as per https://serde.rs/enum-representations.html
/// meaning tuple structs will not be support e.g.
/// ```ignore
/// #[derive(Serialize, Deserialize)]
/// #[serde(tag = "type")]
/// #[tsync]
/// enum Message {
///     Request { id: String, method: String, params: Params },
///     Response { id: String, result: Value },
/// }
/// ``` goes to `type Message = {"type": "REQUEST", "id": "...", "method": "...", "params": {...}} | {"type": "RESPONSE", "id": string, "result": "Value"}`
/// However there is an edge case: purely literal enums. These will be converted using enum syntax
/// ```ignore
/// enum Foo {
///     Bar,            // 0
///     Baz = 123,      // 123
///     Quux,           // 124
/// }
/// enum Animal {
///     Dog,
///     Cat,
/// }
/// ``` to the following
/// ```ignore
/// enum Foo {
///    Bar = 0,          
///    Baz = 123,     
///    Quux = 124,           
/// }
/// enum Animal {
///    Dog = 0,
///    Cat = 1,
/// }
/// ```
fn make_variant(
    tag_name: String,
    exported_struct: syn::ItemEnum,
    state: &mut BuildState,
    casing: Option<Case>,
) {
    state.types.push_str(&format!(
        "type {interface_name}{generics} =",
        interface_name = exported_struct.ident.to_string(),
        generics = utils::extract_struct_generics(exported_struct.generics.clone())
    ));

    for variant in exported_struct.variants {
        state.types.push('\n');
        let comments = utils::get_comments(variant.attrs);
        state.write_comments(&comments, 2);
        let field_name = if let Some(casing) = casing {
            variant.ident.to_string().to_case(casing)
        } else {
            variant.ident.to_string()
        };
        // add discriminant
        state.types.push_str(&format!(
            "  | {{\n{}{}: \"{}\",\n",
            utils::build_indentation(6),
            tag_name,
            field_name,
        ));
        super::structs::process_fields(variant.fields, state, 6);
        state.types.push_str("    }");
    }
    state.types.push_str(";\n");
}

/// This follows serde's default approach of external tagging
fn make_externally_tagged_variant(
    exported_struct: syn::ItemEnum,
    state: &mut BuildState,
    casing: Option<Case>,
) {
    state.types.push_str(&format!(
        "type {interface_name}{generics} =",
        interface_name = exported_struct.ident.to_string(),
        generics = utils::extract_struct_generics(exported_struct.generics.clone())
    ));

    for variant in exported_struct.variants {
        state.types.push('\n');
        let comments = utils::get_comments(variant.attrs);
        state.write_comments(&comments, 2);
        let field_name = if let Some(casing) = casing {
            variant.ident.to_string().to_case(casing)
        } else {
            variant.ident.to_string()
        };
        // add discriminant
        state.types.push_str(&format!(
            "  | {{\n{}\"{}\": {{",
            utils::build_indentation(6),
            field_name,
        ));
        let prepend;
        if variant.fields.len() == 0 {
            prepend = "".into();
        } else {
            prepend = utils::build_indentation(6);
            state.types.push('\n');
            super::structs::process_fields(variant.fields, state, 8);
        }
        state
            .types
            .push_str(&format!("{}}}\n{}}}", prepend, utils::build_indentation(4)));
    }
    state.types.push_str(";\n");
}

fn to_enum_case(val: impl Into<Option<String>>) -> Option<Case> {
    val.into().and_then(|x| {
        for (name, rule) in RENAME_RULES {
            if x == *name {
                return Some(*rule);
            }
        }
        None
    })
}
