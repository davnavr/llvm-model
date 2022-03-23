//! Contains code for converting from `llvm-model`'s representation of LLVM targets to `inkwell`'s representations.

use crate::interop::inkwell::InkwellResult;
use crate::target::Triple;

pub use inkwell::targets::{Target as InkwellTarget, TargetTriple as InkwellTargetTriple};

impl From<&'_ Triple> for InkwellTargetTriple {
    fn from(triple: &Triple) -> Self {
        Self::create(&triple.to_string())
    }
}

impl Triple {
    /// Retrieves an inkwell Target structure corresponding to this target triple.
    ///
    /// # Caution
    /// Before calling, ensure that targets have been initialized beforehand, such as by calling [`inkwell::targets::Target::initialize_all()`],
    /// otherwise an error may be returned.
    pub fn to_inkwell_target(&self) -> InkwellResult<InkwellTarget> {
        InkwellTarget::from_triple(&self.into())
    }
}
