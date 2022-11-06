extern crate proc_macro;

use proc_macro::TokenStream;

// document this attribute
#[proc_macro_attribute]
pub fn tsync(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
