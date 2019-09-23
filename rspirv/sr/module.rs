use crate::{
    sr::constants::Constant,
    sr::instructions,
    sr::ops::{self, Op},
    sr::storage::*,
    sr::types::Type,
};

use spirv;

#[derive(Debug)]
pub struct EntryPoint {
    pub execution_model: spirv::ExecutionModel,
    pub function: Token<Function>,
    pub name: String,
    //pub interface: Vec<spirv::Word>,
}

#[derive(Debug)]
pub struct Block {
    pub arguments: Vec<Token<Type>>,
    pub ops: Vec<Token<Op>>,
    pub terminator: ops::Terminator,
}

/// Jump destination parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Jump {
    /// The block to jump to.
    pub block: Token<Block>,
    /// The argument values corresponding to the block arguments.
    pub arguments: Vec<Token<Op>>,
}

pub struct Function {
    pub control: spirv::FunctionControl,
    /// Function result type.
    pub result: Token<Type>,
    /// Function parameters.
    pub parameters: Vec<Token<Type>>,
    /// Blocks in this function.
    pub blocks: Vec<Token<Block>>,
}

pub struct Module {
    /// Version of the specification.
    pub version: spirv::Word,
    /// All OpCapability instructions.
    pub capabilities: Vec<spirv::Capability>,
    /// All OpExtension instructions.
    pub extensions: Vec<String>,
    /// All OpExtInstImport instructions.
    pub ext_inst_imports: Vec<String>,
    /// The OpMemoryModel instruction.
    pub memory_model: instructions::MemoryModel,
    /// All entry point declarations.
    pub entry_points: Vec<EntryPoint>,

    /// All types
    pub types: Storage<Type>,
    /// All constants.
    pub constants: Storage<Constant>,

    /// All blocks.
    pub blocks: Storage<Block>,
    /// All operations.
    pub ops: Storage<Op>,

    /// All functions.
    pub functions: Vec<Function>,
}
