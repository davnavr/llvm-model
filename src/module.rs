//! LLVM modules contain the code and data of a program.
//!
//! [See the LLVM documentation on modules](https://llvm.org/docs/LangRef.html#module-structure).

use crate::identifier;

/// An LLVM module, containing global values and their symbols.
pub struct Module {
    name: identifier::Identifier,
    //target_triple:
}

impl Module {
    /// Creates a new module with the specified name.
    pub fn with_name(name: identifier::Identifier) -> Self {
        Self { name }
    }

    /// Retrieves the name of the module.
    pub fn name(&self) -> &identifier::Id {
        self.name.as_ref()
    }
}

impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Module").field("name", &self.name).finish()
    }
}
