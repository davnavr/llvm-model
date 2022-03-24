//! Code to interoperate with C APIs to obtain LLVM target information.

use crate::identifier::Id;
use crate::interop::llvm_sys as interop;
use crate::target;
use std::ptr;

pub use llvm_sys::target_machine::{LLVMCodeGenOptLevel, LLVMCodeModel, LLVMRelocMode, LLVMTargetRef};

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
///
/// # Safety
/// Depends on global state, as it calls an LLVM function to determine the host's target triple.
pub unsafe fn host_target_triple() -> target::Triple {
    interop::Message::from_ptr(llvm_sys::target_machine::LLVMGetDefaultTargetTriple())
        .to_identifier()
        .into()
}

impl target::KnownTriple {
    /// Converts this target triple into the LLVM C API's represention for a target.
    ///
    /// # Safety
    /// See [`identifier_to_target_ref`].
    pub unsafe fn to_target_ref(&self) -> interop::Result<LLVMTargetRef> {
        identifier_to_target_ref(self.to_triple_string().as_id())
    }
}

impl From<LLVMCodeGenOptLevel> for target::CodeGenerationOptimization {
    fn from(level: LLVMCodeGenOptLevel) -> Self {
        match level {
            LLVMCodeGenOptLevel::LLVMCodeGenLevelNone => Self::None,
            LLVMCodeGenOptLevel::LLVMCodeGenLevelLess => Self::Less,
            LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault => Self::Default,
            LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive => Self::Aggressive,
        }
    }
}

impl From<LLVMRelocMode> for target::RelocationMode {
    fn from(mode: LLVMRelocMode) -> Self {
        match mode {
            LLVMRelocMode::LLVMRelocDefault => target::RelocationMode::Default,
            LLVMRelocMode::LLVMRelocStatic => target::RelocationMode::Static,
            LLVMRelocMode::LLVMRelocPIC => target::RelocationMode::PIC,
            LLVMRelocMode::LLVMRelocDynamicNoPic => target::RelocationMode::DynamicNoPIC,
            LLVMRelocMode::LLVMRelocROPI => target::RelocationMode::ROPI,
            LLVMRelocMode::LLVMRelocRWPI => target::RelocationMode::RWPI,
            LLVMRelocMode::LLVMRelocROPI_RWPI => target::RelocationMode::ROPIRWPI,
        }
    }
}

impl From<LLVMCodeModel> for target::CodeModel {
    fn from(model: LLVMCodeModel) -> Self {
        match model {
            LLVMCodeModel::LLVMCodeModelDefault => Self::Default,
            LLVMCodeModel::JITDefault => Self::JITDefault,
            LLVMCodeModel::LLVMCodeModelTiny => Self::Tiny,
            LLVMCodeModel::LLVMCodeModelSmall => Self::Small,
            LLVMCodeModel::LLVMCodeModelKernel => Self::Kernel,
            LLVMCodeModel::LLVMCodeModelMedium => Self::Medium,
            LLVMCodeModel::LLVMCodeModelLarge => Self::Large,
        }
    }
}
