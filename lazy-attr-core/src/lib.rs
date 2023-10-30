#![warn(missing_docs)]
#![doc = include_str!("lib.md")]

mod lazy;

use proc_macro::TokenStream;

//--------------------------------------------------------------------------------------------------
// Attribute Procedural Macros
//--------------------------------------------------------------------------------------------------

/// TODO(appcypher): document
#[proc_macro_attribute]
pub fn lazy(_: TokenStream, item: TokenStream) -> TokenStream {
    let fn_syntax = syn::parse_macro_input!(item as syn::ItemFn);
    lazy::expand(fn_syntax).into()
}

//--------------------------------------------------------------------------------------------------
// Test
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    #[test]
    fn lazy() {
        let t = trybuild::TestCases::new();

        t.pass("test/01-correct-func.rs");

        #[cfg(feature = "async")]
        t.pass("test/02-correct-async-func.rs");

        t.compile_fail("test/03-unsupported-args.rs");
    }
}
