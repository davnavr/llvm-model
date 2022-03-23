//! Contains types to represents strings that can be used in LLVM.
//! LLVM uses null-terminated strings, so `null` bytes are not allowed in names.

use std::convert::AsRef;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};

// TODO: Should identifiers contain only valid ASCII?

/// Error type used when an identifier contains `null` bytes.
#[derive(Debug, thiserror::Error)]
#[error("identifiers must not contain null bytes")]
pub struct Error;

/// A borrowed identifier string.
#[repr(transparent)]
pub struct Id(str);

impl Id {
    /// Creates a borrowed identifier from a borrowed string, skipping any checks for `null` bytes.
    ///
    /// # Safety
    /// The caller must ensure that the identifier does not contain any `null` bytes.
    #[allow(clippy::needless_lifetimes)]
    pub unsafe fn new_unchecked<'a>(identifier: &'a str) -> &'a Self {
        // Safety: A transparent representation is used for Id, so a &str should be compatible with a &Id
        &*(identifier as *const str as *const Self)
    }
}

impl<'a> TryFrom<&'a str> for &'a Id {
    type Error = Error;

    fn try_from(identifier: &'a str) -> Result<Self, Self::Error> {
        if identifier.bytes().any(|c| c == b'\0') {
            Err(Error)
        } else {
            // Safety: Check for null bytes is performed earlier.
            Ok(unsafe { Id::new_unchecked(identifier) })
        }
    }
}

impl Deref for Id {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

/// An owned identifier string.
#[derive(Clone, Default)]
#[repr(transparent)]
pub struct Identifier(String);

impl Identifier {
    /// Creates a new owned identifier string without checking for `null` bytes.
    ///
    /// # Safety
    /// The caller must ensure that the identifier does not contain any `null` bytes.
    pub unsafe fn new_unchecked(identifier: String) -> Self {
        Self(identifier)
    }
}

impl TryFrom<String> for Identifier {
    type Error = Error;

    fn try_from(identifier: String) -> Result<Self, Self::Error> {
        <&Id>::try_from(identifier.as_str())?;
        Ok(Self(identifier))
    }
}

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl DerefMut for Identifier {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl AsRef<Id> for Identifier {
    fn as_ref(&self) -> &Id {
        // Safety: The constructors of Identifier use the same validation checks for the constructors of Id.
        unsafe { Id::new_unchecked(&self.0) }
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        <Id as Debug>::fmt(self.as_ref(), f)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        <Id as Display>::fmt(self.as_ref(), f)
    }
}
