use crate::{
    dr,
    dr::ModuleHeader,
    sr::{InstructionError, OperandError},
    sr::constants::Constant,
    sr::instructions,
    sr::ops::{self, Op},
    sr::storage::*,
    sr::types::Type,
};

use spirv;

pub struct EntryPoint {
    pub execution_model: spirv::ExecutionModel,
    pub entry_point: Token<Function>,
    pub name: String,
    //pub interface: Vec<spirv::Word>,
}

pub struct BasicBlock {
   pub terminator: ops::Terminator,
   pub ops: Vec<Op>,
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
    /// The module header.
    pub header: ModuleHeader,
    /// All OpCapability instructions.
    pub capabilities: Vec<instructions::Capability>,
    /// All OpExtension instructions.
    pub extensions: Vec<instructions::Extension>,
    /// All OpExtInstImport instructions.
    pub ext_inst_imports: Vec<instructions::ExtInstImport>,
    /// The OpMemoryModel instruction.
    ///
    /// Although it is required by the specification to appear exactly once
    /// per module, we keep it optional here to allow flexibility.
    pub memory_model: instructions::MemoryModel,
    /// All entry point declarations, using OpEntryPoint.
    pub entry_points: Vec<instructions::EntryPoint>,
    /// All execution mode declarations, using OpExecutionMode.
    pub execution_modes: Vec<instructions::ExecutionMode>,

    // some missing here...
    pub types: Storage<Type>,
    pub constants: Storage<Constant>,

    /// All functions.
    pub functions: Vec<Function>,
}

include!("autogen_module.rs");
