//! Contains structures used to specify the layout of data for an LLVM target triple.

use crate::identifier::{Identifier, Id};
use std::borrow::Cow;
use std::collections::hash_map;
use std::fmt::{Debug, Display, Formatter, Write as _};
use std::num::{NonZeroU32, NonZeroU8};

/// Specifies whether data is laid out in big-endian or little-endian form.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Endianness {
    /// The least signficiant bits have the lowest address (`0xABCD = 0xCD 0xAB`).
    Little,
    /// The least significant bits have the highest address (`0xABCD = 0xAB 0xCD`).
    Big,
}

impl Display for Endianness {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_char(match self {
            Self::Little => 'e',
            Self::Big => 'E',
        })
    }
}

/// An LLVM address space.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AddressSpace(pub u32);

impl AddressSpace {
    /// The LLVM address space `0`, which corresponds to a Von-Neumann architecture where code and data are in the same address
    /// space.
    pub const VON_NEUMANN_DEFAULT: Self = Self(0);
}

/// Specifies the size of an integer or pointer, in bits.
#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BitSize {
    bits: NonZeroU32,
}

impl BitSize {
    /// 1-bit, used in LLVM for boolean values.
    pub const SIZE_1: Self = Self {
        bits: unsafe { NonZeroU32::new_unchecked(1) },
    };

    /// 8 bits, or 1 byte.
    pub const SIZE_8: Self = Self {
        bits: unsafe { NonZeroU32::new_unchecked(8) },
    };

    /// 16 bits, or 2 bytes.
    pub const SIZE_16: Self = Self {
        bits: unsafe { NonZeroU32::new_unchecked(16) },
    };

    /// 32 bits, or 4 bytes.
    pub const SIZE_32: Self = Self {
        bits: unsafe { NonZeroU32::new_unchecked(32) },
    };

    /// 64 bits, or 8 bytes.
    pub const SIZE_64: Self = Self {
        bits: unsafe { NonZeroU32::new_unchecked(64) },
    };

    /// 128 bits, or 16 bytes.
    pub const SIZE_128: Self = Self {
        bits: unsafe { NonZeroU32::new_unchecked(128) },
    };

    /// Creates a size from a value, in bytes.
    pub fn from_bytes(size: NonZeroU8) -> Self {
        Self {
            bits: // Safety: size is guaranteed to be non-zero.
                unsafe { NonZeroU32::new_unchecked(u32::from(size.get()) * 8) }
        }
    }

    /// Gets the size, in bits.
    pub fn bits(self) -> NonZeroU32 {
        self.bits
    }

    fn unwrap_bits(size: Option<Self>) -> u32 {
        size.map(|value| value.bits.get()).unwrap_or_default()
    }
}

impl Debug for BitSize {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(&self.bits(), f)
    }
}

/// Specifies an ABI and an optional preferred alignment. If the preferred alignment is omitted, the ABI alignment is used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlignmentPair {
    // TODO: How to allow ABI alignment of 0
    abi: Option<BitSize>,
    preferred: Option<BitSize>,
}

impl AlignmentPair {
    /// An ABI alignment value of 64 bits, with an omitted preferred alignment.
    pub const ALIGN_64_BITS: Self = Self::new(BitSize::SIZE_64);

    /// Creates a new alignment value, omitting the preferred alignment value.
    pub const fn new(abi_alignment: BitSize) -> Self {
        Self {
            abi: Some(abi_alignment),
            preferred: None,
        }
    }

    /// Creates a new alignment value.
    pub const fn with_preferred_alignment(
        abi_alignment: BitSize,
        preferred_alignment: BitSize,
    ) -> Self {
        Self {
            abi: Some(abi_alignment),
            preferred: Some(preferred_alignment),
        }
    }

    /// Creates a new alignment value, with an ABI alignment of zero.
    pub const fn with_preferred_only(preferred_alignment: BitSize) -> Self {
        Self {
            abi: None,
            preferred: Some(preferred_alignment),
        }
    }

    /// Indicates if the preferred alignment value is omitted.
    pub const fn is_preferred_omitted(&self) -> bool {
        self.preferred.is_none()
    }

    /// Gets the ABI alignment value, in bits.
    pub fn abi_alignment(&self) -> u32 {
        BitSize::unwrap_bits(self.abi)
    }

    /// Gets the preferred alignment value in bits, defaulting to the ABI alignment if the former is omitted.
    pub fn preferred_alignment(&self) -> u32 {
        self.preferred
            .map(|size| size.bits().get())
            .unwrap_or(self.abi_alignment())
    }
}

/// Specifies the layout of a pointer in memory for a particular address space.
#[derive(Clone, Debug)]
pub struct PointerLayout {
    address_space: AddressSpace,
    size: BitSize,
    alignment: AlignmentPair,
    index_size: Option<BitSize>,
}

impl PointerLayout {
    /// A 64-bit pointer that is 64-bit aligned.
    pub const LAYOUT_64_BIT: Self = Self {
        address_space: AddressSpace::VON_NEUMANN_DEFAULT,
        size: BitSize::SIZE_64,
        alignment: AlignmentPair::ALIGN_64_BITS,
        index_size: None,
    };

