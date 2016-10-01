use mr;
use spirv;
use grammar;

use std::result;
use num::traits::cast::FromPrimitive;

use grammar::InstructionTable as GInstTable;
use grammar::OperandKind as GOpKind;
use grammar::OperandQuantifier as GOpCount;

type GInstRef = &'static grammar::Instruction<'static>;

#[derive(Debug)]
pub enum State {
    Normal,
    OpcodeUnknown,
    OperandExpected,
}

type Result<T> = result::Result<T, State>;

const WORD_NUM_BYTES: usize = 4;

struct SpirvWordDecoder {
    words: Vec<spirv::Word>,
    index: usize,
}

impl Iterator for SpirvWordDecoder {
    type Item = spirv::Word;

    fn next(&mut self) -> Option<spirv::Word> {
        if self.empty() {
            None
        } else {
            self.index += 1;
            Some(self.words[self.index - 1])
        }
    }
}

impl SpirvWordDecoder {
    pub fn new(words: Vec<spirv::Word>) -> SpirvWordDecoder {
        SpirvWordDecoder {
            words: words,
            index: 0,
        }
    }

    pub fn empty(&self) -> bool {
        self.index >= self.words.len()
    }

    fn split_word_to_bytes(&self, word: spirv::Word) -> Vec<u8> {
        (0..WORD_NUM_BYTES).map(|i| ((word >> (8 * i)) & 0xff) as u8).collect()
    }

    // TODO(antiagainst): in the following methods, we should distinguish None caused by no next
    // word and cannot decode the next word to the expected type.

    pub fn capability(&mut self) -> Option<spirv::Capability> {
        if let Some(word) = self.next() {
            spirv::Capability::from_u32(word)
        } else {
            None
        }
    }

    pub fn addressing_model(&mut self) -> Option<spirv::AddressingModel> {
        if let Some(word) = self.next() {
            spirv::AddressingModel::from_u32(word)
        } else {
            None
        }
    }

    pub fn memory_model(&mut self) -> Option<spirv::MemoryModel> {
        if let Some(word) = self.next() {
            spirv::MemoryModel::from_u32(word)
        } else {
            None
        }
    }

    pub fn execution_mode(&mut self) -> Option<spirv::ExecutionMode> {
        if let Some(word) = self.next() {
            spirv::ExecutionMode::from_u32(word)
        } else {
            None
        }
    }

    pub fn decode_execution_model(&mut self) -> Option<spirv::ExecutionModel> {
        if let Some(word) = self.next() {
            spirv::ExecutionModel::from_u32(word)
        } else {
            None
        }
    }

    pub fn source_language(&mut self) -> Option<spirv::SourceLanguage> {
        if let Some(word) = self.next() {
            spirv::SourceLanguage::from_u32(word)
        } else {
            None
        }
    }

    pub fn decoration(&mut self) -> Option<spirv::Decoration> {
        if let Some(word) = self.next() {
            spirv::Decoration::from_u32(word)
        } else {
            None
        }
    }

    pub fn storage_class(&mut self) -> Option<spirv::StorageClass> {
        if let Some(word) = self.next() {
            spirv::StorageClass::from_u32(word)
        } else {
            None
        }
    }

    pub fn dim(&mut self) -> Option<spirv::Dim> {
        if let Some(word) = self.next() {
            spirv::Dim::from_u32(word)
        } else {
            None
        }
    }

    pub fn sampler_addressing_mode(&mut self) -> Option<spirv::SamplerAddressingMode> {
        if let Some(word) = self.next() {
            spirv::SamplerAddressingMode::from_u32(word)
        } else {
            None
        }
    }

    pub fn sampler_filter_mode(&mut self) -> Option<spirv::SamplerFilterMode> {
        if let Some(word) = self.next() {
            spirv::SamplerFilterMode::from_u32(word)
        } else {
            None
        }
    }

    pub fn image_format(&mut self) -> Option<spirv::ImageFormat> {
        if let Some(word) = self.next() {
            spirv::ImageFormat::from_u32(word)
        } else {
            None
        }
    }

    pub fn image_channel_order(&mut self) -> Option<spirv::ImageChannelOrder> {
        if let Some(word) = self.next() {
            spirv::ImageChannelOrder::from_u32(word)
        } else {
            None
        }
    }

    pub fn image_channel_data_type(&mut self) -> Option<spirv::ImageChannelDataType> {
        if let Some(word) = self.next() {
            spirv::ImageChannelDataType::from_u32(word)
        } else {
            None
        }
    }

    pub fn fp_rounding_mode(&mut self) -> Option<spirv::FPRoundingMode> {
        if let Some(word) = self.next() {
            spirv::FPRoundingMode::from_u32(word)
        } else {
            None
        }
    }

