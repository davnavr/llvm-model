//! Contains implementations of traits for interoperation with [`inkwell`](https://crates.io/crates/inkwell).

pub mod target;

/// Common result type used in `inkwell` functions, indicating an error with an LLVM allocated message.
pub type InkwellResult<T> = Result<T, inkwell::support::LLVMString>;
