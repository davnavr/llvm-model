//! Contains code to handle LLVM error messages.

use crate::identifier::Identifier;
use std::ffi::CStr;
use std::fmt::{Debug, Formatter};
use std::os::raw::c_char;

/// A wrapper for an LLVM message.
#[repr(transparent)]
pub struct Message {
    contents: std::ptr::NonNull<c_char>,
}

impl Message {
    /// Creates a new wrapper for an LLVM message.
    ///
    /// # Safety
    /// The `contents` pointer must be allocated by LLVM.
    ///
    /// # Panics
    /// Will panic if the `contents` pointer is `null`.
    pub unsafe fn from_ptr(contents: *mut c_char) -> Self {
        Self {
            contents: std::ptr::NonNull::new(contents).expect("message contents must not be null"),
        }
    }

    /// Returns a pointer to the contents of the message.
    ///
    /// # Safety
    /// Callers must ensure that the pointer is only used while the lifetime of the message is valid.
    pub unsafe fn to_ptr(&self) -> *const c_char {
        self.contents.as_ptr()
    }

    /// Interprets the contents of this LLVM message as a C string.
    pub fn as_c_str(&self) -> &CStr {
        unsafe {
            // Safety: Messages allocated by LLVM are assumed to be null terminated.
            CStr::from_ptr(self.contents.as_ptr())
        }
    }

    /// Attempts to copy the contents of this LLVM message into an owned identifier.
    pub fn to_identifier(&self) -> Identifier {
        unsafe {
            // Safety: The message is null terminated, so no interior nul bytes are possible.
            Identifier::new_unchecked(self.to_string().expect("invalid UTF-8 in message"))
        }
    }

    /// Copies the contents of this LLVM message into an owned string, returning an error value if the message did not contain
    /// valid UTF-8.
    pub fn to_string(&self) -> Result<String, std::str::Utf8Error> {
        Ok(self.as_c_str().to_str()?.to_string())
    }
}

impl std::convert::AsRef<CStr> for Message {
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl std::ops::Drop for Message {
    fn drop(&mut self) {
        unsafe { llvm_sys::core::LLVMDisposeMessage(self.contents.as_ptr()) }
    }
}

impl Debug for Message {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(self.as_c_str(), f)
    }
}
