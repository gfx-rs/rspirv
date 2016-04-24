use mr;
use spirv;
use grammar;

use std::result;
use num::traits::cast::FromPrimitive;

use grammar::InstructionTable as GInstTable;
use grammar::OperandKind as GOperandKind;
use grammar::OperandQuantifier as GOperandQuantifier;

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

    pub fn decode_capability(&mut self) -> Option<spirv::Capability> {
        if let Some(word) = self.next() {
            spirv::Capability::from_u32(word)
        } else {
            None
        }
    }

    pub fn decode_addressing_model(&mut self) -> Option<spirv::AddressingModel> {
        if let Some(word) = self.next() {
            spirv::AddressingModel::from_u32(word)
        } else {
            None
        }
    }

    pub fn decode_memory_model(&mut self) -> Option<spirv::MemoryModel> {
        if let Some(word) = self.next() {
            spirv::MemoryModel::from_u32(word)
        } else {
            None
        }
    }

    pub fn decode_execution_mode(&mut self) -> Option<spirv::ExecutionMode> {
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

    pub fn decode_source_language(&mut self) -> Option<spirv::SourceLanguage> {
        if let Some(word) = self.next() {
            spirv::SourceLanguage::from_u32(word)
        } else {
            None
        }
    }

    pub fn decode_decoration(&mut self) -> Option<spirv::Decoration> {
        if let Some(word) = self.next() {
            spirv::Decoration::from_u32(word)
        } else {
            None
        }
    }

    pub fn decode_storage_class(&mut self) -> Option<spirv::StorageClass> {
        if let Some(word) = self.next() {
            spirv::StorageClass::from_u32(word)
        } else {
            None
        }
    }

    pub fn decode_string(&mut self) -> Option<String> {
        let mut bytes = Vec::new();
        while let Some(word) = self.next() {
            bytes.append(&mut self.split_word_to_bytes(word));
            if bytes.last() == Some(&0) {
                break;
            }
        }
        String::from_utf8(bytes).ok()
    }

    pub fn decode_id(&mut self) -> Option<spirv::Word> {
        self.next()
    }

    pub fn decode_literal_integer(&mut self) -> Option<u32> {
        self.next()
    }

    pub fn decode_context_dependent_number(&mut self) -> Option<u32> {
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
                GOperandKind::Capability => {
                    mr::Operand::Capability(decoder.decode_capability().unwrap())
                }
                GOperandKind::IdType => mr::Operand::IdType(decoder.decode_id().unwrap()),
                GOperandKind::IdResult => mr::Operand::IdResult(decoder.decode_id().unwrap()),
                GOperandKind::IdRef => mr::Operand::IdRef(decoder.decode_id().unwrap()),
                GOperandKind::LiteralString => {
                    mr::Operand::LiteralString(decoder.decode_string().unwrap())
                }
                GOperandKind::AddressingModel => {
                    mr::Operand::AddressingModel(decoder.decode_addressing_model()
                                                        .unwrap())
                }
                GOperandKind::MemoryModel => {
                    mr::Operand::MemoryModel(decoder.decode_memory_model().unwrap())
                }
                GOperandKind::ExecutionMode => {
                    mr::Operand::ExecutionMode(decoder.decode_execution_mode()
                                                      .unwrap())
                }
                GOperandKind::ExecutionModel => {
                    mr::Operand::ExecutionModel(decoder.decode_execution_model()
                                                       .unwrap())
                }
                GOperandKind::SourceLanguage => {
                    mr::Operand::SourceLanguage(decoder.decode_source_language()
                                                       .unwrap())
                }
                GOperandKind::LiteralInteger => {
                    mr::Operand::LiteralInteger(decoder.decode_literal_integer()
                                                       .unwrap())
                }
                GOperandKind::Decoration => {
                    mr::Operand::Decoration(decoder.decode_decoration().unwrap())
                }
                GOperandKind::StorageClass => {
                    mr::Operand::StorageClass(decoder.decode_storage_class().unwrap())
                }
                GOperandKind::LiteralContextDependentNumber => {
                    mr::Operand::LiteralContextDependentNumber(
                        decoder.decode_context_dependent_number().unwrap())
                }
                GOperandKind::ImageOperands |
                GOperandKind::FPFastMathMode |
                GOperandKind::SelectionControl |
                GOperandKind::LoopControl |
                GOperandKind::FunctionControl |
                GOperandKind::IdMemorySemantics |
                GOperandKind::MemoryAccess |
                GOperandKind::KernelProfilingInfo |
                GOperandKind::Dim |
                GOperandKind::SamplerAddressingMode |
                GOperandKind::SamplerFilterMode |
                GOperandKind::ImageFormat |
                GOperandKind::ImageChannelOrder |
                GOperandKind::ImageChannelDataType |
                GOperandKind::FPRoundingMode |
                GOperandKind::LinkageType |
                GOperandKind::AccessQualifier |
                GOperandKind::FunctionParameterAttribute |
                GOperandKind::BuiltIn |
                GOperandKind::IdScope |
                GOperandKind::GroupOperation |
                GOperandKind::KernelEnqueueFlags |
                GOperandKind::LiteralExtInstInteger |
                GOperandKind::LiteralSpecConstantOpInteger |
                GOperandKind::PairLiteralIntegerIdRef |
                GOperandKind::PairIdRefLiteralInteger => {
                    println!("unimplemented operand kind: {:?}", logical_operand.kind);
                    unimplemented!();
                }
            });
            match logical_operand.quantifier {
                GOperandQuantifier::One |
                GOperandQuantifier::ZeroOrOne => logical_operand_index += 1,
                GOperandQuantifier::ZeroOrMore => continue,
            }
        } else {
            // We still have logical operands to match but no no more words.
            match logical_operand.quantifier {
                GOperandQuantifier::One => return Err(State::OperandExpected),
                GOperandQuantifier::ZeroOrOne |
                GOperandQuantifier::ZeroOrMore => break,
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
