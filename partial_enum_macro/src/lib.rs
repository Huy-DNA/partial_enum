mod extractors;
mod constructors;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn partial(args: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let ItemEnum { attrs: enum_attrs, variants, vis: enum_vis, ident: enum_ident, generics: enum_generics, .. } = parse_macro_input!(annotated_item as ItemEnum);

    let enum_generics = extractors::generics::extract(&enum_generics);
    let variants = variants.iter().collect();
    let enum_structs = extractors::structs::extract(variants, &enum_generics);

    panic!("Not implemented")
}
