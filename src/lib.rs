//! Library for writing LLVM IR.
//!
//! Useful links:
//! - [Latest API documentation for `llvm-sys`](https://docs.rs/llvm-sys/latest/llvm_sys/)
//! - [LLVM language reference](https://llvm.org/docs/LangRef.html)

#![deny(missing_docs, missing_debug_implementations)]

pub mod identifier;
pub mod module;

pub use module::Module;
