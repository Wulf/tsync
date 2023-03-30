use crate::typescript::convert_type;
use crate::{utils, BuildState};

impl super::ToTypescript for syn::ItemStruct {
    fn convert_to_ts(self, state: &mut BuildState, _debug: bool, uses_typeinterface: bool) {
        let export = if uses_typeinterface { "" } else { "export " };

        state.types.push('\n');

        let comments = utils::get_comments(self.clone().attrs);
        state.write_comments(&comments, 0);

        state.types.push_str(&format!(
            "{export}interface {interface_name}{generics} {{\n",
            interface_name = self.ident,
            generics = utils::extract_struct_generics(self.generics.clone())
        ));
        process_fields(self.fields, state, 2);
        state.types.push('}');

        state.types.push('\n');
    }
}

pub fn process_fields(fields: syn::Fields, state: &mut BuildState, indentation_amount: i8) {
    let space = utils::build_indentation(indentation_amount);
    for field in fields {
        let comments = utils::get_comments(field.attrs);
        state.write_comments(&comments, 2);
        let field_name = field.ident.unwrap().to_string();
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