    pub fn linkage_type(&mut self) -> Option<spirv::LinkageType> {
        if let Some(word) = self.next() {
            spirv::LinkageType::from_u32(word)
        } else {
            None
        }
    }

    pub fn access_qualifier(&mut self) -> Option<spirv::AccessQualifier> {
        if let Some(word) = self.next() {
            spirv::AccessQualifier::from_u32(word)
        } else {
            None
        }
    }

    pub fn function_parameter_attribute(&mut self) -> Option<spirv::FunctionParameterAttribute> {
        if let Some(word) = self.next() {
            spirv::FunctionParameterAttribute::from_u32(word)
        } else {
            None
        }
    }

    pub fn built_in(&mut self) -> Option<spirv::BuiltIn> {
        if let Some(word) = self.next() {
            spirv::BuiltIn::from_u32(word)
        } else {
            None
        }
    }

    pub fn group_operation(&mut self) -> Option<spirv::GroupOperation> {
        if let Some(word) = self.next() {
            spirv::GroupOperation::from_u32(word)
        } else {
            None
        }
    }

    pub fn kernel_enqueue_flags(&mut self) -> Option<spirv::KernelEnqueueFlags> {
        if let Some(word) = self.next() {
            spirv::KernelEnqueueFlags::from_u32(word)
        } else {
            None
        }
    }

    pub fn image_operands(&mut self) -> Option<spirv::ImageOperands> {
        if let Some(word) = self.next() {
            spirv::ImageOperands::from_bits(word)
        } else {
            None
        }
    }

    pub fn fp_fast_math_mode(&mut self) -> Option<spirv::FPFastMathMode> {
        if let Some(word) = self.next() {
            spirv::FPFastMathMode::from_bits(word)
        } else {
            None
        }
    }

    pub fn selection_control(&mut self) -> Option<spirv::SelectionControl> {
        if let Some(word) = self.next() {
            spirv::SelectionControl::from_bits(word)
        } else {
            None
        }
    }

    pub fn loop_control(&mut self) -> Option<spirv::LoopControl> {
        if let Some(word) = self.next() {
            spirv::LoopControl::from_bits(word)
        } else {
            None
        }
    }

    pub fn function_control(&mut self) -> Option<spirv::FunctionControl> {
        if let Some(word) = self.next() {
            spirv::FunctionControl::from_bits(word)
        } else {
            None
        }
    }

    pub fn memory_access(&mut self) -> Option<spirv::MemoryAccess> {
        if let Some(word) = self.next() {
            spirv::MemoryAccess::from_bits(word)
        } else {
            None
        }
    }

    pub fn kernel_profiling_info(&mut self) -> Option<spirv::KernelProfilingInfo> {
        if let Some(word) = self.next() {
            spirv::KernelProfilingInfo::from_bits(word)
        } else {
            None
        }
    }

    pub fn string(&mut self) -> Option<String> {
        let mut bytes = Vec::new();
        while let Some(word) = self.next() {
            bytes.append(&mut self.split_word_to_bytes(word));
            if bytes.last() == Some(&0) {
                break;
            }
        }
        while !bytes.is_empty() && bytes.last() == Some(&0) {
            bytes.pop();
        }
        String::from_utf8(bytes).ok()
    }

    pub fn id(&mut self) -> Option<spirv::Word> {
        self.next()
    }

    pub fn literal_integer(&mut self) -> Option<u32> {
        self.next()
    }

    pub fn context_dependent_number(&mut self) -> Option<u32> {
        // TODO(antiagainst): This should return the correct typed number.
        self.next()
    }
}

