use syn::{GenericParam, Generics, Token, punctuated::Punctuated};

pub fn constructor((generic_params, where_clause): (Vec<GenericParam>, Option<WhereClause>)) -> Generics {
    let params = Punctuated::new();
    for param in generic_params {
        params.push(param);
    }

    Generics {
        lt_token: Token![<],
        params,
        gt_token: Token![>],
        where_clause,
    }
}
