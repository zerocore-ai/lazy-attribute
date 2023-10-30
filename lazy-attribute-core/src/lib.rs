//! This crate provides the `#[lazy]` attribute to lazily evaluate functions.
//!
//! This crate is used by the [`lazy-attribute`](https://crates.io/crates/lazy-attribute) crate and is not intended to be used directly.

mod lazy;

use proc_macro::TokenStream;

//--------------------------------------------------------------------------------------------------
// Attribute Procedural Macros
//--------------------------------------------------------------------------------------------------

/// Provides the `#[lazy]` attribute to lazily evaluate functions.
///
/// The macro expands the function into a wrapper that caches the result
/// in a static `OnceCell`.
///
/// Works for both synchronous and asynchronous
/// functions (with `async` feature).
#[proc_macro_attribute]
pub fn lazy(_: TokenStream, item: TokenStream) -> TokenStream {
    let fn_syntax = syn::parse_macro_input!(item as syn::ItemFn);
    lazy::expand(fn_syntax).into()
}
