//! Contains structures used to specify the layout of data for an LLVM target triple.

use std::collections::hash_map;
use std::fmt::{Debug, Display, Formatter, Write as _};
use std::num::{NonZeroU16, NonZeroU8};

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

/// Indicates the alignment of the stack, in bits.
#[derive(Copy, Clone, Default, Eq, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StackAlignment {
    bytes: Option<NonZeroU8>,
}

impl StackAlignment {
    /// Creates a stack alignment value from a size, in bytes.
    pub const fn from_bytes(size: NonZeroU8) -> Self {
        Self { bytes: Some(size) }
    }

    /// Gets the stack size, in bytes.
    pub const fn bytes(self) -> Option<NonZeroU8> {
        self.bytes
    }

    /// Gets the stack size, in bits.
    pub fn bits(self) -> Option<NonZeroU16> {
        self.bytes.map(|bytes| unsafe {
            // Safety: bytes is guaranteed to not be zero.
            NonZeroU16::new_unchecked(u16::from(bytes.get()) * 8)
        })
    }
}

impl Debug for StackAlignment {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(&self.bits().map(NonZeroU16::get).unwrap_or_default(), f)
    }
}

impl Display for StackAlignment {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_char('S')?;
        if let Some(bit_size) = self.bits() {
            Display::fmt(&bit_size, f)?;
        }
        Ok(())
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
    bytes: NonZeroU8,
}

impl BitSize {
    /// 64-bits, or 8 bytes.
    pub const SIZE_64: Self = Self {
        bytes: unsafe { NonZeroU8::new_unchecked(8) },
    };

    /// Creates a size from a value, in bytes.
    pub const fn from_bytes(size: NonZeroU8) -> Self {
        Self { bytes: size }
    }

    /// Gets the size, in bytes.
    pub const fn bytes(self) -> NonZeroU8 {
        self.bytes
    }

    /// Gets the size, in bits.
    pub fn bits(self) -> NonZeroU16 {
        unsafe {
            // Safety: bytes is guaranteed to not be zero.
            NonZeroU16::new_unchecked(u16::from(self.bytes.get()) * 8)
        }
    }
}

impl Debug for BitSize {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(&self.bits(), f)
    }
}

/// Specifies an ABI and an optional preferred alignment. If the preferred alignment is omitted, the ABI alignment is used.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AlignmentPair {
    abi: BitSize,
    preferred: Option<BitSize>,
}

impl AlignmentPair {
    pub const ALIGN_64_BITS: Self = Self::new(BitSize::SIZE_64);

    /// Creates a new alignment value, omitting the preferred alignment value.
    pub const fn new(abi_alignment: BitSize) -> Self {
        Self {
            abi: abi_alignment,
            preferred: None,
        }
    }

    /// Creates a new alignment value, omitting the preferred alignment value.
    pub const fn with_preferred_alignment(
        abi_alignment: BitSize,
        preferred_alignment: BitSize,
    ) -> Self {
        Self {
            abi: abi_alignment,
            preferred: Some(preferred_alignment),
        }
    }

    /// Indicates if the preferred alignment value is omitted.
    pub const fn is_preferred_omitted(&self) -> bool {
        self.preferred.is_none()
    }

    /// Gets the ABI alignment value.
    pub const fn abi_alignment(&self) -> BitSize {
        self.abi
    }

    /// Gets the preferred alignment value, defaulting to the ABI alignment if the former is omitted.
    pub const fn preferred_alignment(&self) -> BitSize {
        self.preferred.unwrap_or(self.abi)
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
    pub fn insert(&mut self, layout: PointerLayout) -> Result<&PointerLayout, &PointerLayout> {
        if self.is_all_default() {
            Err(&PointerLayout::LAYOUT_64_BIT)
        } else {
            match self.layouts.entry(layout.address_space) {
                hash_map::Entry::Vacant(vacant) => Ok(vacant.insert(layout)),
                hash_map::Entry::Occupied(occupied) => Err(occupied.get()),
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

impl PrimitiveAlignmentMap {
    /// Gets the alignment for a value of a particular size.
    pub fn get(&self, size: BitSize) -> Option<AlignmentPair> {
        self.layouts.get(&size).copied()
    }
}

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum FunctionAlignmentType {
    Independent,
    Multiple,
}

#[derive(Clone, Debug)]
pub struct FunctionAlignment {
    alignment_type: FunctionAlignmentType,
    abi_alignment: BitSize,
}

impl FunctionAlignment {
    pub const fn new(alignment_type: FunctionAlignmentType, abi_alignment: BitSize) -> Self {
        Self {
            alignment_type,
            abi_alignment,
        }
    }
}

/// Indicates how private symbols are mangled.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Mangling {
    /// The Executable and Linkable Format used in Unix-like systems, using the prefix `.L`.
    ELF,
    /// IBM's Generalized Object File Format, which uses the prefix `@`.
    GOFF,
    /// `$`
    MIPS,
    /// Apple's Mach object file format, which uses the prefix `L`.
    MachO,
    WindowsX86COFF,
    WindowsCOFF,
    XCOFF,
}

/// Indicates how data is laid out in memory for a specific target.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Layout {
    /// Specifies the byte endianness of the target.
    pub endianness: Endianness,
    /// Specifies the natual stack alignment in bits.
    pub stack_alignment: StackAlignment,
    /// Specifies which address space corresponds to program memory.
    pub program_address_space: AddressSpace,
    /// Specifies which address space corresponds to program memory.
    pub global_address_space: AddressSpace,
    /// Specifies the address space used by the `alloca` instruction.
    pub alloca_address_space: AddressSpace,
    pub pointer_layouts: PointerLayoutMap,
    pub integer_alignments: PrimitiveAlignmentMap,
    pub vector_alignments: PrimitiveAlignmentMap,
    pub float_alignments: PrimitiveAlignmentMap,
    pub aggregate_object_alignment: AlignmentPair,
    pub function_pointer_alignment: Option<FunctionAlignment>,
    pub mangling: Mangling,
    pub native_integer_widths: Vec<BitSize>,
    //pub non_integral_pointer_types: ,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            endianness: Endianness::Little,
            stack_alignment: StackAlignment::default(),
            function_pointer_alignment: None,
        }
    }
}
