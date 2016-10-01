use grammar;
use spirv;

use spirv::Word;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Module {
    pub header: Option<ModuleHeader>,
    pub capabilities: Vec<spirv::Capability>,
    pub extensions: Vec<String>,
    pub ext_inst_imports: Vec<Instruction>,
    pub memory_model: Option<(spirv::AddressingModel, spirv::MemoryModel)>,
    pub entry_points: Vec<Instruction>,
    pub execution_modes: Vec<Instruction>,
    pub debugs: Vec<Instruction>,
    pub names: HashMap<Word, String>,
    pub annotations: Vec<Instruction>,
    pub types_global_values: Vec<Instruction>,
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct ModuleHeader {
    pub magic_number: Word,
    pub version: Word,
    pub generator: Word,
    pub bound: Word,
    pub reserved_word: Word,
}

#[derive(Debug)]
pub struct Function {
    pub def: Option<Instruction>,
    pub end: Option<Instruction>,
    pub parameters: Vec<Instruction>,
    pub basic_blocks: Vec<BasicBlock>,
}

#[derive(Debug)]
pub struct BasicBlock {
    pub label: Instruction,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct Instruction {
    pub class: &'static grammar::Instruction<'static>,
    pub result_type: Option<Word>,
    pub result_id: Option<Word>,
    pub operands: Vec<Operand>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Operand {
    ImageOperands(spirv::ImageOperands),
    FPFastMathMode(spirv::FPFastMathMode),
    SelectionControl(spirv::SelectionControl),
    LoopControl(spirv::LoopControl),
    FunctionControl(spirv::FunctionControl),
    IdMemorySemantics(Word),
    MemoryAccess(spirv::MemoryAccess),
    KernelProfilingInfo(spirv::KernelProfilingInfo),
    SourceLanguage(spirv::SourceLanguage),
    ExecutionModel(spirv::ExecutionModel),
    AddressingModel(spirv::AddressingModel),
    MemoryModel(spirv::MemoryModel),
    ExecutionMode(spirv::ExecutionMode),
    StorageClass(spirv::StorageClass),
    Dim(spirv::Dim),
    SamplerAddressingMode(spirv::SamplerAddressingMode),
    SamplerFilterMode(spirv::SamplerFilterMode),
    ImageFormat(spirv::ImageFormat),
    ImageChannelOrder(spirv::ImageChannelOrder),
    ImageChannelDataType(spirv::ImageChannelDataType),
    FPRoundingMode(spirv::FPRoundingMode),
    LinkageType(spirv::LinkageType),
    AccessQualifier(spirv::AccessQualifier),
    FunctionParameterAttribute(spirv::FunctionParameterAttribute),
    Decoration(spirv::Decoration),
    BuiltIn(spirv::BuiltIn),
    IdScope(Word),
    GroupOperation(spirv::GroupOperation),
    KernelEnqueueFlags(spirv::KernelEnqueueFlags),
    Capability(spirv::Capability),
    IdType(Word),
    IdResult(Word),
    IdRef(Word),
    LiteralInteger(u32),
    LiteralString(String),
    LiteralContextDependentNumber(u32),
    LiteralExtInstInteger,
    LiteralSpecConstantOpInteger,
    PairLiteralIntegerIdRef,
    PairIdRefLiteralInteger,
}

impl Module {
    pub fn new() -> Module {
        Module {
            header: None,
            capabilities: vec![],
            extensions: vec![],
            ext_inst_imports: vec![],
            memory_model: None,
            entry_points: vec![],
            execution_modes: vec![],
            debugs: vec![],
            names: HashMap::new(),
            annotations: vec![],
            types_global_values: vec![],
            functions: vec![],
        }
    }
}

impl Function {
    pub fn new() -> Function {
        Function {
            def: None,
            end: None,
            parameters: vec![],
            basic_blocks: vec![],
        }
    }
}

impl BasicBlock {
    pub fn new(label: Instruction) -> BasicBlock {
        BasicBlock {
            label: label,
            instructions: vec![],
        }
    }
}

impl Instruction {
    pub fn new(class: &'static grammar::Instruction<'static>,
               result_type: Option<Word>,
               result_id: Option<Word>,
               operands: Vec<Operand>)
               -> Instruction {
        Instruction {
            class: class,
            result_type: result_type,
            result_id: result_id,
            operands: operands,
        }
    }
}
