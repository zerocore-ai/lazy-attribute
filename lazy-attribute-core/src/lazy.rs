use proc_macro2::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_quote, ExprClosure, Ident, ItemFn, Path, ReturnType, Token,
};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const CRATE_NAME: &str = "lazy-attribute";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

pub(super) enum LazyArgs {
    Ref,
    Map(MapArgsSyntax),
}

#[allow(dead_code)]
pub(super) struct MapArgsSyntax {
    r#type: Path,
    comma: Token![,],
    closure: MapValueSyntax,
}

enum MapValueSyntax {
    Closure(ExprClosure),
    Path(Path),
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(super) fn expand(attr_args: LazyArgs, fn_syntax: ItemFn) -> TokenStream {
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
            "Arguments are not supported on #[lazy_*] functions",
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

    let static_name = format_ident!("__lazy_static_{}", fn_name);
    let crate_name = match crate_name {
        FoundCrate::Itself => format_ident!("{}", CRATE_NAME.replace('-', "_")),
        FoundCrate::Name(name) => format_ident!("{name}"),
    };

    let (map_fn, map_fn_return_type) = match attr_args {
        LazyArgs::Ref => (quote! { (|v| v) }, quote!(#fn_return_type)),
        LazyArgs::Map(MapArgsSyntax {
            r#type,
            closure: MapValueSyntax::Closure(closure),
            ..
        }) => (quote! { (#closure) }, quote!(#r#type)),
        LazyArgs::Map(MapArgsSyntax {
            r#type,
            closure: MapValueSyntax::Path(path),
            ..
        }) => (quote! { (|r| #path(r)) }, quote!(#r#type)),
    };

    let static_gen = fn_asyncness.is_none().then_some(quote! {
        #[allow(non_upper_case_globals)]
        static #static_name: #crate_name::__internal::once_cell::sync::OnceCell<#map_fn_return_type> =
            #crate_name::__internal::once_cell::sync::OnceCell::new();
    });

    // TODO(appcypher): This part gets added anyway due to how cargo works. Need to fix: https://github.com/rust-lang/cargo/issues/8144#issuecomment-683964679
    #[cfg(feature = "async")]
    let static_gen = static_gen.or(Some(quote! {
        #[allow(non_upper_case_globals)]
        static #static_name: #crate_name::__internal::async_once_cell::OnceCell<#map_fn_return_type> =
            #crate_name::__internal::async_once_cell::OnceCell::new();
    }));

    let fn_gen = fn_asyncness.is_none().then_some(quote! {
        #(#fn_attrs)*
        #fn_vis fn #fn_name ( #fn_inputs ) -> &'static #map_fn_return_type {
            #static_name
                .get_or_init(|| {
                    let v = {
                        #(#fn_body_stmts)*
                    };
                    let f: fn(#fn_return_type) -> #map_fn_return_type = #map_fn;
                    f(v)
                })
        }
    });

    #[cfg(feature = "async")]
    let fn_gen = fn_gen.or(Some(quote! {
        #(#fn_attrs)*
        #fn_vis async fn #fn_name ( #fn_inputs ) -> &'static #map_fn_return_type {
            #static_name
            .get_or_init(async move {
                let v = {
                    #(#fn_body_stmts)*
                };
                let f: fn(#fn_return_type) -> #map_fn_return_type = #map_fn;
                f(v)
            })
            .await
        }
    }));

    quote! {
        #static_gen
        #fn_gen
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Parse for MapArgsSyntax {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            r#type: input.parse()?,
            comma: input.parse()?,
            closure: input.parse()?,
        })
    }
}

impl Parse for MapValueSyntax {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![|]) {
            Ok(Self::Closure(input.parse()?))
        } else if lookahead.peek(Ident) || lookahead.peek(Token![::]) {
            Ok(Self::Path(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}
