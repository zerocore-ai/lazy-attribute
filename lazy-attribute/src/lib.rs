#![warn(missing_docs)]
#![doc = include_str!("lib.md")]

//--------------------------------------------------------------------------------------------------
// Extern Crates
//--------------------------------------------------------------------------------------------------

// This is needed for the macro to work in tests: https://github.com/bkchr/proc-macro-crate/issues/10#issuecomment-826382619
extern crate self as lazy_attribute;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

#[doc = include_str!("lazy_ref.md")]
pub use lazy_attribute_core::lazy_ref;

#[doc = include_str!("lazy_map.md")]
pub use lazy_attribute_core::lazy_map;

//--------------------------------------------------------------------------------------------------
// Re-export Modules
//--------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod __internal {
    #[cfg(feature = "async")]
    pub use async_once_cell;
    pub use once_cell;
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    #[test]
    fn lazy() {
        let t = trybuild::TestCases::new();

        t.pass("test/01-correct-func.rs");
        #[cfg(feature = "async")]
        t.pass("test/02-correct-async-func.rs");
        t.pass("test/03-correct-map.rs");
        t.compile_fail("test/04-unsupported-args.rs");
        t.compile_fail("test/05-unsupported-map-attr.rs")
    }
}
