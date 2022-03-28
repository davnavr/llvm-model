//! Types to model values in LLVM.

use crate::types;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
enum IntegerValue {
    Inline([u64; 2]),
    Allocated(Box<[u64]>),
}

/// Integer value of a specified type.
#[derive(Clone, Debug)]
pub struct Integer {
    integer_type: types::IntegerSize,
    value: IntegerValue,
}

impl Integer {
    /// Creates an integer value of the specified type with a bit pattern of all zeroes.
    pub fn zero() -> Self {
        todo!("integer value")
    }
}

/// A value.
#[derive(Clone, Debug)]
pub enum Value {
    /// An integer value.
    Integer(Integer),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str("TODO: Print value")
    }
}

crate::enum_case_from!(Value, Integer, Integer);
