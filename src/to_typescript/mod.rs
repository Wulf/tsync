pub mod consts;
pub mod enums;
pub mod structs;
pub mod type_item;

pub trait ToTypescript {
    fn convert_to_ts(self, state: &mut crate::BuildState, debug: bool);
}
