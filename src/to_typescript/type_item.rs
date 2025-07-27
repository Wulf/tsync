use crate::BuildState;
use syn::ext::IdentExt;

impl super::ToTypescript for syn::ItemType {
    fn convert_to_ts(self, state: &mut BuildState, config: &crate::BuildSettings) {
        let export = if config.uses_type_interface { "" } else { "export " };
        state.types.push('\n');
        let name = self.ident.unraw().to_string();
        let ty = crate::typescript::convert_type(&self.ty);
        let comments = crate::utils::get_comments(self.attrs);
        state.write_comments(&comments, 0);
        state
            .types
            .push_str(format!("{export}type {name} = {ty}", name = name, ty = ty.ts_type).as_str());

        state.types.push('\n');
    }
}
