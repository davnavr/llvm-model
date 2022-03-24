//! Code to interoperate with C APIs to obtain LLVM target information.

use crate::target;

impl target::Triple {
    /// Converts the target triple to the LLVM C API's representation for targets.
    ///
    /// # Safety
    /// This function might depend on global state, such as the initialization of targets LLVM can use by ensuring functions such
    /// as by calling [`llvm_sys::target::LLVM_InitializeAllTargets`] and [`llvm_sys::target::LLVM_InitializeAllTargetInfos`].
    pub unsafe fn to_target_ref() -> llvm_sys::target_machine::LLVMTargetRef {
        todo!("LLVMGetTargetFromTriple")
    }
}
