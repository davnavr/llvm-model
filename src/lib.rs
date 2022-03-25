//! Library for writing LLVM IR.
//!
//! Useful links:
//! - [Latest API documentation for `llvm-sys`](https://docs.rs/llvm-sys/latest/llvm_sys/)
//! - [LLVM language reference](https://llvm.org/docs/LangRef.html)

#![deny(missing_docs, missing_debug_implementations)]

pub mod identifier;
pub mod interop;
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

/// Internal helper that defines a `From` trait implementation for an enum case.
#[doc(hidden)]
#[macro_export]
macro_rules! enum_case_from {
    ($enum_type: ty, $enum_case: ident, $case_type: ty) => {
        impl std::convert::From<$case_type> for $enum_type {
            fn from(value: $case_type) -> Self {
                Self::$enum_case(value)
            }
        }
    };
}
