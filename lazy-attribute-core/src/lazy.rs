use proc_macro2::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use syn::{parse_quote, ItemFn, ReturnType};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const CRATE_NAME: &str = "lazy-attribute";

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(super) fn expand(fn_syntax: ItemFn) -> TokenStream {
    let fn_name = &fn_syntax.sig.ident;
    let fn_attrs = &fn_syntax.attrs;
    let fn_vis = &fn_syntax.vis;
    let fn_inputs = &fn_syntax.sig.inputs;
    let fn_return_type = match fn_syntax.sig.output {
        ReturnType::Type(_, fn_return_type) => *fn_return_type,
        ReturnType::Default => parse_quote! { () },
    };
    let fn_body_stmts = &fn_syntax.block.stmts;
    let fn_asyncness = fn_syntax.sig.asyncness;

    if !fn_inputs.is_empty() {
        return syn::Error::new_spanned(
            fn_inputs,
            "Arguments are not supported on #[lazy] functions",
        )
        .to_compile_error();
    }

    #[cfg(not(feature = "async"))]
    if fn_asyncness.is_some() {
        return syn::Error::new_spanned(
            fn_asyncness,
            "Async functions are only supported when the `async` feature is enabled",
        )
        .to_compile_error();
    }

    let Ok(crate_name) = crate_name(CRATE_NAME) else {
        return syn::Error::new_spanned(
            fn_name,
            format!("Could not find `{CRATE_NAME}` crate in your `Cargo.toml`"),
        )
        .to_compile_error();
    };

    let crate_name = match crate_name {
        FoundCrate::Itself => format_ident!("crate"),
        FoundCrate::Name(name) => format_ident!("{name}"),
    };

    let static_name = format_ident!("__lazy_static_{}", fn_name);
    let static_gen = {
        #[cfg(feature = "async")]
        if fn_asyncness.is_some() {
            quote! {
                #[allow(non_upper_case_globals)]
                static #static_name: #crate_name::__internal::async_once_cell::OnceCell<#fn_return_type> =
                    #crate_name::__internal::async_once_cell::OnceCell::new();
            }
        } else {
            quote! {
                #[allow(non_upper_case_globals)]
                static #static_name: #crate_name::__internal::once_cell::sync::OnceCell<#fn_return_type> =
                    #crate_name::__internal::once_cell::sync::OnceCell::new();
            }
        }
        #[cfg(not(feature = "async"))]
        quote! {
            #[allow(non_upper_case_globals)]
            static #static_name: #crate_name::__internal::once_cell::sync::OnceCell<#fn_return_type> =
                #crate_name::__internal::once_cell::sync::OnceCell::new();
        }
    };

    let fn_gen = {
        #[cfg(feature = "async")]
        if fn_asyncness.is_some() {
            quote! {
                #(#fn_attrs)*
                #fn_vis async fn #fn_name ( #fn_inputs ) -> &'static #fn_return_type {
                    #static_name
                        .get_or_init(async move {
                            #(#fn_body_stmts)*
                        }).await
                }
            }
        } else {
            quote! {
                #(#fn_attrs)*
                #fn_vis fn #fn_name ( #fn_inputs ) -> &'static #fn_return_type {
                    #static_name
                        .get_or_init(|| {
                            #(#fn_body_stmts)*
                        })
                }
            }
        }
        #[cfg(not(feature = "async"))]
        quote! {
            #(#fn_attrs)*
            #fn_vis fn #fn_name ( #fn_inputs ) -> &'static #fn_return_type {
                #static_name
                    .get_or_init(|| {
                        #(#fn_body_stmts)*
                    })
            }
        }
    };

    quote! {
        #static_gen
        #fn_gen
    }
}
