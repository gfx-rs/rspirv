pub use self::builder::Builder;
pub use self::builder::State as BuilderState;
pub use self::structs::{Module, ModuleHeader, Function, BasicBlock, Instruction, Operand};

mod builder;
mod structs;
