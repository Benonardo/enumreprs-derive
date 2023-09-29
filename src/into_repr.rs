use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{quote, TokenStreamExt};
use syn::{Data, DataEnum, DeriveInput, Fields};

use crate::util::{get_discriminants, get_repr, is_fieldless};

pub fn into_repr_derive_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let Data::Enum(ref data) = ast.data else {
        abort_call_site!("Not an enum");
    };
    let name = &ast.ident;
    let repr = get_repr(&ast);
    let body = match check_fieldless(data, &repr) {
        Some(stream) => stream,
        None => match_body(data),
    };
    let copy_bound = if is_fieldless(data) {
        quote!(where #name: Copy)
    } else {
        TokenStream::new()
    };

    quote! {
        impl enumreprs::IntoRepr<#repr> for #name
        #copy_bound {
            fn into_repr(&self) -> #repr {
                #body
            }
        }
    }
    .into()
}

fn check_fieldless(data: &DataEnum, repr: &TokenStream) -> Option<TokenStream> {
    if is_fieldless(data) {
        Some(quote!(*self as #repr))
    } else {
        None
    }
}

fn match_body(data: &DataEnum) -> TokenStream {
    let discriminants = get_discriminants(data);
    let match_arms = {
        let mut match_arms: TokenStream = TokenStream::new();
        for (ident, expr, fields) in discriminants {
            match_arms.append_all(match fields {
                Fields::Unit => quote!(&Self::#ident => #expr,),
                Fields::Unnamed(_) => quote!(&Self::#ident(..) => #expr,),
                Fields::Named(_) => quote!(&Self::#ident {..} => #expr,),
            });
        }
        match_arms
    };

    quote! {
        match self {
            #match_arms
        }
    }
}
