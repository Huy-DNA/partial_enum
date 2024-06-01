use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum};

#[proc_macro_attribute]
pub fn partial(args: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let parsed_enum = parse_macro_input!(annotated_item as ItemEnum);

    panic!("Not implemented")
}
