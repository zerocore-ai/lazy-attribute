//! This crate provides the `#[lazy_ref]` and `#[lazy_map]` attributes to lazily evaluate functions.
//!
//! This crate is not meant to be used directly. Instead, you should use the [`lazy-attribute`](https://crates.io/crates/lazy-attribute) crate.

mod lazy;

use lazy::{LazyArgs, MapArgsSyntax};
use proc_macro::TokenStream;

//--------------------------------------------------------------------------------------------------
// Attribute Procedural Macros
//--------------------------------------------------------------------------------------------------

/// Refer to [`lazy-attribute::lazy_ref`](https://docs.rs/lazy-attribute/latest/lazy_attribute/attr.lazy_ref.html) for documentation.
#[proc_macro_attribute]
pub fn lazy_ref(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_syntax = syn::parse_macro_input!(item as syn::ItemFn);
    lazy::expand(LazyArgs::Ref, fn_syntax).into()
}

/// Refer to [`lazy-attribute::lazy_map`](https://docs.rs/lazy-attribute/latest/lazy_attribute/attr.lazy_map.html) for documentation.
#[proc_macro_attribute]
pub fn lazy_map(attr: TokenStream, item: TokenStream) -> TokenStream {
    let lazy_map_args_syntax = syn::parse_macro_input!(attr as MapArgsSyntax);
    let fn_syntax = syn::parse_macro_input!(item as syn::ItemFn);
    lazy::expand(LazyArgs::Map(lazy_map_args_syntax), fn_syntax).into()
}
