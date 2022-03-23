//! LLVM modules contain the code and data of a program.
//!
//! [See the LLVM documentation on modules](https://llvm.org/docs/LangRef.html#module-structure).

use crate::identifier::{Id, Identifier};
use crate::target;

/// An LLVM module, containing global values and their symbols.
pub struct Module {
    name: Identifier,
    target: target::Target,
}

impl Module {
    /// Creates a new module with the specified name and target.
    pub fn new(name: Identifier, target: target::Target) -> Self {
        Self { name, target }
    }

    /// Retrieves the name of the module.
    pub fn name(&self) -> &Id {
        self.name.as_id()
    }

    /// Gets a value to describe the target machine and target layout for this module.
    pub fn target(&self) -> &target::Target {
        &self.target
    }

    /// Gets the target machine for this module.
    pub fn target_machine(&self) -> &target::Machine {
        self.target.machine()
    }

    /// Gets the target triple of this module.
    pub fn target_triple(&self) -> &target::Triple {
        self.target.triple()
    }

    /// Gets the target layout used by this module.
    pub fn target_layout(&self) -> &target::Layout {
        self.target.layout()
    }
}

impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Module")
            .field("name", &self.name)
            .field("target", &self.target)
            .finish()
    }
}
