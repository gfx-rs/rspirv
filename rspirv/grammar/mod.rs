//! The module containing the whole SPIR-V syntax grammar.
//!
//! It defines the syntax grammar of all instructions (their layouts
//! and operands).
//!
//! It also provides many reflect functions.

pub use self::syntax::CoreInstructionTable;
pub use self::syntax::GlslStd450InstructionTable;
pub use self::syntax::OpenCLStd100InstructionTable;
pub use self::syntax::{ExtendedInstruction, Instruction};
pub use self::syntax::{LogicalOperand, OperandKind, OperandQuantifier};

pub mod reflect;
mod syntax;
