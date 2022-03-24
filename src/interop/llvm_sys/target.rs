//! Code to interoperate with C APIs to obtain LLVM target information.

use crate::identifier::Id;
use crate::interop::llvm_sys as interop;
use crate::target;
use std::ptr;

pub use llvm_sys::target_machine::LLVMTargetRef;

/// Converts a target triple string to the LLVM C API's representation for targets.
///
/// # Safety
/// This function might depend on global state, such as the initialization of targets LLVM can use by ensuring functions such
/// as by calling [`llvm_sys::target::LLVM_InitializeAllTargets`] and [`llvm_sys::target::LLVM_InitializeAllTargetInfos`].
pub unsafe fn identifier_to_target_ref(triple: &Id) -> interop::Result<LLVMTargetRef> {
    let mut error = ptr::null_mut();
    let mut target = ptr::null_mut();
    if interop::is_true(llvm_sys::target_machine::LLVMGetTargetFromTriple(
        triple.to_c_string().as_ptr(),
        &mut target as *mut LLVMTargetRef,
        &mut error as *mut *mut _,
    )) {
        Ok(target)
    } else {
        Err(interop::Message::from_ptr(error))
    }
}

/// Gets a target triple corresponding to the host's machine.
pub unsafe fn host_target_triple() -> target::Triple {
    interop::Message::from_ptr(llvm_sys::target_machine::LLVMGetDefaultTargetTriple())
        .to_identifier()
        .into()
}

impl target::KnownTriple {
    /// Converts this target triple into the LLVM C API's represention for a target.
    pub unsafe fn to_target_ref(&self) -> interop::Result<LLVMTargetRef> {
        identifier_to_target_ref(self.to_triple_string().as_id())
    }
}