fn decode_operand(decoder: &mut SpirvWordDecoder,
                  kind: grammar::OperandKind)
                  -> Result<mr::Operand> {
    Ok(match kind {
        GOpKind::IdType => mr::Operand::IdType(decoder.id().unwrap()),
        GOpKind::IdResult => mr::Operand::IdResult(decoder.id().unwrap()),
        GOpKind::IdRef |
        GOpKind::IdMemorySemantics |
        GOpKind::IdScope => mr::Operand::IdRef(decoder.id().unwrap()),
        GOpKind::LiteralString => mr::Operand::LiteralString(decoder.string().unwrap()),
        GOpKind::LiteralContextDependentNumber => {
            mr::Operand::LiteralContextDependentNumber(decoder.context_dependent_number().unwrap())
        }
        GOpKind::Capability => mr::Operand::Capability(decoder.capability().unwrap()),
        GOpKind::Decoration => mr::Operand::Decoration(decoder.decoration().unwrap()),
        GOpKind::AddressingModel => {
            mr::Operand::AddressingModel(decoder.addressing_model()
                                                .unwrap())
        }
        GOpKind::MemoryModel => mr::Operand::MemoryModel(decoder.memory_model().unwrap()),
        GOpKind::ExecutionMode => {
            mr::Operand::ExecutionMode(decoder.execution_mode()
                                              .unwrap())
        }
        GOpKind::ExecutionModel => {
            mr::Operand::ExecutionModel(decoder.decode_execution_model()
                                               .unwrap())
        }
        GOpKind::SourceLanguage => {
            mr::Operand::SourceLanguage(decoder.source_language()
                                               .unwrap())
        }
        GOpKind::LiteralInteger => {
            mr::Operand::LiteralInteger(decoder.literal_integer()
                                               .unwrap())
        }
        GOpKind::StorageClass => mr::Operand::StorageClass(decoder.storage_class().unwrap()),
        GOpKind::ImageOperands => mr::Operand::ImageOperands(decoder.image_operands().unwrap()),
        GOpKind::FPFastMathMode => {
            mr::Operand::FPFastMathMode(decoder.fp_fast_math_mode().unwrap())
        }
        GOpKind::SelectionControl => {
            mr::Operand::SelectionControl(decoder.selection_control().unwrap())
        }
        GOpKind::LoopControl => mr::Operand::LoopControl(decoder.loop_control().unwrap()),
        GOpKind::FunctionControl => {
            mr::Operand::FunctionControl(decoder.function_control().unwrap())
        }
        GOpKind::MemoryAccess => mr::Operand::MemoryAccess(decoder.memory_access().unwrap()),
        GOpKind::KernelProfilingInfo => {
            mr::Operand::KernelProfilingInfo(decoder.kernel_profiling_info()
                                                    .unwrap())
        }
        GOpKind::Dim => mr::Operand::Dim(decoder.dim().unwrap()),
        GOpKind::SamplerAddressingMode => {
            mr::Operand::SamplerAddressingMode(decoder.sampler_addressing_mode()
                                                      .unwrap())
        }
        GOpKind::SamplerFilterMode => {
            mr::Operand::SamplerFilterMode(decoder.sampler_filter_mode().unwrap())
        }
        GOpKind::ImageFormat => mr::Operand::ImageFormat(decoder.image_format().unwrap()),
        GOpKind::ImageChannelOrder => {
            mr::Operand::ImageChannelOrder(decoder.image_channel_order().unwrap())
        }
        GOpKind::ImageChannelDataType => {
            mr::Operand::ImageChannelDataType(decoder.image_channel_data_type()
                                                     .unwrap())
        }
        GOpKind::FPRoundingMode => mr::Operand::FPRoundingMode(decoder.fp_rounding_mode().unwrap()),
        GOpKind::LinkageType => mr::Operand::LinkageType(decoder.linkage_type().unwrap()),
        GOpKind::AccessQualifier => {
            mr::Operand::AccessQualifier(decoder.access_qualifier().unwrap())
        }
        GOpKind::FunctionParameterAttribute => {
            mr::Operand::FunctionParameterAttribute(decoder.function_parameter_attribute().unwrap())
        }
        GOpKind::BuiltIn => mr::Operand::BuiltIn(decoder.built_in().unwrap()),
        GOpKind::GroupOperation => mr::Operand::GroupOperation(decoder.group_operation().unwrap()),
        GOpKind::KernelEnqueueFlags => {
            mr::Operand::KernelEnqueueFlags(decoder.kernel_enqueue_flags().unwrap())
        }
        GOpKind::LiteralExtInstInteger |
        GOpKind::LiteralSpecConstantOpInteger |
        GOpKind::PairLiteralIntegerIdRef |
        GOpKind::PairIdRefLiteralInteger => {
            println!("unimplemented operand kind: {:?}", kind);
            unimplemented!();
        }
    })
}

fn decode_words_to_operands(grammar: GInstRef, words: Vec<spirv::Word>) -> Result<mr::Instruction> {
    let mut decoder = SpirvWordDecoder::new(words);
    let mut logical_operand_index: usize = 0;
    let mut rtype = None;
    let mut rid = None;
    let mut concrete_operands = Vec::new();
    while logical_operand_index < grammar.operands.len() {
        let logical_operand = &grammar.operands[logical_operand_index];
        if !decoder.empty() {
            match logical_operand.kind {
                GOpKind::IdType => rtype = decoder.id(),
                GOpKind::IdResult => rid = decoder.id(),
                _ => {
                    concrete_operands.push(decode_operand(&mut decoder, logical_operand.kind)
                                               .unwrap())
                }
            }
            match logical_operand.quantifier {
                GOpCount::One | GOpCount::ZeroOrOne => logical_operand_index += 1,
                GOpCount::ZeroOrMore => continue,
            }
        } else {
            // We still have logical operands to match but no no more words.
            match logical_operand.quantifier {
                GOpCount::One => return Err(State::OperandExpected),
                GOpCount::ZeroOrOne | GOpCount::ZeroOrMore => break,
            }
        }
    }
    Ok(mr::Instruction::new(grammar, rtype, rid, concrete_operands))
}

