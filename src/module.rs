//! LLVM modules contain the code and data of a program.
//!
//! [See the LLVM documentation on modules](https://llvm.org/docs/LangRef.html#module-structure).

/// An LLVM module, containing global values and their symbols.
pub struct Module {}

impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Module").finish()
    }
}
