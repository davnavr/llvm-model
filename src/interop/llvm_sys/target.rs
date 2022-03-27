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
/// Callers must ensure that they dispose of the returned target reference.
///
/// This function also depends on the initialization of targets LLVM can use by ensuring functions such
/// as by calling [`llvm_sys::target::LLVM_InitializeAllTargets`] and [`llvm_sys::target::LLVM_InitializeAllTargetInfos`].
pub unsafe fn identifier_to_target_ref(triple: &identifier::Id) -> interop::Result<LLVMTargetRef> {
    let mut error = ptr::null_mut();
    let mut target = ptr::null_mut();
    if llvm_sys::target_machine::LLVMGetTargetFromTriple(
        triple.to_c_string().as_ptr(),
        &mut target as *mut LLVMTargetRef,
        &mut error as *mut *mut _,
    ) == 0
    {
        Ok(target)
    } else {
        Err(interop::Message::from_ptr(error))
    }
}

/// An LLVM target triple.
#[derive(Debug)]
pub struct TargetTriple<'a> {
    triple: Cow<'a, target::Triple>,
    // No drop implementation, since a dispose function does not appear to exist for target triples.
    reference: LLVMTargetRef,
}

/// Error used when an attempt to convert from a target triple to an LLVM target reference failed.
#[derive(Debug)]
#[non_exhaustive]
pub enum InvalidTripleError {
    /// A custom target triple was used that contained interior `nul` bytes.
    InvalidIdentifier(identifier::Error),
    /// An LLVM message describing why the target is invalid.
    Message(interop::Message),
}

crate::enum_case_from!(InvalidTripleError, InvalidIdentifier, identifier::Error);
crate::enum_case_from!(InvalidTripleError, Message, interop::Message);

impl<'a> TargetTriple<'a> {
    /// Gets the target triple.
    pub fn triple(&self) -> &target::Triple {
        &self.triple
    }

    /// Gets a reference to a value used to refer to the target triple in LLVM's C API.
    ///
    /// # Safety
    /// Callers must ensure that the returned reference is only used for the lifetime of `self`.
    pub unsafe fn reference(&self) -> LLVMTargetRef {
        self.reference
    }

    /// Creates a well-known target triple for use with LLVM.
    ///
    /// # Safety
    /// See [`identifier_to_target_ref`].
    pub unsafe fn from_known(triple: target::KnownTriple) -> interop::Result<Self> {
        Ok(Self {
            reference: identifier_to_target_ref(triple.to_triple_string().as_id())?,
            triple: Cow::Owned(target::Triple::Known(triple)),
        })
    }

    /// Creates a target triple for use with LLVM.
    ///
    /// # Safety
    /// See [`identifier_to_target_ref`].
    pub unsafe fn new(triple: Cow<'a, target::Triple>) -> Result<Self, InvalidTripleError> {
        Ok(Self {
            reference: identifier_to_target_ref(triple.to_triple_string()?.as_id())?,
            triple,
        })
    }

