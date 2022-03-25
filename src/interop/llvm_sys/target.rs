//! Code to interoperate with C APIs to obtain LLVM target information.

use crate::identifier;
use crate::interop::llvm_sys as interop;
use crate::target;
use std::borrow::Cow;
use std::ptr;

pub use llvm_sys::{
    target::LLVMTargetDataRef,
    target_machine::{
        LLVMCodeGenOptLevel, LLVMCodeModel, LLVMRelocMode, LLVMTargetMachineRef, LLVMTargetRef,
    },
};

/// Converts a target triple string to the LLVM C API's representation for targets.
///
/// # Safety
/// This function also depends on the initialization of targets LLVM can use by ensuring functions such
/// as by calling [`llvm_sys::target::LLVM_InitializeAllTargets`] and [`llvm_sys::target::LLVM_InitializeAllTargetInfos`].
pub unsafe fn identifier_to_target_ref(triple: &identifier::Id) -> interop::Result<LLVMTargetRef> {
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

impl target::KnownTriple {
    /// Converts this target triple into the LLVM C API's represention for a target.
    ///
    /// # Safety
    /// See [`identifier_to_target_ref`].
    pub unsafe fn to_target_ref(&self) -> interop::Result<LLVMTargetRef> {
        identifier_to_target_ref(self.to_triple_string().as_id())
    }
}

/// Error used when an attempt to convert from a target triple to an LLVM target reference failed.
#[derive(Debug)]
#[non_exhaustive]
pub enum InvalidTargetError {
    /// A custom target triple was used that contained interior `nul` bytes.
    InvalidIdentifier(identifier::Error),
    /// An LLVM message describing why the target is invalid.
    Message(interop::Message),
}

crate::enum_case_from!(InvalidTargetError, InvalidIdentifier, identifier::Error);
crate::enum_case_from!(InvalidTargetError, Message, interop::Message);

impl target::Triple {
    /// Converts the target triple to a LLVM C target reference, returning an error if a custom target contains null bytes.
    ///
    /// # Safety
    /// See [`identifier_to_target_ref`].
    pub unsafe fn to_target_ref(&self) -> Result<LLVMTargetRef, InvalidTargetError> {
        Ok(identifier_to_target_ref(self.to_triple_string()?.as_id())?)
    }

    /// Gets a target triple corresponding to the host's machine.
    ///
    /// # Safety
    /// Depends on global state, as it calls an LLVM function to determine the host's target triple.
    pub unsafe fn host_machine() -> target::Triple {
        interop::Message::from_ptr(llvm_sys::target_machine::LLVMGetDefaultTargetTriple())
            .to_identifier()
            .into()
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

impl From<target::CodeGenerationOptimization> for LLVMCodeGenOptLevel {
    fn from(level: target::CodeGenerationOptimization) -> Self {
        match level {
            target::CodeGenerationOptimization::None => Self::LLVMCodeGenLevelNone,
            target::CodeGenerationOptimization::Less => Self::LLVMCodeGenLevelLess,
            target::CodeGenerationOptimization::Default => Self::LLVMCodeGenLevelDefault,
            target::CodeGenerationOptimization::Aggressive => Self::LLVMCodeGenLevelAggressive,
        }
    }
}

impl From<LLVMRelocMode> for target::RelocationMode {
    fn from(mode: LLVMRelocMode) -> Self {
        match mode {
            LLVMRelocMode::LLVMRelocDefault => Self::Default,
            LLVMRelocMode::LLVMRelocStatic => Self::Static,
            LLVMRelocMode::LLVMRelocPIC => Self::PIC,
            LLVMRelocMode::LLVMRelocDynamicNoPic => Self::DynamicNoPIC,
            LLVMRelocMode::LLVMRelocROPI => Self::ROPI,
            LLVMRelocMode::LLVMRelocRWPI => Self::RWPI,
            LLVMRelocMode::LLVMRelocROPI_RWPI => Self::ROPIRWPI,
        }
    }
}

impl From<target::RelocationMode> for LLVMRelocMode {
    fn from(mode: target::RelocationMode) -> Self {
        match mode {
            target::RelocationMode::Default => Self::LLVMRelocDefault,
            target::RelocationMode::Static => Self::LLVMRelocStatic,
            target::RelocationMode::PIC => Self::LLVMRelocPIC,
            target::RelocationMode::DynamicNoPIC => Self::LLVMRelocDynamicNoPic,
            target::RelocationMode::ROPI => Self::LLVMRelocROPI,
            target::RelocationMode::RWPI => Self::LLVMRelocRWPI,
            target::RelocationMode::ROPIRWPI => Self::LLVMRelocROPI_RWPI,
        }
    }
}

impl From<LLVMCodeModel> for target::CodeModel {
    fn from(model: LLVMCodeModel) -> Self {
        match model {
            LLVMCodeModel::LLVMCodeModelDefault => Self::Default,
            LLVMCodeModel::LLVMCodeModelJITDefault => Self::JITDefault,
            LLVMCodeModel::LLVMCodeModelTiny => Self::Tiny,
            LLVMCodeModel::LLVMCodeModelSmall => Self::Small,
            LLVMCodeModel::LLVMCodeModelKernel => Self::Kernel,
            LLVMCodeModel::LLVMCodeModelMedium => Self::Medium,
            LLVMCodeModel::LLVMCodeModelLarge => Self::Large,
        }
    }
}

impl From<target::CodeModel> for LLVMCodeModel {
    fn from(model: target::CodeModel) -> Self {
        match model {
            target::CodeModel::Default => Self::LLVMCodeModelDefault,
            target::CodeModel::JITDefault => Self::LLVMCodeModelJITDefault,
            target::CodeModel::Tiny => Self::LLVMCodeModelTiny,
            target::CodeModel::Small => Self::LLVMCodeModelSmall,
            target::CodeModel::Kernel => Self::LLVMCodeModelKernel,
            target::CodeModel::Medium => Self::LLVMCodeModelMedium,
            target::CodeModel::Large => Self::LLVMCodeModelLarge,
        }
    }
}

/// Represents a target machine.
pub struct TargetMachine<'a> {
    triple: Cow<'a, target::Triple>,
    machine: Cow<'a, target::Machine>,
    reference: LLVMTargetMachineRef,
}

impl TargetMachine<'_> {
    pub fn machine(&self) -> &target::Machine {
        &self.machine
    }

    /// A value used to refer to the target machine in the LLVM C APIs.
    pub unsafe fn reference(&self) -> LLVMTargetMachineRef {
        self.reference
    }

    /// Gets the host's target machine.
    pub fn host_machine() -> Result<Self, InvalidTargetError> {
        todo!("host")
    }
}

impl<'a> TryFrom<Cow<'a, target::Machine>> for TargetMachine<'a> {
    type Error = InvalidTargetError;

    fn try_from(target_machine: Cow<'a, target::Machine>) -> Result<Self, Self::Error> {
        Ok(Self {
            machine: target_machine,
            triple: Cow::Borrowed(target_machine.target_triple()),
            reference: unsafe {
                // Safety: The Drop implementation disposes the target machine.
                llvm_sys::target_machine::LLVMCreateTargetMachine(
                    target_machine.target_triple().to_target_ref()?,
                    target_machine
                        .target_triple()
                        .to_triple_string()?
                        .into_c_string()
                        .as_ptr(),
                    target_machine.cpu_name().to_c_string().as_ptr(),
                    target_machine.features().to_c_string().as_ptr(),
                    target_machine.code_generation_optimization_level().into(),
                    target_machine.relocation_mode().into(),
                    target_machine.code_model().into(),
                )
            },
        })
    }
}

