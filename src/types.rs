//! Model of the LLVM type system.

use std::fmt::{Display, Formatter, Write as _};
use std::num::NonZeroU32;
use std::rc::Rc;

/// Represents the size of an integer, which can be a value from `1` to `2^23`.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IntegerSize(NonZeroU32);

impl IntegerSize {
    /// Creates a new size value, without any bounds checks.
    ///
    /// # Safety
    /// Callers must ensure that the size is in the range `1` to `2^23`.
    pub const unsafe fn new_unchecked(size: u32) -> Self {
        Self(NonZeroU32::new_unchecked(size))
    }

    /// Minimum size value.
    pub const MIN: Self = unsafe { Self::new_unchecked(1) };

    /// Size of a 1-byte integer, known as a byte.
    pub const SIZE_8: Self = unsafe { Self::new_unchecked(8) };

    /// Size of a 2-byte integer.
    pub const SIZE_16: Self = unsafe { Self::new_unchecked(16) };

    /// Size of a 4-byte integer.
    pub const SIZE_32: Self = unsafe { Self::new_unchecked(32) };

    /// Size of an 8-byte integer.
    pub const SIZE_64: Self = unsafe { Self::new_unchecked(64) };

    /// Gets the size, in bits.
    pub fn bits(self) -> u32 {
        self.0.get()
    }
}

impl Display for IntegerSize {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

/// Represents a floating-point type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Float {
    /// 16-bit, IEEE-754 `binary16`.
    Half,
    /// 32-bit, IEEE-754 `binary32`.
    Float,
    /// 64-bit, IEEE-754 `binary64`.
    Double,
}

impl Display for Float {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::Half => "half",
            Self::Float => "float",
            Self::Double => "double",
        })
    }
}

pub use crate::target::layout::AddressSpace;

/// A pointer type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pointer {
    pointee_type: Rc<FirstClass>, // TODO: Allow function pointers, have an enum PointeeType?
    address_space: AddressSpace,
}

impl Pointer {
    /// Creates a pointer type pointing to a objects of a particular type in a particular address space.
    pub fn in_address_space(pointee_type: Rc<FirstClass>, address_space: AddressSpace) -> Self {
        Self {
            pointee_type,
            address_space,
        }
    }

    /// Creates a pointer type pointing to objects of a particular type.
    pub fn new(pointee_type: Rc<FirstClass>) -> Self {
        Self::in_address_space(pointee_type, AddressSpace::VON_NEUMANN_DEFAULT)
    }

    /// The type of object that is pointed to by the pointer type.
    pub fn pointee_type(&self) -> &Rc<FirstClass> {
        &self.pointee_type
    }

    /// The address space of the pointer, indicating where the object pointed to resides.
    pub fn address_space(&self) -> AddressSpace {
        self.address_space
    }
}

impl Display for Pointer {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(&self.pointee_type, f)?;
        if self.address_space.0 != 0 {
            write!(f, " addrspace({})", self.address_space)?;
        }
        f.write_char('*')
    }
}

/// A vector of elements of a specified size.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Vector {
    element_type: Rc<FirstClass>,
    count: NonZeroU32,
    //vscale: bool,
}

impl Vector {
    /// Creates a vector type containing a specified number of elements of a specified type.
    pub fn new(element_type: Rc<FirstClass>, count: NonZeroU32) -> Self {
        Self {
            element_type,
            count,
        }
    }

    /// Gets the type of elements of this vector type.
    pub fn element_type(&self) -> &Rc<FirstClass> {
        &self.element_type
    }

    /// Gets the number of elements, guaranteed to be greater than zero.
    pub fn count(&self) -> u32 {
        // -> Result<u32, u32> // Error if vscale?
        self.count.get()
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "<{} x {}>", self.count(), &self.element_type)
    }
}

/// A subset of the types that are valid in registers.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SingleValue {
    /// An integer type of an arbitrary bit width.
    Integer(IntegerSize),
    /// Floating point type.
    Float(Float),
    /// A pointer type.
    Pointer(Pointer),
    /// A vector of elements of a specified size.
    Vector(Vector),
}

