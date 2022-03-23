//! Library for writing LLVM IR.
//!
//! Useful links:
//! - [Latest API documentation for `llvm-sys`](https://docs.rs/llvm-sys/latest/llvm_sys/)
//! - [LLVM language reference](https://llvm.org/docs/LangRef.html)

#![deny(missing_docs, missing_debug_implementations)]

pub mod identifier;
pub mod module;
pub mod target;

pub use module::Module;
pub use target::Target;

/// Internal helper used to define a default value for enumerations.
#[doc(hidden)]
#[macro_export]
macro_rules! enum_default {
    ($enum_type: ty, $enum_case: ident) => {
        impl std::default::Default for $enum_type {
            fn default() -> Self {
                Self::$enum_case
            }
        }
    };
}
