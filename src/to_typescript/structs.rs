use crate::typescript::convert_type;
use crate::{utils, BuildState};
use convert_case::{Case, Casing};

impl super::ToTypescript for syn::ItemStruct {
    fn convert_to_ts(self, state: &mut BuildState, config: &crate::BuildSettings) {
        let export = if config.uses_type_interface {
            ""
        } else {
            "export "
        };
        let casing = utils::get_attribute_arg("serde", "rename_all", &self.attrs);
        let casing = utils::parse_serde_case(casing);
        state.types.push('\n');

        let comments = utils::get_comments(self.clone().attrs);
        state.write_comments(&comments, 0);

        let intersections = get_intersections(&self.fields);

        match (
            intersections,
            matches!(self.fields, syn::Fields::Unnamed(_)),
        ) {
            (Some(intersections), false) => {
                state.types.push_str(&format!(
                    "{export}type {struct_name}{generics} = {intersections} & ",
                    export = export,
                    struct_name = self.ident,
                    generics = utils::extract_struct_generics(self.generics.clone()),
                    intersections = intersections
                ));
            }
            (None, false) => {
                state.types.push_str(&format!(
                    "{export}interface {interface_name}{generics} ",
                    interface_name = self.ident,
                    generics = utils::extract_struct_generics(self.generics.clone())
                ));
            }
            (None, true) => {
                state.types.push_str(&format!(
                    "{export}type {struct_name}{generics} = ",
                    export = export,
                    struct_name = self.ident,
                    generics = utils::extract_struct_generics(self.generics.clone()),
                ));
            }
            (Some(_), true) => {
                if crate::DEBUG.try_get().is_some_and(|d| *d) {
                    println!(
                        "#[tsync] failed for struct {}. cannot flatten fields of tuple struct",
                        self.ident
                    );
                }
                return;
            }
        }

        if let syn::Fields::Unnamed(unnamed) = self.fields {
            process_tuple_fields(unnamed, state);
        } else {
            state.types.push_str("{\n");
            process_fields(self.fields, state, 2, casing);
            state.types.push('}');
        }

        state.types.push('\n');
    }
}

pub fn process_fields(
    fields: syn::Fields,
    state: &mut BuildState,
    indentation_amount: i8,
    case: impl Into<Option<Case>>,
) {
    let space = utils::build_indentation(indentation_amount);
    let case = case.into();
    for field in fields {
        debug_assert!(
            field.ident.is_some(),
            "struct fields should have names, found unnamed field"
        );

        // Check if the field has the serde flatten attribute, if so, skip it
        let has_flatten_attr = utils::get_attribute_arg("serde", "flatten", &field.attrs).is_some();
        if has_flatten_attr {
            continue;
        }

        let comments = utils::get_comments(field.attrs);

        state.write_comments(&comments, 2);
        let field_name = if let Some(name_case) = case {
            field
                .ident
                .map(|id| id.to_string().to_case(name_case))
                .unwrap()
        } else {
            field.ident.map(|i| i.to_string()).unwrap()
        };

        let field_type = convert_type(&field.ty);
        state.types.push_str(&format!(
            "{space}{field_name}{optional_parameter_token}: {field_type};\n",
            space = space,
            field_name = field_name,
            optional_parameter_token = if field_type.is_optional { "?" } else { "" },
            field_type = field_type.ts_type
        ));
    }
}

/// Process tuple fields
///
/// NOTE: Currently, this function does not handle comments or attributes on tuple fields.
///
/// # Example
///
/// ```ignore
/// struct Todo(String, u32);
/// ```
///
/// should become
///
/// ```ignore
/// type Todo = [string, number];
/// ```
pub fn process_tuple_fields(fields: syn::FieldsUnnamed, state: &mut BuildState) {
    let out = fields
        .unnamed
        .into_iter()
        .map(|field| {
            let field_type = convert_type(&field.ty);
            field_type.ts_type
        })
        .collect::<Vec<String>>();

    if out.len() == 1 {
        state.types.push_str(&out[0].to_string());
    } else if !out.is_empty() {
        state.types.push_str(&format!("[ {} ]", out.join(", ")));
    }
}

fn get_intersections(fields: &syn::Fields) -> Option<String> {
    let mut types = Vec::new();

    for field in fields {
        let has_flatten_attr = utils::get_attribute_arg("serde", "flatten", &field.attrs).is_some();
        let field_type = convert_type(&field.ty);
        if has_flatten_attr {
            types.push(field_type.ts_type);
        }
    }

    if types.is_empty() {
        return None;
    }

    Some(types.join(" & "))
}
