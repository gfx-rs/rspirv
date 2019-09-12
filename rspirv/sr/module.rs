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
pub struct BasicBlock {
   pub ops: Vec<Op>,
   pub terminator: ops::Terminator,
}

pub struct Function {
    pub control: spirv::FunctionControl,
    /// Function result type.
    pub result: Token<Type>,
    /// Function parameters.
    pub parameters: Vec<Token<Type>>,
    /// Basic blocks in this function.
    pub basic_blocks: Vec<BasicBlock>,
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

    /// All functions.
    pub functions: Vec<Function>,
}
