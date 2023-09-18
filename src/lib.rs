mod from_repr;
mod into_repr;
mod util;

use proc_macro::TokenStream;

use from_repr::from_repr_derive_impl;
use into_repr::into_repr_derive_impl;

#[proc_macro_derive(IntoRepr)]
pub fn into_repr_derive(input: TokenStream) -> TokenStream {
    into_repr_derive_impl(input)
}

#[proc_macro_derive(FromRepr)]
pub fn from_repr_derive(input: TokenStream) -> TokenStream {
    from_repr_derive_impl(input)
}
