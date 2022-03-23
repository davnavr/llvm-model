//! Contains code for interoperation with other LLVM libraries.
//! Most of these modules don't actually contain functions, but instead contain implementations of traits and `impl` blocks.

#[cfg(feature = "inkwell_interop")]
pub mod inkwell;