impl TryFrom<target::Machine> for TargetMachine<'_> {
    type Error = InvalidTargetError;

    fn try_from(target_machine: target::Machine) -> Result<Self, Self::Error> {
        Self::try_from(Cow::Owned(target_machine))
    }
}

impl<'a> TryFrom<&'a target::Machine> for TargetMachine<'a> {
    type Error = InvalidTargetError;

    fn try_from(target_machine: &'a target::Machine) -> Result<Self, Self::Error> {
        Self::try_from(Cow::Borrowed(target_machine))
    }
}

impl Drop for TargetMachine<'_> {
    fn drop(&mut self) {
        unsafe { llvm_sys::target_machine::LLVMDisposeTargetMachine(self.reference) }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum InvalidLayoutError {
    InvalidTarget(InvalidTargetError),
    ParseError(target::layout::ParseError<'static>),
}

crate::enum_case_from!(InvalidLayoutError, InvalidTarget, InvalidTargetError);
crate::enum_case_from!(InvalidLayoutError, ParseError, target::layout::ParseError<'static>);

impl target::Layout {
    /// Creates a target layout from a data layout.
    ///
    /// # Safety
    /// Callers must ensure that the target data layout is a valid pointer.
    pub unsafe fn from_layout_ref(
        target_layout: LLVMTargetDataRef,
    ) -> Result<Self, target::layout::ParseError<'static>> {
        interop::Message::from_ptr(llvm_sys::target::LLVMCopyStringRepOfTargetData(
            target_layout,
        ))
        .to_identifier()
        .try_into()
    }

    /// Creates a target layout corresponding to a target machine.
    pub fn from_machine_ref(
        target_machine: &TargetMachine,
    ) -> Result<Self, target::layout::ParseError<'static>> {
        Self::from_layout_ref(llvm_sys::target_machine::LLVMCreateTargetDataLayout(
            target_machine.reference(),
        ))
    }

    pub unsafe fn host_machine() -> Result<Self, InvalidLayoutError> {
        Ok(Self::from_machine_ref(&TargetMachine::host_machine()?)?)
    }
}

//impl target::Target { pub fn host_machine() -> Result<Self> {  } }
