//! Contains code for interacting with LLVM memory buffers.

use llvm_sys::prelude::LLVMMemoryBufferRef;

/// A wrapper around an LLVM memory buffer.
#[repr(transparent)]
pub struct MemoryBuffer {
    buffer: std::ptr::NonNull<llvm_sys::LLVMMemoryBuffer>,
}

impl MemoryBuffer {
    /// Creates a wrapper for the specified LLVM memory buffer.
    ///
    /// # Safety
    /// Callers must ensure that the memory buffer refernce is valid and has not been disposed.
    pub unsafe fn from_reference_unchecked(buffer: LLVMMemoryBufferRef) -> Self {
        Self {
            buffer: std::ptr::NonNull::new_unchecked(buffer),
        }
    }

    /// Returns the underlying memory buffer.
    ///
    /// # Safety
    /// Callers must ensure that that the wrapper outlives any use of the returned pointer.
    pub unsafe fn reference(&self) -> LLVMMemoryBufferRef {
        self.buffer.as_ptr()
    }

    /// Gets the length of this buffer.
    pub fn len(&self) -> usize {
        unsafe {
            // Safety: Buffer is assumed to be valid, and pointer is only used while self has not been dropped.
            llvm_sys::core::LLVMGetBufferSize(self.reference())
        }
    }

    /// Gets a value indicating if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets a pointer to the start of this buffer.
    pub fn as_ptr(&self) -> *const u8 {
        unsafe {
            // Safety: Buffer is assumed to be valid.
            let buffer_start: *const i8 = llvm_sys::core::LLVMGetBufferStart(self.reference());
            buffer_start as *const u8
        }
    }

    /// Gets a byte slice over the memory buffer.
    pub fn as_slice<'b>(&'b self) -> &'b [u8] {
        unsafe {
            // Safety:
            // - The length, start of buffer, is assumed to be valid.
            // - The alignment is probably correct.
            // - Mutation never occurs assuming LLVM only reads buffers.
            std::slice::from_raw_parts::<'b, u8>(self.as_ptr(), self.len())
        }
    }
}

impl std::ops::Deref for MemoryBuffer {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl std::convert::AsRef<[u8]> for MemoryBuffer {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl std::borrow::Borrow<[u8]> for MemoryBuffer {
    fn borrow(&self) -> &[u8] {
        self.as_slice()
    }
}

impl From<&[u8]> for MemoryBuffer {
    fn from(memory: &[u8]) -> Self {
        unsafe {
            let buffer: *const u8 = memory.as_ptr();
            Self::from_reference_unchecked(
                llvm_sys::core::LLVMCreateMemoryBufferWithMemoryRangeCopy(
                    buffer as *const i8,
                    memory.len(),
                    // Assuming here that buffer name can be empty.
                    std::ffi::CString::default().as_ptr(),
                ),
            )
        }
    }
}

impl std::fmt::Debug for MemoryBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.as_slice(), f)
    }
}

impl std::ops::Drop for MemoryBuffer {
    fn drop(&mut self) {
        unsafe {
            // Safety: The memory buffer is assumed to be valid.
            llvm_sys::core::LLVMDisposeMemoryBuffer(self.reference())
        }
    }
}
