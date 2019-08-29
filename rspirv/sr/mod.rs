//! **S**tructured **r**epresentation of various SPIR-V language constructs.

pub use self::autogen_decoration::Decoration;
pub use self::autogen_instructions as instructions;
pub use self::autogen_ops as ops;
pub use self::constants::{Constant};
pub use self::types::{Type};

mod autogen_decoration;
pub mod autogen_instructions;
pub mod autogen_ops;
mod constants;
pub mod module;
pub mod storage;
mod types;

/// Error lifting a data representation of an operand into the structured
/// representation.
#[derive(Clone, Debug)]
pub enum OperandError {
    /// Operand has a wrong type.
    WrongType,
    /// Operand is missing from the list.
    Missing,
}

/// Error lifting a data representation of an instruction.
#[derive(Clone, Debug)]
pub enum InstructionError {
    /// Instruction has a wrong opcode.
    WrongOpcode,
    /// One of the operands can not be lifted.
    Operand(OperandError),
}

impl From<OperandError> for InstructionError {
    fn from(error: OperandError) -> Self {
        InstructionError::Operand(error)
    }
}
