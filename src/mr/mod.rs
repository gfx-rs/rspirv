pub use self::loader::Loader;
pub use self::loader::State as LoaderState;
pub use self::constructs::{Module, ModuleHeader, Function, BasicBlock, Instruction, Operand};

mod loader;
mod constructs;
