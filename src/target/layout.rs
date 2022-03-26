//! Contains structures used to specify the layout of data for an LLVM target triple.

use crate::identifier::{Id, Identifier};
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
            .unwrap_or_else(|| self.abi_alignment())
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
    ///
    /// Prefer using `PointerLayoutMap::get_or_default` for determining the pointer layout for a given address space.
    pub fn get(&self, address_space: AddressSpace) -> Option<&PointerLayout> {
        self.layouts.get(&address_space)
    }

    /// Gets the pointer layout used for a given address space, returning the default layout value if it is not specified.
    pub fn get_or_default(&self, address_space: AddressSpace) -> &PointerLayout {
        self.get(address_space)
            .unwrap_or(&PointerLayout::LAYOUT_64_BIT)
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

    /// Inserts alignment values corresponding to a particular size.
    pub fn try_insert(
        &mut self,
        size: BitSize,
        alignment: AlignmentPair,
    ) -> Result<&AlignmentPair, AlignmentPair> {
        match self.layouts.entry(size) {
            hash_map::Entry::Vacant(vacant) => Ok(vacant.insert(alignment)),
            hash_map::Entry::Occupied(occupied) => Err(occupied.get().clone()),
        }
    }

    /// Inserts an alignment value for a particular size, overwritting any previous value.
    pub fn insert_or_replace(&mut self, size: BitSize, alignment: AlignmentPair) {
        self.layouts.insert(size, alignment);
    }

    /// Gets the alignment for a value of a particular size.
    pub fn get(&self, size: BitSize) -> Option<&AlignmentPair> {
        self.layouts.get(&size)
    }

    // TODO: See https://llvm.org/docs/LangRef.html#data-layout point 2 for rules regarding alignment for a type if it is not explicitly set.
    // TODO: May need helpers to determine smallest, largest, and nearest values.
    //pub fn get_or_default(&self, size: BitSize) -> &AlignmentPair {  }
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

// TODO: How to enforce multiple of 8 bits for some values, such as stack alignment?
// pub struct ByteSize

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
#[non_exhaustive]
pub enum ParseError {
    /// Used when an unknown specification was parsed.
    #[error("'{0}' is not a valid specification")]
    InvalidSpecification(char),
    /// Used when an integer could not be parsed.
    #[error(transparent)]
    InvalidInteger(#[from] std::num::ParseIntError),
    /// Used when the specification ends after a `:`.
    #[error("missing information after colon")]
    MissingInformation,
    /// Used when remaining characters in a specification could not be parsed.
    #[error("expected end, but got {0}")]
    ExpectedEnd(String),
    /// Used when more than one `p` specification for a particular address space.
    #[error("duplicate pointer layout specified for address space {0:?}")]
    DuplicatePointerLayout(AddressSpace),
    /// Used when a non-zero size was expected in a particular specification.
    #[error("expected non-zero size value in specification '{0}'")]
    ExpectedNonZeroSize(char),
    /// Used when an `m` specification exists that did not specify any option.
    #[error("a mangling specification exists but did not specify any option")]
    MissingManglingValue,
    /// Used when an `m` specification uses an invalid option.
    #[error("{0} is not a valid mangling specification option")]
    InvalidManglingValue(char),
    /// Used when an `i`, `v`, or `f` specification is duplicated for a particular size.
    #[error("duplicate '{specification}' specification for size {size:?}")]
    DuplicatePrimitiveAlignment {
        /// The duplicated specification.
        specification: char,
        /// The duplicate size value.
        size: BitSize,
    },
    /// Used when a specification string is empty.
    #[error("specifications must not be empty")]
    EmptySpecification,
}

impl TryFrom<&Id> for Layout {
    type Error = ParseError;

    fn try_from(layout: &Id) -> Result<Self, Self::Error> {
        let mut specifications = layout.split('-');
        let mut layout = Self::default();

        // TODO: Check for some duplicate specifications.

        type ParseResult<'a, T> = Result<(&'a [char], T), ParseError>;

        fn parse_integer<T: std::str::FromStr<Err = std::num::ParseIntError>>(
            s: &[char],
        ) -> ParseResult<'_, T> {
            let mut digits = String::new();
            let mut parse_count = 0;

            for d in s.iter().take_while(|c| c.is_ascii_digit()) {
                digits.push(*d);
                parse_count += 1;
            }

            let value = T::from_str(&digits)?;

            Ok((&s[parse_count..], value))
        }

        fn parse_bit_size(s: &[char]) -> ParseResult<Option<BitSize>> {
            let (remaining, value) = parse_integer::<u32>(s)?;
            Ok((
                remaining,
                NonZeroU32::new(value).map(|bits| BitSize { bits }),
            ))
        }

        fn parse_address_space(s: &[char]) -> ParseResult<AddressSpace> {
            let (remaining, value) = parse_integer::<u32>(s)?;
            Ok((remaining, AddressSpace(value)))
        }

        fn parse_information<T, P: FnOnce(&[char]) -> ParseResult<T>>(
            parser: P,
            s: &[char],
        ) -> ParseResult<Option<T>> {
            match s.first() {
                Some(':') => {
                    let (remaining, value) = parser(&s[1..])?;
                    Ok((remaining, Some(value)))
                }
                Some(_) => Err(ParseError::ExpectedEnd(s.iter().skip(1).collect())),
                None => Ok((&[], None)),
            }
        }

        fn parse_information_or<
            T,
            P: FnOnce(&[char]) -> ParseResult<T>,
            E: FnOnce() -> ParseError,
        >(
            parser: P,
            error: E,
            s: &[char],
        ) -> ParseResult<T> {
            match parse_information(parser, s)? {
                (remaining, Some(value)) => Ok((remaining, value)),
                (_, None) => Err(error()),
            }
        }

        fn parse_primitive_alignment<'a>(
            specification: char,
            lookup: &mut PrimitiveAlignmentMap,
            s: &'a [char],
        ) -> ParseResult<'a, ()> {
            let (remaining, size) = parse_bit_size(s)?;
            let (remaining, abi) =
                parse_information_or(parse_bit_size, || ParseError::MissingInformation, remaining)?;
            let (remaining, pref) = parse_information(parse_bit_size, remaining)?;

            // TODO: Better way to replace duplicate primitive alignment.
            lookup.insert_or_replace(
                size.ok_or_else(|| ParseError::ExpectedNonZeroSize(specification))?,
                AlignmentPair {
                    abi,
                    preferred: pref.flatten(),
                },
            );

            Ok((remaining, ()))
        }

        fn parse_specification(layout: &mut Layout, s: &[char]) -> Result<(), ParseError> {
            if let Some(kind) = s.first() {
                let information = &s[1..];

                macro_rules! set_address_space {
                    ($name: ident) => {{
                        let (remaining, address_space) = parse_address_space(information)?;
                        layout.$name = address_space;
                        remaining
                    }};
                }

                let remaining = match kind {
                    'E' => {
                        layout.endianness = Endianness::Big;
                        &s[1..]
                    }
                    'e' => {
                        layout.endianness = Endianness::Little;
                        &s[1..]
                    }
                    'S' => {
                        let (remaining, alignment) = parse_bit_size(information)?;
                        layout.stack_alignment = alignment;
                        remaining
                    }
                    'P' => set_address_space!(program_address_space),
                    'G' => set_address_space!(global_address_space),
                    'A' => set_address_space!(alloca_address_space),
                    'p' => {
                        // Peek to see if an address space is specified.
                        let (remaining, address_space) = match information.first() {
                            Some(':') => (information, AddressSpace::VON_NEUMANN_DEFAULT),
                            Some(_) => parse_address_space(information)?,
                            _ => return Err(ParseError::MissingInformation),
                        };

                        let (remaining, size) = parse_information_or(
                            parse_bit_size,
                            || ParseError::MissingInformation,
                            remaining,
                        )?;
                        let (remaining, abi) = parse_information_or(
                            parse_bit_size,
                            || ParseError::MissingInformation,
                            remaining,
                        )?;
                        let (remaining, pref) = parse_information(parse_bit_size, remaining)?;
                        let (remaining, idx) = parse_information(parse_bit_size, remaining)?;

                        match layout.pointer_layouts.insert(PointerLayout {
                            address_space,
                            alignment: AlignmentPair {
                                abi,
                                preferred: pref.flatten(),
                            },
                            size: size.ok_or_else(|| ParseError::ExpectedNonZeroSize('p'))?,
                            index_size: idx.flatten(),
                        }) {
                            Ok(_) => remaining,
                            Err(_) => {
                                return Err(ParseError::DuplicatePointerLayout(address_space))
                            }
                        }
                    }
                    'i' => {
                        let (remaining, ()) = parse_primitive_alignment(
                            'i',
                            &mut layout.integer_alignments,
                            information,
                        )?;
                        remaining
                    }
                    'v' => {
                        let (remaining, ()) = parse_primitive_alignment(
                            'i',
                            &mut layout.vector_alignments,
                            information,
                        )?;
                        remaining
                    }
                    'f' => {
                        let (remaining, ()) = parse_primitive_alignment(
                            'i',
                            &mut layout.float_alignments,
                            information,
                        )?;
                        remaining
                    }
                    //'a'
                    //'F'
                    'm' => {
                        let (remaining, mangling) = parse_information_or(
                            |s| {
                                if let Some(mangling_option) = s.first() {
                                    let remaining = &s[1..];
                                    match mangling_option {
                                        'e' => Ok((remaining, Mangling::ELF)),
                                        'l' => Ok((remaining, Mangling::GOFF)),
                                        'm' => Ok((remaining, Mangling::MIPS)),
                                        'o' => Ok((remaining, Mangling::MachO)),
                                        'x' => Ok((remaining, Mangling::WindowsX86COFF)),
                                        'w' => Ok((remaining, Mangling::WindowsCOFF)),
                                        'a' => Ok((remaining, Mangling::XCOFF)),
                                        _ => {
                                            Err(ParseError::InvalidManglingValue(*mangling_option))
                                        }
                                    }
                                } else {
                                    Err(ParseError::MissingManglingValue)
                                }
                            },
                            || ParseError::MissingManglingValue,
                            information,
                        )?;

                        layout.mangling = Some(mangling);
                        remaining
                    }
                    _ => return Err(ParseError::InvalidSpecification(*kind)),
                };

                if remaining.is_empty() {
                    Ok(())
                } else {
                    Err(ParseError::ExpectedEnd(remaining.iter().collect()))
                }
            } else {
                Err(ParseError::EmptySpecification)
            }
        }

        let mut buffer: Vec<char> = Vec::new();

        while let Some(spec) = specifications.next() {
            buffer.clear();
            buffer.extend(spec.chars());
            parse_specification(&mut layout, &buffer)?;
        }

        Ok(layout)
    }
}

impl TryFrom<Identifier> for Layout {
    type Error = ParseError;

    fn try_from(layout: Identifier) -> Result<Self, Self::Error> {
        Self::try_from(layout.as_id())
    }
}
