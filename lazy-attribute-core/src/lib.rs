//! This crate provides the `#[lazy]` attribute to lazily evaluate functions.
//!
//! This crate is used by the [`lazy-attribute`](https://crates.io/crates/lazy-attribute) crate and is not intended to be used directly.

mod lazy;

use lazy::{LazyArgs, MapArgsSyntax};
use proc_macro::TokenStream;

//--------------------------------------------------------------------------------------------------
// Attribute Procedural Macros
//--------------------------------------------------------------------------------------------------

#[proc_macro_attribute]
pub fn lazy_ref(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_syntax = syn::parse_macro_input!(item as syn::ItemFn);
    lazy::expand(LazyArgs::Ref, fn_syntax).into()
}

#[proc_macro_attribute]
pub fn lazy_map(attr: TokenStream, item: TokenStream) -> TokenStream {
    let lazy_map_args_syntax = syn::parse_macro_input!(attr as MapArgsSyntax);
    let fn_syntax = syn::parse_macro_input!(item as syn::ItemFn);
    lazy::expand(LazyArgs::Map(lazy_map_args_syntax), fn_syntax).into()
}