    /// Retrieves the address space that this pointer layout applies to.
    pub const fn address_space(&self) -> AddressSpace {
        self.address_space
    }

    /// Gets the size of pointers, in bits.
    pub const fn size(&self) -> BitSize {
        self.size
    }

    /// Gets the alignment of pointers.
    pub const fn alignment(&self) -> &AlignmentPair {
        &self.alignment
    }

    /// Gets the index size, which defaults to the pointer size if it is unspecified.
    pub fn index_size(&self) -> BitSize {
        self.index_size.unwrap_or(self.size)
    }
}

/// Describes the layout of pointers for a particular address space.
#[derive(Clone, Debug)]
pub struct PointerLayoutMap {
    layouts: hash_map::HashMap<AddressSpace, PointerLayout>,
}

impl PointerLayoutMap {
    /// The default pointer layouts used by LLVM, where pointers in all address spaces have the same layout as a 64-bit pointer
    /// in the default address space.
    pub fn all_default() -> Self {
        Self {
            layouts: hash_map::HashMap::default(),
        }
    }

    /// Gets a value indicating if the default pointer layouts is being used, meaning that pointers in all address spaces have
    /// the same layout as the pointer in the default address space.
    pub fn is_all_default(&self) -> bool {
        self.layouts.is_empty()
    }

    /// Creates a pointer layout from a single layout value.
    pub fn from_layout(layout: PointerLayout) -> Self {
        let mut layouts = std::collections::HashMap::with_capacity(1);
        layouts.insert(layout.address_space, layout);
        Self { layouts }
    }

    /// Inserts a pointer layout for a particular address space.
    pub fn insert(&mut self, layout: PointerLayout) -> Result<&PointerLayout, PointerLayout> {
        if self.is_all_default() {
            Ok(&PointerLayout::LAYOUT_64_BIT)
        } else {
            match self.layouts.entry(layout.address_space) {
                hash_map::Entry::Vacant(vacant) => Ok(vacant.insert(layout)),
                hash_map::Entry::Occupied(occupied) => Err(occupied.get().clone()),
            }
        }
    }

    /// Gets the pointer layout used for the given address space.
    pub fn get(&self, address_space: AddressSpace) -> Option<&PointerLayout> {
        self.layouts.get(&address_space)
    }
}

/// Describes the alignment for integer, vector, or floating-point types of particular sizes.
///
/// For floating-point type layouts, sizes of 32 or 64 bits are supported on all targets, while more exotic targets may not be
/// supported.
#[derive(Clone, Debug)]
pub struct PrimitiveAlignmentMap {
    layouts: hash_map::HashMap<BitSize, AlignmentPair>,
}

lazy_static::lazy_static! {
    static ref INTEGER_ALIGNMENT_DEFAULTS: PrimitiveAlignmentMap = PrimitiveAlignmentMap {
        layouts: hash_map::HashMap::from([
            (BitSize::SIZE_1, AlignmentPair::new(BitSize::SIZE_8)),
            (BitSize::SIZE_8, AlignmentPair::new(BitSize::SIZE_8)),
            (BitSize::SIZE_16, AlignmentPair::new(BitSize::SIZE_16)),
            (BitSize::SIZE_32, AlignmentPair::new(BitSize::SIZE_32)),
            (BitSize::SIZE_64, AlignmentPair::new(BitSize::SIZE_64)),
        ])
    };

    static ref FLOAT_ALIGNMENT_DEFAULTS: PrimitiveAlignmentMap = PrimitiveAlignmentMap {
        layouts: hash_map::HashMap::from([
            (BitSize::SIZE_16, AlignmentPair::new(BitSize::SIZE_16)),
            (BitSize::SIZE_32, AlignmentPair::new(BitSize::SIZE_32)),
            (BitSize::SIZE_64, AlignmentPair::new(BitSize::SIZE_64)),
            (BitSize::SIZE_128, AlignmentPair::new(BitSize::SIZE_128)),
        ])
    };

    static ref VECTOR_ALIGNMENT_DEFAULTS: PrimitiveAlignmentMap = PrimitiveAlignmentMap {
        layouts: hash_map::HashMap::from([
            (BitSize::SIZE_64, AlignmentPair::new(BitSize::SIZE_64)),
            (BitSize::SIZE_128, AlignmentPair::new(BitSize::SIZE_128)),
        ])
    };
}

impl PrimitiveAlignmentMap {
    /// The default alignment values used for integers.
    pub fn integer_defaults() -> &'static Self {
        &INTEGER_ALIGNMENT_DEFAULTS
    }

    /// The default alignment values used for floating-point types.
    pub fn float_defaults() -> &'static Self {
        &FLOAT_ALIGNMENT_DEFAULTS
    }

    /// The default alignment values used for vectors.
    pub fn vector_defaults() -> &'static Self {
        &VECTOR_ALIGNMENT_DEFAULTS
    }

    /// Gets the alignment for a value of a particular size.
    pub fn get(&self, size: BitSize) -> Option<&AlignmentPair> {
        self.layouts.get(&size)
    }
}

