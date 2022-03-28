//! Contains implementations of traits for interoperation with the Rust bindings of the LLVM C API.
//!
//! # Useful links:
//! - [Latest API documentation for `llvm-sys`](https://docs.rs/llvm-sys/latest/llvm_sys/)
//! - [Documentation for the LLVM C API](https://llvm.org/doxygen/group__LLVMC.html)

pub mod buffer;
pub mod message;
pub mod module;
pub mod target;

pub use buffer::MemoryBuffer;
pub use message::Message;
pub use module::Builder as ModuleBuilder;

/// An error type for operations that call the LLVM C APIs that can potentially fail.
pub type Result<T> = std::result::Result<T, Message>;
