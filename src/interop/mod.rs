//! Contains code for interoperation with other LLVM libraries. Most of these modules don't actually contain functions, but
//! instead contain implementations of traits and `impl` blocks.

#[cfg(feature = "inkwell_interop")]
pub mod inkwell;

#[cfg(feature = "llvm_sys_interop")]
pub mod llvm_sys;