/// Indicates the type of alignment used for function pointers.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum FunctionAlignmentType {
    /// Indicates that the alignment of function pointers is independent of functions.
    Independent,
    /// Indicates that the alignment of function pointers is a multiple of the alignment for functions.
    Multiple,
}

/// Describes the alignment of function pointers.
#[derive(Clone, Debug)]
pub struct FunctionAlignment {
    alignment_type: FunctionAlignmentType,
    abi_alignment: BitSize,
}

impl FunctionAlignment {
    /// Creates a new function alignment value.
    pub const fn new(alignment_type: FunctionAlignmentType, abi_alignment: BitSize) -> Self {
        Self {
            alignment_type,
            abi_alignment,
        }
    }

    /// Gets a value indicating how function pointers are aligned.
    pub const fn alignment_type(&self) -> FunctionAlignmentType {
        self.alignment_type
    }

    /// The alignment for function pointers.
    pub const fn abi_alignment(&self) -> BitSize {
        self.abi_alignment
    }
}

/// Indicates how symbols are mangled.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Mangling {
    /// The Executable and Linkable Format used in Unix-like systems, which uses the prefix `.L` for private symbols.
    ELF,
    /// IBM's Generalized Object File Format, which uses the prefix `@` for private symbols.
    GOFF,
    /// `$`
    MIPS,
    /// Apple's Mach object file format, which uses the prefix `L` for private symbols.
    MachO,
    /// See LLVM documentation for more information.
    WindowsX86COFF,
    /// Similar to [`Mangling::WindowsX86COFF`].
    WindowsCOFF,
    /// A `L..` prefix is used for private symbols.
    XCOFF,
}

/// Indicates how data is laid out in memory for a specific target.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Layout {
    /// Specifies the byte endianness of the target.
    pub endianness: Endianness,
    /// Specifies the natual stack alignment.
    pub stack_alignment: Option<BitSize>,
    /// Specifies which address space corresponds to program memory.
    pub program_address_space: AddressSpace,
    /// Specifies which address space corresponds to program memory.
    pub global_address_space: AddressSpace,
    /// Specifies the address space used by the `alloca` instruction.
    pub alloca_address_space: AddressSpace,
    /// Indicates the layout of pointers for certain address spaces.
    pub pointer_layouts: PointerLayoutMap,
    /// Indicates how integers of certain sizes are aligned.
    pub integer_alignments: PrimitiveAlignmentMap,
    /// Indicates how vectors of certain sizes are aligned.
    pub vector_alignments: PrimitiveAlignmentMap,
    /// Indicates how floating-point types of certain sizes are aligned.
    pub float_alignments: PrimitiveAlignmentMap,
    /// Specifies the alignment for aggregate types.
    pub aggregate_object_alignment: AlignmentPair,
    /// Indicates how function pointers are aligned.
    pub function_pointer_alignment: Option<FunctionAlignment>,
    /// Specifies how symbol names are mangled in the output.
    pub mangling: Option<Mangling>,
    /// Indicates the native integer widths for the target CPU.
    pub native_integer_widths: Vec<BitSize>,
    //pub non_integral_pointer_types: ,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            endianness: Endianness::Little,
            stack_alignment: None,
            program_address_space: AddressSpace::VON_NEUMANN_DEFAULT,
            global_address_space: AddressSpace::VON_NEUMANN_DEFAULT,
            alloca_address_space: AddressSpace::VON_NEUMANN_DEFAULT,
            pointer_layouts: PointerLayoutMap::all_default(),
            integer_alignments: PrimitiveAlignmentMap::integer_defaults().clone(),
            vector_alignments: PrimitiveAlignmentMap::vector_defaults().clone(),
            float_alignments: PrimitiveAlignmentMap::float_defaults().clone(),
            aggregate_object_alignment: AlignmentPair::with_preferred_only(BitSize::SIZE_64),
            function_pointer_alignment: None,
            mangling: None,
            native_integer_widths: Vec::default(),
        }
    }
}

/// Error used when a layout could not be parsed.
#[derive(Clone, Debug, thiserror::Error)]
#[error("{message}")]
pub struct ParseError<'a> {
    message: Cow<'a, str>,
}

impl<'a> TryFrom<&'a Id> for Layout {
    type Error = ParseError<'a>;

    fn try_from(layout: &'a Id) -> Result<Self, Self::Error> {
        let mut specifications = layout.split('-');
        let mut layout = Self::default();

        todo!("parse specifications");

        Ok(layout)
    }
}

impl TryFrom<Identifier> for Layout {
    type Error = ParseError<'static>;

    fn try_from(layout: Identifier) -> Result<Self, Self::Error> {
        Self::try_from(layout.as_id()).map_err(|error| ParseError {
            message: Cow::Owned(error.message.into_owned()),
        })
    }
}
