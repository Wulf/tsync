use super::{utils, BuildState};
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
/// Since Rust doesn't support string enums I don't think it makes much sense to.
/// The conversion will adhere to the `serde` `tag` and `rename` attributes for the name of the tag and the case of the enum variants.
pub fn process(exported_struct: syn::ItemEnum, state: &mut BuildState, debug: bool) {
    // check we don't have any tuple structs that could mess things up.
    // if we do ignore this struct
    for variant in exported_struct.variants.iter() {
        for f in variant.fields.iter() {
            if f.ident.is_none() {
                if debug {
                    println!(
                        "#[tsync] failed for enum {}",
                        exported_struct.ident.to_string()
                    );
                }
                return;
            }
        }
    }

    state.types.push('\n');

    let comments = utils::get_comments(exported_struct.clone().attrs);
    let casing = utils::get_attribute_arg("serde", "renameAll", &exported_struct.attrs);
    let casing = to_enum_case(casing);

    let is_single = !exported_struct.variants.iter().any(|x| x.fields.len() > 0);
    state.write_comments(&comments, 0);

    if is_single {
        make_enum(exported_struct, state, casing)
    } else {
        make_variant(exported_struct, state, casing)
    }
}

fn make_enum(exported_struct: syn::ItemEnum, state: &mut BuildState, casing: Case) {
    state.types.push_str(&format!(
        "enum {interface_name} {{",
        interface_name = exported_struct.ident.to_string()
    ));

    let mut num = 0;

    for variant in exported_struct.variants {
        state.types.push('\n');
        let field_name = variant.ident.to_string().to_case(casing);
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
fn make_variant(exported_struct: syn::ItemEnum, state: &mut BuildState, casing: Case) {
    let tag_name =
        utils::get_attribute_arg("serde", "tag", &exported_struct.attrs).unwrap_or("type".into());

    state.types.push_str(&format!(
        "type {interface_name}{generics} =",
        interface_name = exported_struct.ident.to_string(),
        generics = utils::extract_struct_generics(exported_struct.generics.clone())
    ));

    for variant in exported_struct.variants {
        state.types.push('\n');
        let comments = utils::get_comments(variant.attrs);
        state.write_comments(&comments, 2);
        let field_name = variant.ident.to_string().to_case(casing);
        // add discriminant
        state.types.push_str(&format!(
            "  | {{\n{}{}: \"{}\",\n",
            utils::build_indentation(6),
            tag_name,
            field_name,
        ));
        crate::structs::process_fields(variant.fields, state, 6);
        state.types.push_str("    }");
    }
    state.types.push_str(";\n");
}

fn to_enum_case(val: impl Into<Option<String>>) -> Case {
    val.into()
        .and_then(|x| {
            for (name, rule) in RENAME_RULES {
                if x == *name {
                    return Some(*rule);
                }
            }
            None
        })
        .unwrap_or(Case::ScreamingSnake)
}
