//! Code to interop with LLVM's C API's for writing a module.

use crate::interop::llvm_sys as interop;

/// Contains pointers to objects allocated with LLVM's C API needed to create a module,
/// as well a [`llvm-model::Module`].
#[derive(Debug)]
pub struct Builder {
    target: interop::target::Target,
}
