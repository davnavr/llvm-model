//! LLVM target triple and layout information is used to describe the host that will run the code.

use crate::identifier::{self, Id, Identifier};
use std::fmt::{Display, Formatter};

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
    /// A family of CISC instruction set architectures, sometimes known as i686.
    X86,
    /// A 64-bit version of the X86 architecture, sometimes known as AMD64.
    X86_64,
}

impl Architecture {
    /// An estimate for the architecture corresponding to the target that this library and your code is compiled for.
    ///
    /// If the target architecture is exotic, defaults to [`Architecture::Unknown`].
    pub const fn current_estimate() -> Self {
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

impl Display for Architecture {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::Unknown => "unknown",
            Self::ARM => "arm",
            Self::AArch64 => "aarch64",
            Self::MIPS => "mips",
            Self::Wasm32 => "wasm32",
            Self::Wasm64 => "wasm64",
            Self::X86 => "i686",
            Self::X86_64 => "x86_64",
        })
    }
}

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
    /// An estimate for the vendor corresponding to the target that this library and your code is compiled for.
    ///
    /// Defaults to [`Architecture::Unknown`].
    pub const fn current_estimate() -> Self {
        if cfg!(target_vendor = "pc") {
            Self::PC
        } else {
            Self::Unknown
        }
    }
}

impl Display for Vendor {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::Unknown => "unknown",
            Self::PC => "pc",
        })
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
    /// An estimate for the operating system that this library and your code is targeting.
    ///
    /// Defaults to [`OperatingSystem::Unknown`].
    pub const fn current_estimate() -> Self {
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

impl Display for OperatingSystem {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::Unknown => "unknown",
            Self::None => "none",
            Self::IOS => "ios",
            Self::Linux => "linux",
            Self::MacOSX => "macosx", //"darwin",
            Self::Windows => "windows",
            Self::WASI => "wasi",
        })
    }
}

/// Additional information used to disambiguate targets.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Environment {
    /// An unknown environment.
    Unknown,
    /// Family of open-source software that includes an implementation of the C standard library called
    /// [`glibc`](https://www.gnu.org/software/libc/).
    GNU,
    /// Open-source implementation of the C standard library (<https://musl.libc.org/>).
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
    /// An estimate for the environment that this library and your code is targeting.
    ///
    /// Defaults to [`Environment::Unknown`].
    pub const fn current_estimate() -> Self {
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

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::Unknown => "unknown",
            Self::GNU => "gnu",
            Self::MUSL => "musl",
            Self::MSVC => "msvc",
            Self::CoreCLR => "coreclr",
        })
    }
}

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
    architecture: Architecture::current_estimate(),
    vendor: Vendor::current_estimate(),
    operating_system: OperatingSystem::current_estimate(),
    environment: Environment::current_estimate(),
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

    /// An estimate for the target triple corresponding to the target that this library and your code is compiled for.
    pub const fn current_estimate() -> &'static KnownTriple {
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

    /// Returns the LLVM triple string for this target triple.
    pub fn to_triple_string(&self) -> Identifier {
        unsafe {
            // Safety: Callers cannot create a custom triple here, so no null bytes exist.
            Identifier::new_unchecked(self.to_string())
        }
    }
}

/// Used when a known target triple could not be parsed correctly.
///
/// If you know for sure that your target triple is correct, consider using [`Triple::Known`] instead.
#[derive(Clone, Debug, thiserror::Error)]
#[error("{contents} is not a known target triple")]
pub struct UnknownTripleError<'a> {
    contents: &'a str,
}

impl<'a> std::convert::TryFrom<&'a Id> for KnownTriple {
    type Error = UnknownTripleError<'a>;

