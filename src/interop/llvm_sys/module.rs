//! Code to interop with LLVM's C APIs for writing a module.

use crate::block;
use crate::global;
use crate::interop::llvm_sys as interop;
use crate::types;
use crate::Identifier;
use std::collections::hash_map;
use std::rc::Rc;

pub use llvm_sys::target_machine::LLVMCodeGenFileType as EmitType;

impl From<global::Linkage> for llvm_sys::LLVMLinkage {
    fn from(linkage: global::Linkage) -> Self {
        match linkage {
            global::Linkage::Private => Self::LLVMLinkerPrivateLinkage,
            global::Linkage::Internal => Self::LLVMInternalLinkage,
            global::Linkage::AvailableExternally => Self::LLVMAvailableExternallyLinkage,
            global::Linkage::Appending => Self::LLVMAppendingLinkage,
            global::Linkage::External => Self::LLVMExternalLinkage,
            _ => todo!("bad linkage {}", linkage),
        }
    }
}

/// Error used when an attempt to convert a module into an `LLVMModuleRef` fails.
#[derive(Debug)]
#[non_exhaustive]
pub enum BuildError {
    /// An unknown error produced by LLVM.
    Unknown(interop::Message),
}

crate::enum_case_from!(BuildError, Unknown, interop::Message);

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

    /// Transforms the contents of this module into an `LLVMModuleRef` suitable for use with the LLVM C APIs.
    ///
    /// # Safety
    /// Callers must ensure that the context reference is a valid pointer and that the context has not been disposed.
    pub unsafe fn into_reference(
        mut self,
        context: llvm_sys::prelude::LLVMContextRef,
    ) -> Result<Wrapper, BuildError> {
        let empty_string = std::ffi::CString::default();

        // Safety: module name is newly allocated and is valid.
        let reference = {
            let module_identfier = self.module.name().to_c_string();

            // Safety: module pointer is guaranteed to be valid.
            Wrapper::new_unchecked(llvm_sys::core::LLVMModuleCreateWithNameInContext(
                module_identfier.as_ptr(),
                context,
            ))
        };

        {
            // Safety: triple string is wrapped in message.
            let triple_string =
                interop::Message::from_ptr(llvm_sys::target_machine::LLVMGetTargetMachineTriple(
                    self.target.machine().reference(),
                ));

            // Safety: Message pointer is guaranteed to be valid.
            llvm_sys::core::LLVMSetTarget(reference.reference(), triple_string.to_ptr());
        }

        // Safety: target layout was previously allocated and is valid.
        llvm_sys::target::LLVMSetModuleDataLayout(
            reference.reference(),
            self.target.data_layout().reference(),
        );

        let mut type_cache = hash_map::HashMap::new();
        let mut get_type = |t: Rc<types::FirstClass>| match type_cache.entry(t) {
            hash_map::Entry::Occupied(occupied) => *occupied.get(),
            hash_map::Entry::Vacant(vacant) => {
                let type_reference = match std::convert::AsRef::as_ref(vacant.key()) {
                    types::FirstClass::Single(single_value_type) => match single_value_type {
                        types::SingleValue::Integer(integer_size) => {
                            llvm_sys::core::LLVMIntType(integer_size.bits())
                        }
                        _ => todo!("single value type not yet supported"),
                    },
                    _ => todo!("type not yet supported"),
                };

                *vacant.insert(type_reference)
            }
        };

        let mut function_type_cache = hash_map::HashMap::new();
        let mut get_function_type =
            |function_type: Rc<types::Function>| match function_type_cache.entry(function_type) {
                hash_map::Entry::Occupied(occupied) => *occupied.get(),
                hash_map::Entry::Vacant(vacant) => {
                    let function_type = vacant.key();

                    let return_type = match function_type.return_type() {
                        types::Return::Void => {
                            llvm_sys::core::LLVMVoidTypeInContext(reference.context())
                        }
                        types::Return::FirstClass(actual_return_type) => {
                            get_type(actual_return_type.clone())
                        }
                    };

                    let mut parameter_type_buffer = function_type
                        .parameter_types()
                        .iter()
                        .map(|parameter_type| get_type(parameter_type.clone()))
                        .collect::<Vec<_>>();

                    *vacant.insert(llvm_sys::core::LLVMFunctionType(
                        return_type,
                        parameter_type_buffer.as_mut_ptr(),
                        parameter_type_buffer
                            .len()
                            .try_into()
                            .expect("too many parameters"),
                        0,
                    ))
                }
            };

        // Safety: Builder is disposed after all functions are generated.
        let instruction_builder = llvm_sys::core::LLVMCreateBuilderInContext(reference.context());

        //LLVMConstIntOfArbitraryPrecision for values

        for global in self.module.drain_global_values() {
            match global {
                global::Value::Function(function) => {
                    let function_reference = llvm_sys::core::LLVMAddFunction(
                        reference.reference(),
                        function.name().to_c_string().as_ptr(),
                        get_function_type(function.signature().clone()),
                    );

                    llvm_sys::core::LLVMSetFunctionCallConv(
                        function_reference,
                        function.get_calling_convention().value(),
                    );

                    llvm_sys::core::LLVMSetLinkage(
                        function_reference,
                        function.get_linkage().into(),
                    );

                    // TODO: Iterate over all blocks
                    for block in function.take_basic_blocks().drain(..) {
                        let block_reference = llvm_sys::core::LLVMAppendBasicBlockInContext(
                            reference.context(),
                            function_reference,
                            empty_string.as_ptr(),
                        );

                        llvm_sys::core::LLVMPositionBuilderAtEnd(
                            instruction_builder,
                            block_reference,
                        );

                        for instruction in block.take_instructions().drain(..) {
                            use block::Instruction as Instr;

                            match instruction {
                                Instr::Ret(None) => {
                                    llvm_sys::core::LLVMBuildRetVoid(instruction_builder);
                                }
                                _ => todo!("bad instr"),
                            }
                        }
                    }

                    // TODO: Function attributes and other things.
                }
            }
        }

        llvm_sys::core::LLVMDisposeBuilder(instruction_builder);

        // TODO: Validate module?

        Ok(reference)
    }

    /// Writes the string representation of the LLVM module into a message.
    ///
    /// # Safety
    /// Callers must ensure that the LLVM context reference is not null and has not been disposed.
    pub unsafe fn into_message(
        self,
        context: llvm_sys::prelude::LLVMContextRef,
    ) -> Result<interop::Message, BuildError> {
        let module = self.into_reference(context)?;
        // Safety: String representation is an LLVM message that is disposed when the message is dropped.
        Ok(interop::Message::from_ptr(
            llvm_sys::core::LLVMPrintModuleToString(module.reference()),
        ))
    }

    /// Emits assembly code or an object file for the module's target machine into a memory buffer.
    ///
    /// # Safety
    /// See [`into_reference`].
    pub unsafe fn emit_target_code_to_buffer(
        self,
        context: llvm_sys::prelude::LLVMContextRef,
        file_type: EmitType,
    ) -> Result<interop::MemoryBuffer, BuildError> {
        let target_machine = self.target.machine();
        let module = self.into_reference(context)?;

        let mut buffer: llvm_sys::prelude::LLVMMemoryBufferRef = std::ptr::null_mut();
        let mut error: *mut i8 = std::ptr::null_mut();

        // Don't know if 1 or 0 means success, so the buffer is just checked instead.
        // Safety: Error is wrapped in a Message later so it is properly disposed.
        llvm_sys::target_machine::LLVMTargetMachineEmitToMemoryBuffer(
            target_machine.reference(),
            module.reference(),
            file_type,
            &mut error as *mut _,
            &mut buffer as *mut llvm_sys::prelude::LLVMMemoryBufferRef,
        );

        if buffer.is_null() {
            Err(BuildError::Unknown(interop::Message::from_ptr(error)))
        } else {
            Ok(interop::MemoryBuffer::from_reference_unchecked(buffer))
        }
    }
}