    /// Gets a target triple corresponding to the host's machine.
    ///
    /// # Safety
    /// See [`identifier_to_target_ref`].
    pub unsafe fn host_machine() -> Result<Self, InvalidTripleError> {
        // Safety: Allocated message is disposed after being converted to an identifier.
        let triple_string =
            interop::Message::from_ptr(llvm_sys::target_machine::LLVMGetDefaultTargetTriple())
                .to_identifier();

        Self::new(Cow::Owned(target::Triple::from(triple_string)))
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
#[derive(Debug)]
pub struct TargetMachine {
    machine: target::Machine,
    reference: LLVMTargetMachineRef,
}

impl TargetMachine {
    /// Information that describes this target machine.
    pub fn machine(&self) -> &target::Machine {
        &self.machine
    }

    /// A value used to refer to the target machine in the LLVM C APIs.
    ///
    /// # Safety
    /// Callers must ensure that the reference is only used for the lifetime of `self`.
    pub unsafe fn reference(&self) -> LLVMTargetMachineRef {
        self.reference
    }

    /// Gets the host's target machine.
    ///
    /// # Safety
    /// Relies on the initialization of target information for the host machine, such as with
    /// [`llvm_sys::target::LLVM_InitializeNativeTarget`].
    pub unsafe fn host_machine(
        optimization_level: target::CodeGenerationOptimization,
        relocation_mode: target::RelocationMode,
        code_model: target::CodeModel,
    ) -> Result<Self, InvalidTripleError> {
        let host_triple = TargetTriple::host_machine()?;

        // Safety: disposed only after returning.
        let cpu_name = interop::Message::from_ptr(llvm_sys::target_machine::LLVMGetHostCPUName());

        // Safety: disposed only after returning.
        let features =
            interop::Message::from_ptr(llvm_sys::target_machine::LLVMGetHostCPUFeatures());

        Ok(Self {
            reference:
                // Safety: The Drop implementation disposes the target machine.
                llvm_sys::target_machine::LLVMCreateTargetMachine(
                    host_triple.reference(),
                    host_triple
                        .triple()
                        .to_triple_string()?
                        .into_c_string()
                        .as_ptr(),
                    cpu_name.to_ptr(),
                    features.to_ptr(),
                    optimization_level.into(),
                    relocation_mode.into(),
                    code_model.into(),
                ),
            machine: target::Machine::new(
                host_triple.triple().clone(),
                cpu_name.to_identifier(),
                features.to_identifier(),
                optimization_level,
                relocation_mode,
                code_model,
            ),
        })
    }

    //pub unsafe fn from_reference
}

impl TryFrom<target::Machine> for TargetMachine {
    type Error = InvalidTripleError;

    fn try_from(target_machine: target::Machine) -> Result<Self, Self::Error> {
        Ok(Self {
            reference: unsafe {
                let target_triple =
                    TargetTriple::new(Cow::Borrowed(target_machine.target_triple()))?;

                // Safety: The Drop implementation disposes the target machine.
                llvm_sys::target_machine::LLVMCreateTargetMachine(
                    target_triple.reference(),
                    target_triple
                        .triple()
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
            machine: target_machine,
        })
    }
}

impl Drop for TargetMachine {
    fn drop(&mut self) {
        unsafe { llvm_sys::target_machine::LLVMDisposeTargetMachine(self.reference) }
    }
}

/// Error used when parsing a target data layout fails.
pub type LayoutParseError = target::layout::ParseError;

/// Represents a target data layout.
#[derive(Debug)]
pub struct TargetLayout {
    layout: target::Layout,
    reference: LLVMTargetDataRef,
}

impl TargetLayout {
    /// Description of the target layout.
    pub fn layout(&self) -> &target::Layout {
        &self.layout
    }

    /// A value used to refer to the target layout in the LLVM C APIs.
    ///
    /// # Safety
    /// Callers must ensure that the reference is only used for the lifetime of `self`.
    pub unsafe fn reference(&self) -> LLVMTargetDataRef {
        self.reference
    }

    /// Creates a target layout from a reference.
    ///
    /// # Safety
    /// Callers must ensure that the target data layout is a valid pointer and are responsible for disposing of the reference
    /// ONLY if an error is returned.
    pub unsafe fn from_reference(
        target_layout: LLVMTargetDataRef,
    ) -> Result<Self, LayoutParseError> {
        // Safety: not dropped before being converted to an identifier.
        let layout_string = interop::Message::from_ptr(
            llvm_sys::target::LLVMCopyStringRepOfTargetData(target_layout),
        );

        let parsed_layout = target::Layout::try_from(layout_string.to_identifier())?;

        Ok(Self {
            layout: parsed_layout,
            reference: target_layout,
        })
    }
}

impl TryFrom<&TargetMachine> for TargetLayout {
    type Error = LayoutParseError;

    fn try_from(target_machine: &TargetMachine) -> Result<Self, Self::Error> {
        unsafe {
            // Safety: Target machine reference is only used for duration of this function call.
            let machine_reference = target_machine.reference();
            // Safety: Target machine reference is a valid pointer.
            let layout_reference =
                llvm_sys::target_machine::LLVMCreateTargetDataLayout(machine_reference);
            // Safety: Target layout reference is a valid pointer, and disposal of reference on error is performed below.
            let result = Self::from_reference(layout_reference);

            if result.is_err() {
                // Safety: Target layout reference is a valid pointer.
                llvm_sys::target::LLVMDisposeTargetData(layout_reference);
            }

            result
        }
    }
}

//impl TryFrom<target::Layout> for TargetLayout { }

impl Drop for TargetLayout {
    fn drop(&mut self) {
        unsafe {
            // Safety: Target layout reference is a valid pointer that was "owned" by self.
            llvm_sys::target::LLVMDisposeTargetData(self.reference())
        }
    }
}

/// Error used when creation of a target fails.
#[derive(Debug)]
#[non_exhaustive]
pub enum InvalidTargetError {
    /// Used when an attempt to create a target failed to an invalid target triple.
    InvalidTriple(InvalidTripleError),
    /// Used when a target's data layout could not be parsed.s
    InvalidLayout(LayoutParseError),
}

crate::enum_case_from!(InvalidTargetError, InvalidTriple, InvalidTripleError);
crate::enum_case_from!(InvalidTargetError, InvalidLayout, LayoutParseError);

/// An LLVM target machine and data layout.
#[derive(Debug)]
pub struct Target {
    machine: TargetMachine,
    layout: TargetLayout,
    target: target::Target,
}

impl Target {
    /// Creates a new target.
    pub fn new(machine: TargetMachine, layout: TargetLayout) -> Self {
        Self {
            target: target::Target::new(machine.machine().clone(), layout.layout().clone()),
            machine,
            layout,
        }
    }

    /// Gets the machine for this target.
    pub fn machine(&self) -> &TargetMachine {
        &self.machine
    }

    /// Gets the target.
    pub fn target(&self) -> &target::Target {
        &self.target
    }

    /// Gets the data layout for this target.
    pub fn data_layout(&self) -> &TargetLayout {
        &self.layout
    }

    /// Gets a target for the host machine.
    ///
    /// # Safety
    /// See [`TargetMachine::host_machine`].
    pub unsafe fn host_machine_target(
        optimization_level: target::CodeGenerationOptimization,
        relocation_mode: target::RelocationMode,
        code_model: target::CodeModel,
    ) -> Result<Self, InvalidTargetError> {
        let host_machine =
            TargetMachine::host_machine(optimization_level, relocation_mode, code_model)?;
        let host_layout = TargetLayout::try_from(&host_machine)?;
        Ok(Self::new(host_machine, host_layout))
    }
}
