//! Code to interop with LLVM's C API's for writing a module.

use crate::interop::llvm_sys as interop;
use crate::Identifier;

/// Contains pointers to objects allocated with LLVM's C API needed to create a module,
/// as well a [`llvm-model::Module`].
#[derive(Debug)]
pub struct Builder<'t> {
    target: &'t interop::target::Target,
    module: crate::Module<'t>,
}

impl<'t> Builder<'t> {
    /// Creates a module with the specified name and target.
    pub fn new(name: Identifier, target: &'t interop::target::Target) -> Self {
        Self {
            target,
            module: crate::Module::new(name, target.target()),
        }
    }

    /// The target machine and layout of the module.
    pub fn target(&self) -> &'t interop::target::Target {
        self.target
    }

    /// Used to mutate the contents of the module.
    pub fn module(&mut self) -> &mut crate::Module<'t> {
        &mut self.module
    }
}
