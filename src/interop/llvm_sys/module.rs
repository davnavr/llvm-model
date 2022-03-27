//! Code to interop with LLVM's C APIs for writing a module.

use crate::interop::llvm_sys as interop;
use crate::Identifier;

/// Error used when an attempt to convert a module into an `LLVMModuleRef` fails.
#[derive(Debug)]
#[non_exhaustive]
pub enum BuildError {}

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
    /// Callers must ensure that the context reference is a valid pointer, and that they are responsible for disposing the
    /// returned module reference by calling [`llvm_sys::core::LLVMDisposeModule`].
    pub unsafe fn into_reference(
        self,
        context: llvm_sys::prelude::LLVMContextRef,
    ) -> Result<llvm_sys::prelude::LLVMModuleRef, BuildError> {
        // Safety: module name is newly allocated and is valid.
        let reference = {
            let module_identfier = self.module.name().to_c_string();

            llvm_sys::core::LLVMModuleCreateWithNameInContext(module_identfier.as_ptr(), context)
        };

        // TODO: Figure out if things like CPU name, CPU features, code_layout, etc. of target machine is needed or can even be set.
        {
            // Safety: triple string is wrapped in message.
            let triple_string =
                interop::Message::from_ptr(llvm_sys::target_machine::LLVMGetTargetMachineTriple(
                    self.target.machine().reference(),
                ));

            // Safety: Message pointer is guaranteed to be valid.
            llvm_sys::core::LLVMSetTarget(reference, triple_string.to_ptr());
        }

        // Safety: target layout was previously allocated and is valid.
        llvm_sys::target::LLVMSetModuleDataLayout(reference, self.target.data_layout().reference());

        Ok(reference)
    }

    /// Writes the string representation of the LLVM module into a message.
    ///
    /// # Safety
    /// Callers must ensure that the LLVM context reference is not null.
    pub unsafe fn into_message(
        self,
        context: llvm_sys::prelude::LLVMContextRef,
    ) -> Result<interop::Message, BuildError> {
        // Safety: module reference is not null.
        let module = Wrapper::new_unchecked(self.into_reference(context)?);
        // Safety: String representation is an LLVM message that is disposed when the message is dropped.
        Ok(interop::Message::from_ptr(
            llvm_sys::core::LLVMPrintModuleToString(module.reference()),
        ))
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

    /// Returns the underlying module reference.
    ///
    /// # Safety
    /// Callers must ensure that the reference is used for the lifetime of the wrapper.
    pub unsafe fn reference(&self) -> llvm_sys::prelude::LLVMModuleRef {
        self.0.as_ptr()
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