impl Display for SingleValue {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Integer(size) => write!(f, "i{}", size),
            Self::Float(float) => Display::fmt(float, f),
            Self::Pointer(pointer) => Display::fmt(pointer, f),
            Self::Vector(vector) => Display::fmt(vector, f),
        }
    }
}

/// Describes the type of value returned by a function.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Return {
    /// A type representing no value.
    Void,
    /// A return type.
    FirstClass(Rc<FirstClass>),
}

impl Display for Return {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::FirstClass(return_type) => Display::fmt(return_type, f),
            Self::Void => f.write_str("void"),
        }
    }
}

/// Represents a function type, which describes the return types and parameter types of a function.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Function {
    return_type: Return,
    parameter_types: Vec<Rc<FirstClass>>,
}

impl Function {
    /// Creates a function type.
    pub fn new(return_type: Return, parameter_types: impl Into<Vec<Rc<FirstClass>>>) -> Self {
        Self {
            return_type,
            parameter_types: parameter_types.into(),
        }
    }

    /// Gets the return type.
    pub fn return_type(&self) -> &Return {
        &self.return_type
    }

    /// Gets the types of the parameters.
    pub fn parameter_types(&self) -> &[Rc<FirstClass>] {
        &self.parameter_types
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(&self.return_type, f)?;
        f.write_str(" (")?;
        for (index, parameter_type) in self.parameter_types.iter().enumerate() {
            if index > 0 {
                f.write_str(", ")?;
            }
            Display::fmt(&parameter_type, f)?;
        }
        f.write_char(')')
    }
}

/// A type containing a fixed number of elements that are sequentially arranged in memory.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Array {
    element_type: Rc<FirstClass>,
    count: u32,
}

impl Array {
    /// Creates an array type containing a specified number of elements of a specified type.
    pub fn new(element_type: Rc<FirstClass>, count: u32) -> Self {
        Self {
            element_type,
            count,
        }
    }

    /// Gets the type of elements of this array type.
    pub fn element_type(&self) -> &Rc<FirstClass> {
        &self.element_type
    }

    /// Gets the number of elements for this array type.
    pub fn count(&self) -> u32 {
        self.count
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[{} x {}]", self.count, &self.element_type)
    }
}

/// Structure types contain members, which each have their own types.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Struct {
    packed: bool,
    member_types: Vec<Rc<FirstClass>>,
}

impl Struct {
    /// Creates a struct with the specified members.
    pub fn new(member_types: impl Into<Vec<Rc<FirstClass>>>, packed: bool) -> Self {
        Self {
            member_types: member_types.into(),
            packed,
        }
    }

    /// Gets a value indicating if the struct type is packed.
    pub fn is_packed(&self) -> bool {
        self.packed
    }

    /// Gets the types of each member.
    pub fn member_types(&self) -> &[Rc<FirstClass>] {
        &self.member_types
    }
}

impl Display for Struct {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.packed {
            f.write_char('>')?;
        }
        f.write_str("{ ")?;
        for (index, member_type) in self.member_types.iter().enumerate() {
            if index > 0 {
                f.write_str(", ")?;
            }
            Display::fmt(&member_type, f)?;
        }
        f.write_str("} ")?;
        if self.packed {
            f.write_char('>')?;
        }
        Ok(())
    }
}

/// Aggregate types represent types that contain multiple members.
///
/// Note that vector types are not aggregate types.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Aggregate {
    /// An array type containing a specific number of elements.
    Array(Array),
    /// A structure type.
    Struct(Struct),
    //Opaque,
}

impl Display for Aggregate {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Array(array) => Display::fmt(array, f),
            Self::Struct(structure) => Display::fmt(structure, f),
        }
    }
}

/// Values of first class types "are the only ones that can be produced by instructions".
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FirstClass {
    /// Single
    Single(SingleValue),
    /// Types that contain multiple members.
    Aggregate(Aggregate),
}

impl Display for FirstClass {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Single(single) => Display::fmt(single, f),
            Self::Aggregate(aggregate) => Display::fmt(aggregate, f),
        }
    }
}