    /// Attempts to parse a known target triple from an identifier, expecting a triple in the format `ARCHITECTURE-VENDOR-OS` or
    /// `ARCHITECTURE-VENDOR-OS-ENVIORNMENT`
    fn try_from(triple: &'a Id) -> Result<Self, Self::Error> {
        let mut identifiers = triple.split('-');

        macro_rules! fail {
            () => {
                return Err(UnknownTripleError { contents: triple })
            };
        }

        macro_rules! next_identifier {
            () => {
                if let Some(next) = identifiers.next() {
                    next
                } else {
                    fail!()
                }
            };
        }

        let architecture = match next_identifier!() {
            "aarch64" => Architecture::AArch64,
            "mips" => Architecture::MIPS,
            "wasm32" => Architecture::Wasm32,
            "wasm64" => Architecture::Wasm64,
            "i686" => Architecture::X86,
            "x86_64" => Architecture::X86_64,
            _ => fail!(),
        };

        let vendor = match next_identifier!() {
            "unknown" => Vendor::Unknown,
            "pc" => Vendor::PC,
            _ => fail!(),
        };

        let operating_system = match next_identifier!() {
            "unknown" => OperatingSystem::Unknown,
            "none" => OperatingSystem::None,
            "ios" => OperatingSystem::IOS,
            "linux" => OperatingSystem::Linux,
            "macosx" => OperatingSystem::MacOSX,
            "wasi" => OperatingSystem::WASI,
            "windows" => OperatingSystem::Windows,
            _ => fail!(),
        };

        let environment = match identifiers.next() {
            Some("unknown") | None => Environment::Unknown,
            Some("gnu") => Environment::GNU,
            Some("musl") => Environment::MUSL,
            Some("msvc") => Environment::MSVC,
            Some("coreclr") => Environment::CoreCLR,
            Some(_) => fail!(),
        };

        if identifiers.next().is_some() {
            fail!()
        }

        Ok(Self::with_environment(
            architecture,
            vendor,
            operating_system,
            environment,
        ))
    }
}

impl Display for KnownTriple {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}-{}-{}",
            self.architecture(),
            self.vendor(),
            self.operating_system()
        )?;

        match self.environment() {
            Environment::Unknown => Ok(()),
            environment => write!(f, "-{}", environment),
        }
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

impl Triple {
    /// Returns the LLVM triple string for this target triple, returning an error if a custom triple string is used that contains
    /// a null byte.
    pub fn to_triple_string(&self) -> Result<Identifier, identifier::Error> {
        Identifier::try_from(self.to_string())
    }
}

impl std::default::Default for Triple {
    /// A target triple whose components are all unknown.
    fn default() -> Self {
        Self::Known(KnownTriple::default())
    }
}

impl From<KnownTriple> for Triple {
    fn from(triple: KnownTriple) -> Self {
        Self::Known(triple)
    }
}

impl From<&'_ Id> for Triple {
    fn from(triple: &Id) -> Self {
        match KnownTriple::try_from(triple) {
            Ok(known) => Self::Known(known),
            Err(_) => Triple::Custom(triple.into()),
        }
    }
}

impl From<Identifier> for Triple {
    fn from(triple: Identifier) -> Self {
        match KnownTriple::try_from(triple.as_id()) {
            Ok(known) => Self::Known(known),
            Err(_) => Triple::Custom(triple),
        }
    }
}

impl Display for Triple {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Custom(triple) => Display::fmt(&triple, f),
            Self::Known(triple) => Display::fmt(&triple, f),
        }
    }
}

pub mod layout;

pub use layout::Layout;

/// An `LLVMCodeGenOptLevel`, which indicates the level of optimization to use during code generation.
#[derive(Copy, Clone, Debug)]
pub enum CodeGenerationOptimization {
    /// Specifies that optimizations should be disabled, corresponds to `-O0`.
    None,
    /// Allows optimizations that preserve the ability to debug the program, corresponds to `-O1`.
    Less,
    /// The default optimization level, optimizing for fast execution without significant compile times, corresponds to `-O2`.
    Default,
    /// Optimizes for fast execution, corresponds to `-O3`.
    Aggressive,
}

crate::enum_default!(CodeGenerationOptimization, Default);

