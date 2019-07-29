use crate::{
    mr::ModuleHeader,
    sr::{op, Token},
    sr::instructions::{Instruction, Terminator},
    sr::types::{Type},
};
use spirv;

pub struct EntryPoint {
    pub execution_model: spirv::ExecutionModel,
    pub entry_point: Token<Function>,
    pub name: String,
    //pub interface: Vec<spirv::Word>,
}

pub struct BasicBlock {
   pub terminator: Terminator,
   pub instructions: Vec<Instruction>,
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
    pub capabilities: Vec<op::Capability>,
    /// All OpExtension instructions.
    pub extensions: Vec<op::Extension>,
    /// All OpExtInstImport instructions.
    pub ext_inst_imports: Vec<op::ExtInstImport>,
    /// The OpMemoryModel instruction.
    ///
    /// Although it is required by the specification to appear exactly once
    /// per module, we keep it optional here to allow flexibility.
    pub memory_model: op::MemoryModel,
    /// All entry point declarations, using OpEntryPoint.
    pub entry_points: Vec<op::EntryPoint>,
    /// All execution mode declarations, using OpExecutionMode.
    pub execution_modes: Vec<op::ExecutionMode>,

    // some missing here...

    /// All functions.
    pub functions: Vec<Function>,
}
