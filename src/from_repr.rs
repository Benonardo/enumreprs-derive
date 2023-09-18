use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{Data, DeriveInput};

use crate::util::{get_discriminants, get_repr, is_fieldless};

pub fn from_repr_derive_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let Data::Enum(ref data) = ast.data else {
        panic!("Not an enum");
    };
    if !is_fieldless(data) {
        panic!("Enum is not fieldless")
    }
    let name = &ast.ident;
    let repr = get_repr(&ast);
    let discriminants = get_discriminants(data);
    let match_arms = {
        let mut match_arms = TokenStream::new();
        for (ident, expr, _) in discriminants {
            match_arms.append_all(quote!(#expr => std::result::Result::Ok(Self::#ident),));
        }
        match_arms
    };

    quote! {
        impl enumreprs::FromRepr<#repr> for #name {
            fn from_repr(repr: #repr) -> std::result::Result<Self, enumreprs::FromReprError<#repr>> {
                match repr {
                    #match_arms
                    other => std::result::Result::Err(enumreprs::FromReprError::InvalidVariant(other))
                }
            }
        }
    }
    .into()
}
