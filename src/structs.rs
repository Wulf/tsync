use super::{utils, BuildState};
use crate::typescript::convert_type;

pub fn process(exported_struct: syn::ItemStruct, state: &mut BuildState, _debug: bool) {
    state.types.push('\n');

    let comments = utils::get_comments(exported_struct.clone().attrs);
    state.write_comments(&comments, 0);

    state.types.push_str(&format!(
        "interface {interface_name}{generics} {{\n",
        interface_name = exported_struct.clone().ident.to_string(),
        generics = utils::extract_struct_generics(exported_struct.generics.clone())
    ));
    process_fields(exported_struct.fields, state, 2);
    state.types.push_str("}");

    state.types.push('\n');
}

pub fn process_fields(fields: syn::Fields, state: &mut BuildState, indentation_amount: i8) {
    let space = utils::build_indentation(indentation_amount);
    for field in fields {
        let comments = utils::get_comments(field.attrs);
        state.write_comments(&comments, 2);
        let field_name = field.ident.unwrap().to_string();
        let field_type = convert_type(&field.ty);
        state.types.push_str(&format!(
            "{space}{field_name}{optional_parameter_token}: {field_type}\n",
            space = space,
            field_name = field_name,
            optional_parameter_token = if field_type.is_optional { "?" } else { "" },
            field_type = field_type.ts_type
        ));
    }
}
