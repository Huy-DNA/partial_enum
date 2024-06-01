use syn::{GenericParam, Generics, WhereClause};

pub fn extract(generics: &Generics) -> (Vec<GenericParam>, Option<WhereClause>) {
    (
        generics.iter().collect(),
        generics.where_clause,
    )
}
