//! Contains implementations of traits for interoperation with the Rust bindings of the LLVM C API.

pub mod error;
pub mod target;

pub use llvm_sys as module;

pub use error::Message;

/// An error type for operations that call the LLVM C APIs that can potentially fail.
pub type Result<T> = std::result::Result<T, Message>;

/// Converts an LLVM integer boolean value to a Rust boolean.
pub const fn is_true(value: llvm_sys::prelude::LLVMBool) -> bool {
    value != 0
}