use crate::typescript::convert_type;
use crate::{utils, BuildState};
use convert_case::{Case, Casing};
use syn::__private::ToTokens;

/// Conversion of Rust Enum to Typescript using external tagging as per https://serde.rs/enum-representations.html
/// however conversion will adhere to the `serde` `tag` such that enums are intenrally tagged
/// (while the other forms such as adjacent tagging aren't supported).
/// `rename_all` attributes for the name of the tag will also be adhered to.
impl super::ToTypescript for syn::ItemEnum {
    fn convert_to_ts(self, state: &mut BuildState, uses_type_interface: bool) {
        // check we don't have any tuple structs that could mess things up.
        // if we do ignore this struct
        for variant in self.variants.iter() {
            // allow single-field tuple structs to pass through as newtype structs
            let mut is_newtype = false;
            for f in variant.fields.iter() {
                if f.ident.is_none() {
                    // If we already marked this variant as a newtype, we have a multi-field tuple struct
                    if is_newtype {
                        if crate::DEBUG.try_get().is_some_and(|d| *d) {
                            println!("#[tsync] failed for enum {}", self.ident);
                        }
                        return;
                    } else {
                        is_newtype = true;
                    }
                }
            }
        }

        state.types.push('\n');

        let comments = utils::get_comments(self.clone().attrs);
        let casing = utils::get_attribute_arg("serde", "rename_all", &self.attrs);
        let casing = utils::parse_serde_case(casing);

        let is_single = !self.variants.iter().any(|x| !x.fields.is_empty());
        state.write_comments(&comments, 0);

        if is_single {
            if utils::has_attribute_arg("derive", "Serialize_repr", &self.attrs) {
                add_numeric_enum(self, state, casing, uses_type_interface)
            } else {
                add_enum(self, state, casing, uses_type_interface)
            }
        } else if let Some(tag_name) = utils::get_attribute_arg("serde", "tag", &self.attrs) {
            add_internally_tagged_enum(tag_name, self, state, casing, uses_type_interface)
        } else {
            add_externally_tagged_enum(self, state, casing, uses_type_interface)
        }
    }
}

/// This convert an all unit enums to a union of const strings in Typescript.
/// It will ignore any discriminants.
fn add_enum(
    exported_struct: syn::ItemEnum,
    state: &mut BuildState,
    casing: Option<Case>,
    uses_type_interface: bool,
) {
    let export = if uses_type_interface { "" } else { "export " };
    state.types.push_str(&format!(
        "{export}type {interface_name} =\n{space}",
        interface_name = exported_struct.ident,
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
fn add_numeric_enum(
    exported_struct: syn::ItemEnum,
    state: &mut BuildState,
    casing: Option<Case>,
    uses_type_interface: bool,
) {
    let declare = if uses_type_interface { "declare " } else { "export " };
    state.types.push_str(&format!(
        "{declare}enum {interface_name} {{",
        interface_name = exported_struct.ident
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
fn add_internally_tagged_enum(
    tag_name: String,
    exported_struct: syn::ItemEnum,
    state: &mut BuildState,
    casing: Option<Case>,
    uses_type_interface: bool,
) {
    let export = if uses_type_interface { "" } else { "export " };
    state.types.push_str(&format!(
        "{export}type {interface_name}{generics} =",
        interface_name = exported_struct.ident,
        generics = utils::extract_struct_generics(exported_struct.generics.clone())
    ));

    for variant in exported_struct.variants.iter() {
        // Assumes that non-newtype tuple variants have already been filtered out
        if variant.fields.iter().any(|v| v.ident.is_none()) {
            // TODO: Generate newtype structure
            // This should contain the discriminant plus all fields of the inner structure as a flat structure
            // TODO: Check for case where discriminant name matches an inner structure field name
            // We should reject clashes
        } else {
            state.types.push('\n');
            state.types.push_str(&format!(
                "  | {interface_name}__{variant_name}",
                interface_name = exported_struct.ident,
                variant_name = variant.ident,
            ))
        }
    }

    state.types.push_str(";\n");

    for variant in exported_struct.variants {
        // Assumes that non-newtype tuple variants have already been filtered out
        if !variant.fields.iter().any(|v| v.ident.is_none()) {
            state.types.push('\n');
            let comments = utils::get_comments(variant.attrs);
            state.write_comments(&comments, 0);
            state.types.push_str(&format!(
                "type {interface_name}__{variant_name} = ",
                interface_name = exported_struct.ident,
                variant_name = variant.ident,
            ));

            let field_name = if let Some(casing) = casing {
                variant.ident.to_string().to_case(casing)
            } else {
                variant.ident.to_string()
            };
            // add discriminant
            state.types.push_str(&format!(
                "{{\n{}{}: \"{}\";\n",
                utils::build_indentation(2),
                tag_name,
                field_name,
            ));
            super::structs::process_fields(variant.fields, state, 2, casing);
            state.types.push_str("};");
        }
    }
    state.types.push('\n');
}

/// This follows serde's default approach of external tagging
fn add_externally_tagged_enum(
    exported_struct: syn::ItemEnum,
    state: &mut BuildState,
    casing: Option<Case>,
    uses_type_interface: bool,
) {
    let export = if uses_type_interface { "" } else { "export " };
    state.types.push_str(&format!(
        "{export}type {interface_name}{generics} =",
        interface_name = exported_struct.ident,
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
        // Assumes that non-newtype tuple variants have already been filtered out
        let is_newtype = variant.fields.iter().any(|v| v.ident.is_none());

        if is_newtype {
            // add discriminant
            state.types.push_str(&format!("  | {{ \"{}\":", field_name));
            for field in variant.fields {
                state.types.push_str(&format!(" {}", convert_type(&field.ty).ts_type,));
            }
            state.types.push_str(" }");
        } else {
            // add discriminant
            state.types.push_str(&format!(
                "  | {{\n{}\"{}\": {{",
                utils::build_indentation(6),
                field_name,
            ));
            let prepend;
            if variant.fields.is_empty() {
                prepend = "".into();
            } else {
                prepend = utils::build_indentation(6);
                state.types.push('\n');
                super::structs::process_fields(variant.fields, state, 8, casing);
            }
            state
                .types
                .push_str(&format!("{}}}\n{}}}", prepend, utils::build_indentation(4)));
        }
    }
    state.types.push_str(";\n");
}
