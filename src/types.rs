//! Model of the LLVM type system.

use std::fmt::{Display, Formatter};
use std::num::NonZeroU32;

/// Represents the size of an integer, which can be a value from `1` to `2^23`.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IntegerSize(NonZeroU32);

impl IntegerSize {
    /// Minimum size value.
    pub const MIN: Self = Self(unsafe { NonZeroU32::new_unchecked(1) });

    /// Gets the size, in bits.
    pub fn bits(self) -> u32 {
        self.0.get()
    }
}

/// Represents an integer type of an arbitrary bit width.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Integer {
    /// A signed integer type.
    Signed(IntegerSize),
    /// An unsigned integer type.
    Unsigned(IntegerSize),
}

/// Represents a floating-point type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Float {
    /// 16-bit, IEEE-754 `binary16`.
    Half,
    /// 32-bit, IEEE-754 `binary32`.
    Float,
    /// 64-bit, IEEE-754 `binary64`.
    Double,
}

pub use crate::target::layout::AddressSpace;

/// A pointer type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pointer {
    pointee_type: Box<FirstClass>,
    address_space: AddressSpace,
}

impl Pointer {
    /// Creates a pointer type pointing to a objects of a particular type in a particular address space.
    pub fn in_address_space(pointee_type: FirstClass, address_space: AddressSpace) -> Self {
        Self {
            pointee_type: Box::new(pointee_type),
            address_space,
        }
    }

    /// Creates a pointer type pointing to objects of a particular type.
    pub fn new(pointee_type: FirstClass) -> Self {
        Self::in_address_space(pointee_type, AddressSpace::VON_NEUMANN_DEFAULT)
    }

    /// The type of object that is pointed to by the pointer type.
    pub fn pointee_type(&self) -> &FirstClass {
        &self.pointee_type
    }

    /// The address space of the pointer, indicating where the object pointed to resides.
    pub fn address_space(&self) -> AddressSpace {
        self.address_space
    }
}

/// A vector of elements of a specified size.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vector {
    element_type: Box<FirstClass>,
    count: NonZeroU32,
}

impl Vector {
    /// Creates a vector type containing a specified number of elements of a specified type.
    pub fn new(element_type: FirstClass, count: NonZeroU32) -> Self {
        Self {
            element_type: Box::new(element_type),
            count,
        }
    }

    /// Gets the type of elements of this vector type.
    pub fn element_type(&self) -> &FirstClass {
        &self.element_type
    }

    /// Gets the number of elements, guaranteed to be greater than zero.
    pub fn count(&self) -> u32 {
        self.count.get()
    }
}

/// A subset of the types that are valid in registers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SingleValue {
    /// Fixed size integer type.
    Integer(Integer),
    /// Floating point type.
    Float(Float),
    /// A pointer type.
    Pointer(Pointer),
    /// A vector of elements of a specified size.
    Vector(Vector),
}

//#[derive(Clone, Debug, Eq, PartialEq)]
//pub struct Function

/// A type containing a fixed number of elements that are sequentially arranged in memory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Array {
    element_type: Box<FirstClass>,
    count: u32,
}

impl Array {
    /// Creates an array type containing a specified number of elements of a specified type.
    pub fn new(element_type: FirstClass, count: u32) -> Self {
        Self {
            element_type: Box::new(element_type),
            count,
        }
    }

    /// Gets the type of elements of this array type.
    pub fn element_type(&self) -> &FirstClass {
        &self.element_type
    }

    /// Gets the number of elements for this array type.
    pub fn count(&self) -> u32 {
        self.count
    }
}

/// Structure types contain members, which each have their own types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Struct {
    packed: bool,
    member_types: Vec<FirstClass>,
}

impl Struct {
    /// Creates a struct with the specified members.
    pub fn new(member_types: impl Into<Vec<FirstClass>>, packed: bool) -> Self {
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
    pub fn member_types(&self) -> &[FirstClass] {
        &self.member_types
    }
}

/// Aggregate types represent types that contain multiple members.
/// 
/// Note that vector types are not aggregate types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Aggregate {
    /// An array type containing a specific number of elements.
    Array(Array),
    /// A structure type.
    Struct(Struct),
    //Opaque,
}

/// Values of first class types "are the only ones that can be produced by instructions".
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FirstClass {
    /// Single
    Single(SingleValue),
    /// Types that contain multiple members.
    Aggregate(Aggregate),
}

/// A type representing no value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Void;
