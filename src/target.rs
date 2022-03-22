//! LLVM target triple and layout information is used to describe the host that will run the code.

use crate::identifier::Identifier;

/// The Instruction Set Architecture being targeted in a target triple.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Architecture {
    /// An unknown architecture type, usually the architecture being targeted is known beforehand.
    Unknown,
    /// A family of RISC architectures.
    ARM,
    /// 64-bit version of the ARM architecture, sometimes known as ARM64.
    AArch64,
    /// A RISC architecture commonly used in embedded systems and by students in universities.
    MIPS,
    /// WebAssembly is a stack-based bytecode language supported by all major web browsers.
    Wasm32,
    /// Version of WebAssembly with support for 64-bit memory indices.
    ///
    /// [See the original proposal here](https://github.com/WebAssembly/memory64) for more information.
    Wasm64,
    /// A family of CISC instruction set architectures.
    X86,
    /// A 64-bit version of the X86 architecture, sometimes known as AMD64.
    X86_64,
}

impl Architecture {
    /// The architecture corresponding to the target that this library and your code is compiled for.
    ///
    /// If the target architecture is exotic, defaults to [`Architecture::Unknown`].
    pub const fn current() -> Self {
        if cfg!(target_arch = "x86_64") {
            Self::X86_64
        } else if cfg!(target_arch = "aarch64") {
            Self::AArch64
        } else if cfg!(target_arch = "x86") {
            Self::X86
        } else if cfg!(target_arch = "mips") {
            Self::MIPS
        } else if cfg!(target_arch = "arm") {
            Self::ARM
        } else if cfg!(target_arch = "wasm32") {
            Self::Wasm32
        } else if cfg!(target_arch = "wasm64") {
            Self::Wasm64
        } else {
            Self::Unknown
        }
    }
}

crate::enum_default!(Architecture, Unknown);

/// Describes the vendor of a target triple.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Vendor {
    /// An unknown vendor.
    Unknown,
    /// Vendor used for some windows and linux targets.
    PC,
}

crate::enum_default!(Vendor, Unknown);

impl Vendor {
    /// The vendor corresponding to the target that this library and your code is compiled for.
    ///
    /// Defaults to [`Architecture::Unknown`].
    pub const fn current() -> Self {
        if cfg!(target_vendor = "pc") {
            Self::PC
        } else {
            Self::Unknown
        }
    }
}

/// The operating system of a target triple.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum OperatingSystem {
    /// An unknown operating system, usually the operating system that is being targeted is known.
    Unknown,
    /// Indicates that code compiled for this target is running on the bare metal.
    None,
    /// Operating system for Apple's iPhone.
    IOS,
    /// A family of Unix-like operating systems.
    Linux,
    /// Operating system developed by Apple.
    MacOSX,
    /// The [WebAssembly System Interface](https://github.com/WebAssembly/WASI), which allows WebAssembly programs to interact
    /// with the outside world.
    WASI,
    /// The Windows family of operating systems created by Microsoft.
    Windows,
}

impl OperatingSystem {
    /// The operating system that this library and your code is targeting.
    ///
    /// Defaults to [`OperatingSystem::Unknown`].
    pub const fn current() -> Self {
        if cfg!(target_os = "linux") {
            Self::Linux
        } else if cfg!(target_os = "windows") {
            Self::Windows
        } else {
            Self::Unknown
        }
    }
}

crate::enum_default!(OperatingSystem, Unknown);

/// Additional information used to disambiguate targets.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Environment {
    /// An unknown environment.
    Unknown,
    /// Family of open-source software that includes an implementation of the C standard library called
    /// [`glibc`](https://www.gnu.org/software/libc/).
    GNU,
    /// Open-source implementation of the C standard library (https://musl.libc.org/).
    MUSL,
    /// The Microsoft Visual C/C++ toolchain, available on windows as an additional component for Visual Studio.
    MSVC,
    /// The Common Language Runtime, used to run languages such as C# or F#.
    ///
    /// This environment type seems to have been added for the now defunct [`LLILC project`](https://github.com/dotnet/llilc/),
    /// which translated Common Intermediate Language bytecode into LLVM IR.
    CoreCLR,
}

impl Environment {
    /// The environment that this library and your code is targeting.
    ///
    /// Defaults to [`Environment::Unknown`].
    pub const fn current() -> Self {
        if cfg!(target_env = "gnu") {
            Self::GNU
        } else if cfg!(target_env = "musl") {
            Self::MUSL
        } else if cfg!(target_env = "msvc") {
            Self::MSVC
        } else {
            Self::Unknown
        }
    }
}

crate::enum_default!(Environment, Unknown);

/// Represents a typical LLVM target triple.
///
/// If a custom target triple is needed, use [`Triple::Custom`] instead.
#[derive(Clone, Debug, Default)]
pub struct KnownTriple {
    architecture: Architecture,
    vendor: Vendor,
    operating_system: OperatingSystem,
    environment: Environment,
}

const CURRENT_TARGET_TRIPLE: KnownTriple = KnownTriple {
    architecture: Architecture::current(),
    vendor: Vendor::current(),
    operating_system: OperatingSystem::current(),
    environment: Environment::current(),
};

impl KnownTriple {
    /// Creates a target triple
    pub fn with_environment(
        architecture: Architecture,
        vendor: Vendor,
        operating_system: OperatingSystem,
        environment: Environment,
    ) -> Self {
        Self {
            architecture,
            vendor,
            operating_system,
            environment,
        }
    }

    /// Creates a target triple with an unknown environment.
    pub fn with_operating_system(
        architecture: Architecture,
        vendor: Vendor,
        operating_system: OperatingSystem,
    ) -> Self {
        Self::with_environment(architecture, vendor, operating_system, Environment::Unknown)
    }

    /// The target triple corresponding to the target that this library and your code is compiled for.
    pub const fn current() -> &'static KnownTriple {
        &CURRENT_TARGET_TRIPLE
    }

    /// Gets the architecture of this target triple, which describes the instruction set being used.
    pub fn architecture(&self) -> &Architecture {
        &self.architecture
    }

    /// Gets the vendor component of this target triple.
    pub fn vendor(&self) -> &Vendor {
        &self.vendor
    }

    /// Gets the system component of this target triple.
    pub fn operating_system(&self) -> &OperatingSystem {
        &self.operating_system
    }

    /// Gets the environment component of this target triple.
    pub fn environment(&self) -> &Environment {
        &self.environment
    }
}

/// An LLVM target triple, typically in the format `ARCHITECTURE-VENDOR-OPERATING_SYSTEM`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Triple {
    /// A custom LLVM target triple.
    ///
    /// Use this if you need to specify certain advanced options such as the sub-architecture or ABI.
    Custom(Identifier),
    /// A target triple that is not custom.
    Known(KnownTriple),
}

impl std::default::Default for Triple {
    /// A target triple whose components are all unknown.
    fn default() -> Self {
        Self::Known(KnownTriple::default())
    }
}

//pub struct Layout
