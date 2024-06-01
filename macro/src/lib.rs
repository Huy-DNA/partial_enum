use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn partial(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    annotated_item
}
