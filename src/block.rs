//! Code to create and write the contents of basic blocks.
//!
//! See [the LLVM instruction set reference here](https://llvm.org/docs/LangRef.html#instruction-reference).

use crate::value::Value;
use std::fmt::{Display, Formatter, Write as _};
use std::rc::Rc;

fn block_name(block: &BasicBlock, f: &mut Formatter) -> std::fmt::Result {
    write!(f, "B{:X}", block as *const BasicBlock as usize)
}

struct BlockLabel<'b>(&'b BasicBlock);

impl Display for BlockLabel<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_char('%')?;
        block_name(self.0, f)
    }
}

#[derive(Debug)]
enum Instruction {
    Ret(Option<Value>),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Ret(value) => {
                f.write_str("ret ")?;
                match value {
                    Some(return_value) => Display::fmt(&return_value, f),
                    None => f.write_str("void"),
                }
            }
        }
    }
}

/// An LLVM basic block contains the instructions that make up function definitions.
#[derive(Debug)]
pub struct BasicBlock {
    instructions: Vec<Instruction>,
    terminated: bool,
}

impl BasicBlock {
    /// Creates an empty basic block containing no instructions.
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            instructions: Vec::new(),
            terminated: false,
        })
    }

    fn append_instruction(&mut self, instruction: Instruction) {
        if self.terminated {
            panic!(
                "attempt to append instruction {}, but block {} already ends with a terminator instruction",
                instruction,
                BlockLabel(self),
            );
        } else {
            self.instructions.push(instruction)
        }
    }

    /// Appends an `ret` instruction, which returns control flow back to the calling function.
    pub fn ret(&mut self, value: Option<Value>) {
        self.append_instruction(Instruction::Ret(value));
        self.terminated = true;
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        block_name(self, f)?;
        writeln!(f, ":")?;
        for instruction in self.instructions.iter() {
            writeln!(f, "  {}", instruction)?;
        }
        Ok(())
    }
}
