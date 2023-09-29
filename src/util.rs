use proc_macro2::{Ident, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use syn::{DataEnum, DeriveInput, Expr, Fields, Meta};

pub fn get_repr(ast: &DeriveInput) -> TokenStream {
    let mut repr: Option<TokenStream> = None;
    for ref attr in &ast.attrs {
        let Meta::List(ref list) = attr.meta else {
            continue;
        };
        let Some(ident) = list.path.get_ident() else {
            continue;
        };

        if ident == "repr" {
            repr = Some(list.tokens.clone());
        }
    }

    match repr {
        Some(repr) => repr,
        None => abort_call_site!("No repr attribute"),
    }
}

pub fn is_fieldless(data: &DataEnum) -> bool {
    let mut fieldless = true;
    for variant in &data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            fieldless = false;
        }
    }
    fieldless
}

pub fn assert_fieldless(data: &DataEnum) -> bool {
    let mut fieldless = true;
    for variant in &data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            abort!(variant, "Variant is not fieldless");
        }
    }
    fieldless
}

pub fn get_discriminants(data: &DataEnum) -> Vec<(&Ident, &Expr, &Fields)> {
    let mut discriminants: Vec<(&Ident, &Expr, &Fields)> = Vec::new();
    for variant in &data.variants {
        let ident = &variant.ident;
        let Some((_, expr)) = &variant.discriminant else {
            abort!(variant, "Variant has no explicit discriminant");
        };
        let fields = &variant.fields;
        discriminants.push((ident, expr, fields));
    }
    discriminants
}