/// An `LLVMRelocMode`, which specifies the if and how code is relocated.
#[derive(Copy, Clone, Debug)]
pub enum RelocationMode {
    /// Some default mode.
    Default,
    /// Might refer to code that expects to be loaded at a certain address
    Static,
    /// Position-Independent Code.
    PIC,
    /// No idea what this means.
    DynamicNoPIC,
    /// Read-Only Position Independence, used in embedded systems.
    ROPI,
    /// Read-Write Position Independence, used in embedded systems.
    RWPI,
    /// Relocation mode used for embedded systems.
    ROPIRWPI,
}

crate::enum_default!(RelocationMode, Default);

/// An `LLVMCodeModel`.
///
/// According to <https://stackoverflow.com/questions/40493448/what-does-the-codemodel-in-clang-llvm-refer-to#40498306>,
/// this provides "restrictions on the relative location of code and data".
#[derive(Copy, Clone, Debug)]
pub enum CodeModel {
    /// D
    Default,
    /// Default used for Just-in-Time compiled code.
    JITDefault,
    /// No idea what tiny will do, might really force things to be close together.
    Tiny,
    /// Safe to use for static code, and might be the default value.
    Small,
    /// Your guess is as good as mine.
    Kernel,
    /// Good if JITing or if ASLR is enabled?
    Medium,
    /// Seems to be a good value if data and code is far away.
    Large,
}

crate::enum_default!(CodeModel, Default);

/// Represents an LLVM target machine.
#[derive(Clone, Debug)]
pub struct Machine {
    triple: Triple,
    cpu_name: Identifier,
    features: Identifier,
    optimization_level: CodeGenerationOptimization,
    relocation_mode: RelocationMode,
    code_model: CodeModel,
}

impl Machine {
    /// Creates a new target machine.
    pub fn new(
        triple: Triple,
        cpu_name: Identifier,
        features: Identifier,
        optimization_level: CodeGenerationOptimization,
        relocation_mode: RelocationMode,
        code_model: CodeModel,
    ) -> Self {
        Self {
            triple,
            cpu_name,
            features,
            optimization_level,
            relocation_mode,
            code_model,
        }
    }

    /// Creates a new target machine using the default optimization level, relocation mode, and code model.
    pub fn with_defaults(triple: Triple, cpu_name: Identifier, features: Identifier) -> Self {
        Self::new(
            triple,
            cpu_name,
            features,
            CodeGenerationOptimization::default(),
            RelocationMode::default(),
            CodeModel::default(),
        )
    }

    /// Gets the target triple for this target machine.
    pub fn target_triple(&self) -> &Triple {
        &self.triple
    }

    /// Gets the CPU name of the target machine.
    pub fn cpu_name(&self) -> &Id {
        self.cpu_name.as_id()
    }

    /// A string describing additional features of the target machine.
    pub fn features(&self) -> &Id {
        self.features.as_id()
    }

    /// Gets a value indicating how much code is optimized for this target machine.
    pub fn code_generation_optimization_level(&self) -> CodeGenerationOptimization {
        self.optimization_level
    }

    /// Indicates how code is relocated in this target machine.
    pub fn relocation_mode(&self) -> RelocationMode {
        self.relocation_mode
    }

    /// Gets the code model value used for this target machine.
    pub fn code_model(&self) -> CodeModel {
        self.code_model
    }
}

/// A target machine and layout, fully describing the host that will run a module's code.
#[derive(Clone, Debug)]
pub struct Target {
    machine: Machine,
    layout: Layout,
}

impl Target {
    /// Creates a target to describe a host from a target machine and layout.
    pub fn new(machine: Machine, layout: Layout) -> Self {
        Self { machine, layout }
    }

    /// Gets the target triple for this target.
    pub fn triple(&self) -> &Triple {
        self.machine().target_triple()
    }

    /// Gets the target machine.
    pub fn machine(&self) -> &Machine {
        &self.machine
    }

    /// Gets the target layout.
    pub fn layout(&self) -> &Layout {
        &self.layout
    }
}