pub struct Builder {
    module: Option<mr::Module>,
    function: Option<mr::Function>,
    block: Option<mr::BasicBlock>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            module: None,
            function: None,
            block: None,
        }
    }

    pub fn initialize(&mut self, header: mr::ModuleHeader) {
        let mut module = mr::Module::new();
        module.header = Some(header);
        self.module = Some(module);
    }

    pub fn finalize(&mut self) -> Option<mr::Module> {
        self.module.take()
    }

    pub fn require_capability(&mut self, capability: mr::Operand) {
        if let mr::Operand::Capability(cap) = capability {
            self.module
                .as_mut()
                .unwrap()
                .capabilities
                .push(cap)

        } else {
            // TODO(antiagainst): we should return a suitable error here.
            panic!()
        }

    }

    pub fn enable_extension(&mut self, extension: mr::Operand) {
        if let mr::Operand::LiteralString(ext) = extension {
            self.module.as_mut().unwrap().extensions.push(ext)
        } else {
            panic!()
        }
    }

    pub fn attach_name(&mut self, id: mr::Operand, name: mr::Operand) {
        if let (mr::Operand::IdRef(id_ref), mr::Operand::LiteralString(name_str)) = (id, name) {
            self.module.as_mut().unwrap().names.insert(id_ref, name_str);
        } else {
            panic!()
        }
    }

    pub fn add_instruction(&mut self, opcode: u16, words: Vec<spirv::Word>) -> State {
        assert!(self.module.is_some());
        if let Some(grammar) = GInstTable::lookup_opcode(opcode) {
            let mut inst = decode_words_to_operands(grammar, words).unwrap();
            match inst.class.opcode {
                spirv::Op::Capability => self.require_capability(inst.operands.pop().unwrap()),
                spirv::Op::Extension => self.enable_extension(inst.operands.pop().unwrap()),
                spirv::Op::ExtInstImport => {
                    self.module
                        .as_mut()
                        .unwrap()
                        .ext_inst_imports
                        .push(inst)
                }
                spirv::Op::MemoryModel => {
                    let memory = inst.operands.pop().unwrap();
                    let address = inst.operands.pop().unwrap();
                    if let (mr::Operand::AddressingModel(am), mr::Operand::MemoryModel(mm)) =
                           (address, memory) {
                        self.module.as_mut().unwrap().memory_model = Some((am, mm))
                    }
                }
                spirv::Op::EntryPoint => {
                    self.module
                        .as_mut()
                        .unwrap()
                        .entry_points
                        .push(inst)
                }
                spirv::Op::ExecutionMode => {
                    self.module
                        .as_mut()
                        .unwrap()
                        .execution_modes
                        .push(inst)
                }
                spirv::Op::Name => {
                    let name = inst.operands.pop().unwrap();
                    let id = inst.operands.pop().unwrap();
                    self.attach_name(id, name);
                }
                opcode if grammar::reflect::is_nonlocation_debug(opcode) => {
                    self.module
                        .as_mut()
                        .unwrap()
                        .debugs
                        .push(inst)
                }
                opcode if grammar::reflect::is_annotation(opcode) => {
                    self.module
                        .as_mut()
                        .unwrap()
                        .annotations
                        .push(inst)
                }
                opcode if grammar::reflect::is_type(opcode) ||
                          grammar::reflect::is_constant(opcode) ||
                          grammar::reflect::is_variable(opcode) => {
                    self.module
                        .as_mut()
                        .unwrap()
                        .types_global_values
                        .push(inst)
                }
                spirv::Op::Function => {
                    let mut f = mr::Function::new();
                    f.def = Some(inst);
                    self.function = Some(f)
                }
                spirv::Op::FunctionEnd => {
                    self.function.as_mut().unwrap().end = Some(inst);
                    self.module.as_mut().unwrap().functions.push(self.function.take().unwrap())
                }
                spirv::Op::FunctionParameter => {
                    self.function
                        .as_mut()
                        .unwrap()
                        .parameters
                        .push(inst);
                }
                spirv::Op::Label => self.block = Some(mr::BasicBlock::new(inst)),
                opcode if grammar::reflect::is_terminator(opcode) => {
                    self.block
                        .as_mut()
                        .unwrap()
                        .instructions
                        .push(inst);
                    self.function.as_mut().unwrap().basic_blocks.push(self.block.take().unwrap())
                }
                _ => {
                    self.block
                        .as_mut()
                        .unwrap()
                        .instructions
                        .push(inst)
                }
            }
            State::Normal
        } else {
            State::OpcodeUnknown
        }
    }
}
