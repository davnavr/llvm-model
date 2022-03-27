//! Library for writing LLVM IR.
//!
//! Useful links:
//! - [Latest API documentation for `llvm-sys`](https://docs.rs/llvm-sys/latest/llvm_sys/)
//! - [LLVM language reference](https://llvm.org/docs/LangRef.html)

#![deny(missing_docs, missing_debug_implementations)]

pub mod global;
pub mod identifier;
pub mod interop;
pub mod module;
pub mod target;
pub mod types;

pub use identifier::{Id, Identifier};
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

/// Internal helper used to define public getter and setter functions for a [`std::cell::Cell`].
#[doc(hidden)]
#[macro_export]
macro_rules! cell_get_set {
    ($field: ident, $field_type: ty, $getter_desc: literal, $getter_name: ident, $setter_desc: literal, $setter_name: ident) => {
        #[doc = $getter_desc]
        pub fn $getter_name(&self) -> $field_type {
            std::cell::Cell::<$field_type>::get(&self.$field)
        }

        #[doc = $setter_desc]
        pub fn $setter_name(&self, value: $field_type) {
            std::cell::Cell::<$field_type>::set(&self.$field, value)
        }
    };
}
