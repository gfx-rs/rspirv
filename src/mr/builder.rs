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
    UnknownOpcode,
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

fn decode_words_to_operands(grammar: GInstRef,
                            words: Vec<spirv::Word>)
                            -> Result<Vec<mr::Operand>> {
    let mut decoder = SpirvWordDecoder::new(words);
    let mut logical_operand_index: usize = 0;
    let mut concrete_operands = Vec::new();
    while logical_operand_index < grammar.operands.len() {
        let logical_operand = &grammar.operands[logical_operand_index];
        if !decoder.empty() {
            concrete_operands.push(match logical_operand.kind {
                GOpKind::IdType => mr::Operand::IdType(decoder.id().unwrap()),
                GOpKind::IdResult => mr::Operand::IdResult(decoder.id().unwrap()),
                GOpKind::IdRef |
                GOpKind::IdMemorySemantics |
                GOpKind::IdScope => mr::Operand::IdRef(decoder.id().unwrap()),
                GOpKind::LiteralString => mr::Operand::LiteralString(decoder.string().unwrap()),
                GOpKind::LiteralContextDependentNumber => {
                    mr::Operand::LiteralContextDependentNumber(decoder.context_dependent_number()
                                                                      .unwrap())
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
                GOpKind::StorageClass => {
                    mr::Operand::StorageClass(decoder.storage_class().unwrap())
                }
                GOpKind::ImageOperands => {
                    mr::Operand::ImageOperands(decoder.image_operands().unwrap())
                }
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
                GOpKind::MemoryAccess => {
                    mr::Operand::MemoryAccess(decoder.memory_access().unwrap())
                }
                GOpKind::KernelProfilingInfo => {
                    mr::Operand::KernelProfilingInfo(decoder.kernel_profiling_info().unwrap())
                }
                GOpKind::Dim => mr::Operand::Dim(decoder.dim().unwrap()),
                GOpKind::SamplerAddressingMode => {
                    mr::Operand::SamplerAddressingMode(decoder.sampler_addressing_mode().unwrap())
                }
                GOpKind::SamplerFilterMode => {
                    mr::Operand::SamplerFilterMode(decoder.sampler_filter_mode().unwrap())
                }
                GOpKind::ImageFormat => mr::Operand::ImageFormat(decoder.image_format().unwrap()),
                GOpKind::ImageChannelOrder => {
                    mr::Operand::ImageChannelOrder(decoder.image_channel_order().unwrap())
                }
                GOpKind::ImageChannelDataType => {
                    mr::Operand::ImageChannelDataType(decoder.image_channel_data_type().unwrap())
                }
                GOpKind::FPRoundingMode => {
                    mr::Operand::FPRoundingMode(decoder.fp_rounding_mode().unwrap())
                }
                GOpKind::LinkageType => mr::Operand::LinkageType(decoder.linkage_type().unwrap()),
                GOpKind::AccessQualifier => {
                    mr::Operand::AccessQualifier(decoder.access_qualifier().unwrap())
                }
                GOpKind::FunctionParameterAttribute => {
                    mr::Operand::FunctionParameterAttribute(decoder.function_parameter_attribute()
                                                                   .unwrap())
                }
                GOpKind::BuiltIn => mr::Operand::BuiltIn(decoder.built_in().unwrap()),
                GOpKind::GroupOperation => {
                    mr::Operand::GroupOperation(decoder.group_operation().unwrap())
                }
                GOpKind::KernelEnqueueFlags => {
                    mr::Operand::KernelEnqueueFlags(decoder.kernel_enqueue_flags().unwrap())
                }
                GOpKind::LiteralExtInstInteger |
                GOpKind::LiteralSpecConstantOpInteger |
                GOpKind::PairLiteralIntegerIdRef |
                GOpKind::PairIdRefLiteralInteger => {
                    println!("unimplemented operand kind: {:?}", logical_operand.kind);
                    unimplemented!();
                }
            });
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
    Ok(concrete_operands)
}

pub struct Builder<'a> {
    module: Option<mr::Module<'a>>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Builder<'a> {
        Builder { module: None }
    }

    pub fn initialize(&mut self, header: mr::ModuleHeader) {
        let mut module = mr::Module::new();
        module.header = Some(header);
        self.module = Some(module);
    }

    pub fn add_capability(&mut self, cap: spirv::Word) {
        self.module
            .as_mut()
            .unwrap()
            .capabilities
            .push(spirv::Capability::from_u32(cap).unwrap());
    }

    pub fn add_instruction(&mut self, opcode: u16, operands: Vec<spirv::Word>) -> State {
        assert!(self.module.is_some());
        if let Some(inst) = GInstTable::lookup_opcode(opcode) {
            println!("opcode: {:?}, operands: {:?}",
                     inst.opcode,
                     decode_words_to_operands(inst, operands).unwrap());
            State::Normal
        } else {
            State::UnknownOpcode
        }
    }

    pub fn finalize(&mut self) -> Option<mr::Module<'a>> {
        self.module.take()
    }
}
