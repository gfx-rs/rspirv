use grammar;
use spirv;

use spirv::Word;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Module<'a> {
    pub header: Option<ModuleHeader>,
    pub capabilities: Vec<spirv::Capability>,
    pub instructions: Vec<Instruction>,
    pub functions: Vec<Function<'a>>,
    pub names: HashMap<Word, String>,
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
pub struct Function<'a> {
    pub basic_blocks: Vec<BasicBlock<'a>>,
}

#[derive(Debug)]
pub struct BasicBlock<'a> {
    pub function: &'a Function<'a>,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct Instruction {
    pub class: &'static grammar::Instruction<'static>,
    pub result_type: Option<Word>,
    pub result_id: Option<Word>,
    pub operands: Vec<Word>,
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

impl<'a> Module<'a> {
    pub fn new() -> Module<'a> {
        Module {
            header: None,
            capabilities: vec![],
            instructions: vec![],
            functions: vec![],
            names: HashMap::new(),
        }
    }
}

impl Instruction {
    pub fn new(class: &'static grammar::Instruction<'static>) -> Instruction {
        Instruction {
            class: class,
            result_type: None,
            result_id: None,
            operands: vec![],
        }
    }
}
