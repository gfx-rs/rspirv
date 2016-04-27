pub use self::builder::Builder;
pub use self::builder::State as BuilderState;
pub use self::constructs::{Module, ModuleHeader, Function, BasicBlock, Instruction, Operand};

mod builder;
mod constructs;
