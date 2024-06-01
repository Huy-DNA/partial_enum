use syn::{GenericParam, ItemStruct, Token, Variant, Visibility, WhereClause};

use crate::constructors;

fn extract_structs(variants: Vec<&Variant>, generics: (Vec<GenericParam>, Option<WhereClause>)) -> Vec<ItemStruct> {
    variants.iter().map(|variant| {
        if variant.discriminant.is_some() {
            compile_error!("A variant can not be assigned a number");
        }
        
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Public(Token![pub]),
            struct_token: Token![struct],
            ident: variant.ident,
            generics: constructors::generics::construct(generics),
            fields: variant.fields,
            semi_token: None,
        } 
    })
}
