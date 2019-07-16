use crate::{
	mr::ModuleHeader,
	spirv,
	sr::TypeToken,
};
use super::instructions as inst;

pub struct EntryPoint {
	pub execution_model: spirv::ExecutionModel,
    pub entry_point: FunctionToken,
    pub name: String,
    //pub interface: Vec<spirv::Word>,
}

pub struct BasicBlock {
	//line: Line,
	terminator: inst::Terminator,
}

pub struct Function {
	pub control: spirv::FunctionControl,
	/// Function result type.
    pub result: TypeToken,
    /// Function parameters.
    pub parameters: Vec<TypeToken>,
    /// Basic blocks in this function.
    pub basic_blocks: Vec<BasicBlock>,
}

pub struct FunctionToken {
	index: usize,
}

pub struct Module {
    /// The module header.
    pub header: Option<ModuleHeader>,
    /// All OpCapability instructions.
    pub capabilities: Vec<inst::Capability>,
    /// All OpExtension instructions.
    pub extensions: Vec<inst::Extension>,
    /// All OpExtInstImport instructions.
    pub ext_inst_imports: Vec<inst::ExtInstImport>,
    /// The OpMemoryModel instruction.
    ///
    /// Although it is required by the specification to appear exactly once
    /// per module, we keep it optional here to allow flexibility.
    pub memory_model: Option<inst::MemoryModel>,
    /// All entry point declarations, using OpEntryPoint.
    pub entry_points: Vec<inst::EntryPoint>,
    /// All execution mode declarations, using OpExecutionMode.
    pub execution_modes: Vec<inst::ExecutionMode>,

    // some missing here...

    /// All functions.
    pub functions: Vec<Function>,
}
