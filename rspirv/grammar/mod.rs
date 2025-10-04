//! The module containing the whole SPIR-V syntax grammar.
//!
//! It defines the syntax grammar of all instructions (their layouts
//! and operands).
//!
//! It also provides many reflect functions.

pub use self::syntax::*;
pub use self::syntax::{ExtendedInstruction, Instruction};
pub use self::syntax::{LogicalOperand, OperandKind, OperandQuantifier};

pub mod reflect;
mod syntax;
