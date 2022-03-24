//! Contains implementations of traits for interoperation with the Rust bindings of the LLVM C API.

pub mod error;
pub mod target;

pub use llvm_sys as module;
