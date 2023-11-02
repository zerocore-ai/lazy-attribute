//! This crate provides the `#[lazy_ref]` attribute to lazily evaluate functions.
//!
//! This crate is not meant to be used directly. Instead, you should use the [`lazy-attribute`](https://crates.io/crates/lazy-attribute) crate.

mod lazy;

use proc_macro::TokenStream;

//--------------------------------------------------------------------------------------------------
// Attribute Procedural Macros
//--------------------------------------------------------------------------------------------------

#[proc_macro_attribute]
pub fn lazy_ref(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_syntax = syn::parse_macro_input!(item as syn::ItemFn);
    lazy::expand(fn_syntax).into()
}
