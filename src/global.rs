//! Modules consist of global values, which are global variables or function definitions.

use crate::types;
use crate::{Id, Identifier};
use std::cell::Cell;
use std::fmt::{Display, Formatter, Write as _};
use std::rc::Rc;

// TODO: Split linkage types into those that are valid for global variables, functions, and both.
/// Describes how global variables or functions are linked.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Linkage {
    /// Accessible only to the current module, and renames any symbols "as necessary to avoid collisions".
    Private,
    /// "Similar to private, but the value shows as a local symbol...Corresponds to the notion of the `static` keyword in C".
    Internal,
    /// The global is an external definition.
    AvailableExternally,
    /// Merged with globals of the same name.
    LinkOnce,
    /// Similar to `linkonce`, "except that unreferenced globals...may not be discarded".
    Weak,
    /// Similar to `weak`. Note that "Functions and aliases may not have `common` linkage".
    Common,
    /// Can "only be applied to global variables of pointer to array type". Used to append global arrays together.
    Appending,
    /// "the symbol is weak until linked, if not linked, the symbol becomes null"
    ExternWeak,
    /// Similar to `linkonce`, except that equivalent globals are merged.
    LinkOnceODR,
    /// Similar to `weak`, except that equivalent globals are merged.
    WeakODR,
    /// Indicates that the global is externally visible.
    External,
}

crate::enum_default!(Linkage, External);

impl Display for Linkage {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::Private => "private",
            Self::Internal => "internal",
            Self::AvailableExternally => "available_externally",
            Self::LinkOnce => "linkonce",
            Self::Weak => "weak",
            Self::Common => "common",
            Self::Appending => "appending",
            Self::ExternWeak => "extern_weak",
            Self::LinkOnceODR => "linkonce_odr",
            Self::WeakODR => "weak_odr",
            Self::External => "external",
        })
    }
}

/// Well-known calling conventions used by functions.
///
/// See [the latest LLVM documentation on calling conventions here](https://llvm.org/docs/LangRef.html#callingconv).
#[derive(Copy, Clone, Debug, Eq)]
#[non_exhaustive]
pub enum CallingConvention {
    /// The target platform's C calling conventions.
    C,
    /// This "attempts to make calls as fast as possible (e.g. by passing things in registers)", and allows allows tail call
    /// optimization.
    Fast,
    /// This makes "code in the caller as efficient as possible under the assumption that the call is not commonly executed".
    Cold,
    /// Used by the Glasgow Haskell Compiler, this convention "passes everything in registers, going to exteremes...disabling
    /// callee save registers." Refer to the language reference for more information on its limitations.
    GHC,
    /// This convention is used by the High-Performance Erlang compiler, refer to the language reference for information
    /// regarding its limitations.
    HiPE,
    /// Calling convention used by the WebKit FTL JIT compiler for JavaScript.
    WebKitJS,
    /// This "supports patching an arbitrary code sequence in place of the call site." Refer to the language reference for more
    /// information.
    AnyReg,
    /// This "attempts to make code in the caller as unintrusive as possible."
    PreserveMost,
    /// More powerful version of `preserve_most`.
    PreserveAll,
    /// Used by the Clang compiler when generating "an access function to access C++-style TLS."
    CxxFastTLS,
    ///// Supports tail call optimization.
    //Tail,
    /// Calling convention used by the Swift programming language.
    Swift,
    ///// Similar to the Swift language calling conventions, but supports mandatory tail calls.
    //SwiftTail,
    /// A custom calling convention, with target specific calling conventions starting at `64`.
    Custom(u32),
}

crate::enum_default!(CallingConvention, C);

impl CallingConvention {
    /// Gets an integer value indicating the calling convention.
    pub fn value(self) -> u32 {
        match self {
            Self::C => 0,
            Self::Fast => 8,
            Self::Cold => 9,
            Self::GHC => 10,
            Self::HiPE => 11,
            Self::WebKitJS => 12,
            Self::AnyReg => 13,
            Self::PreserveMost => 14,
            Self::PreserveAll => 15,
            Self::Swift => 16,
            Self::CxxFastTLS => 17,
            Self::Custom(value) => value,
        }
    }
}

impl std::cmp::PartialEq for CallingConvention {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Display for CallingConvention {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::C => f.write_str("ccc"),
            Self::Fast => f.write_str("fastcc"),
            Self::Cold => f.write_str("coldcc"),
            Self::WebKitJS => f.write_str("webkit_jscc"),
            Self::AnyReg => f.write_str("anyregcc"),
            Self::PreserveMost => f.write_str("preserve_mostcc"),
            Self::PreserveAll => f.write_str("preserve_allcc"),
            Self::CxxFastTLS => f.write_str("cxx_fast_tlscc"),
            Self::Swift => f.write_str("swiftcc"),
            _ => write!(f, "cc {}", self.value()),
        }
    }
}

/// A function definition or declaration.
///
/// See [the latest LLVM documentation on functions here](https://llvm.org/docs/LangRef.html#functions).
#[derive(Debug)]
pub struct Function {
    name: Identifier,
    signature: Rc<types::Function>,
    linkage: Cell<Linkage>,
    calling_convention: Cell<CallingConvention>,
}

impl Function {
    /// Creates a new function.
    pub fn new(name: Identifier, signature: impl Into<Rc<types::Function>>) -> Rc<Self> {
        Rc::new(Self {
            name,
            signature: signature.into(),
            linkage: Cell::default(),
            calling_convention: Cell::default(),
        })
    }

    /// Gets the name of this function.
    pub fn name(&self) -> &Id {
        self.name.as_id()
    }

    /// Gets the signature of this function.
    pub fn signature(&self) -> &Rc<types::Function> {
        &self.signature
    }

    /// Gets the linkage type for this function.
    pub fn get_linkage(&self) -> Linkage {
        self.linkage.get()
    }

    /// Sets the linkage type for this function.
    pub fn set_linkage(&self, linkage: Linkage) {
        self.linkage.set(linkage)
    }

    /// Gets the calling convention of this function.
    pub fn get_calling_convention(&self) -> CallingConvention {
        self.calling_convention.get()
    }

    /// Sets the calling convention used by this function.
    pub fn set_calling_convention(&self, calling_convention: CallingConvention) {
        self.calling_convention.set(calling_convention)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "define {}", self.get_linkage())?;
        //rtpreemt
        //visibility
        //dllst
        write!(f, " {}", self.get_calling_convention())?;
        //unnamed_addr
        write!(f, " {}", self.signature.return_type())?;
        //attribute of return type
        write!(f, " @{} (", self.name())?;
        for (index, parameter_type) in self.signature().parameter_types().iter().enumerate() {
            if index > 0 {
                f.write_str(", ")?;
            }

            // parameter attributes
            Display::fmt(&parameter_type, f)?;
        }
        f.write_char(')')?;
        // other things
        //'{' and then the basic blocks
        Ok(())
    }
}

/// A global value in a module, either a global variable or a function definition.
#[derive(Debug)]
pub enum Value {
    //Variable(Variable),
    /// A function definition.
    Function(Rc<Function>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Function(function) => Display::fmt(&function, f),
        }
    }
}