/// A wrapper over an LLVM module reference.
#[derive(Debug)]
#[repr(transparent)]
pub struct Wrapper(std::ptr::NonNull<llvm_sys::LLVMModule>);

impl Wrapper {
    /// Creates a wrapper over a module reference.
    ///
    /// # Safety
    /// Callers must ensure that the module reference is valid.
    pub unsafe fn new_unchecked(module: llvm_sys::prelude::LLVMModuleRef) -> Self {
        Self(std::ptr::NonNull::new_unchecked(module))
    }

    /// Gets the underlying module reference.
    ///
    /// # Safety
    /// Callers must ensure that the reference is used for the lifetime of the wrapper.
    pub unsafe fn reference(&self) -> llvm_sys::prelude::LLVMModuleRef {
        self.0.as_ptr()
    }

    /// Returns the underlying reference to the module.
    ///
    /// # Safety
    /// Callers are responsible for disposing the returned module reference by calling [`llvm_sys::core::LLVMDisposeModule`].
    pub unsafe fn into_reference(self) -> llvm_sys::prelude::LLVMModuleRef {
        self.reference()
    }

    /// Returns the context associated with the module.
    pub fn context(&self) -> llvm_sys::prelude::LLVMContextRef {
        unsafe {
            // Safety: module reference is assumed to be valid.
            llvm_sys::core::LLVMGetModuleContext(self.reference())
        }
    }
}

impl std::ops::Drop for Wrapper {
    fn drop(&mut self) {
        unsafe {
            // Safety: module reference is assumed to be valid.
            llvm_sys::core::LLVMDisposeModule(self.reference())
        }
    }
}
