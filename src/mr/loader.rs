// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use mr;
use spirv;
use grammar;

use std::result;

use binary::OperandDecoder as BOpDecoder;
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

fn decode_operand(decoder: &mut BOpDecoder, kind: GOpKind) -> Result<mr::Operand> {
    Ok(match kind {
        GOpKind::IdResultType => mr::Operand::IdResultType(decoder.id().unwrap()),
        GOpKind::IdResult => mr::Operand::IdResult(decoder.id().unwrap()),
        GOpKind::IdRef |
        GOpKind::IdMemorySemantics |
        GOpKind::IdScope => mr::Operand::IdRef(decoder.id().unwrap()),
        GOpKind::Scope => mr::Operand::Scope(decoder.scope().unwrap()),
        GOpKind::MemorySemantics => {
            mr::Operand::MemorySemantics(decoder.memory_semantics().unwrap())
        }
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
        GOpKind::PairIdRefLiteralInteger |
        GOpKind::PairIdRefIdRef => {
            println!("unimplemented operand kind: {:?}", kind);
            unimplemented!();
        }
    })
}

fn decode_words_to_operands(grammar: GInstRef, words: Vec<spirv::Word>) -> Result<mr::Instruction> {
    let mut decoder = BOpDecoder::new(words);
    let mut rtype = None;
    let mut rid = None;
    let mut logical_operand_index: usize = 0;
    let mut concrete_operands = Vec::new();
    while logical_operand_index < grammar.operands.len() {
        let logical_operand = &grammar.operands[logical_operand_index];
        if !decoder.empty() {
            match logical_operand.kind {
                GOpKind::IdResultType => rtype = decoder.id(),
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

pub struct Loader {
    module: mr::Module,
    function: Option<mr::Function>,
    block: Option<mr::BasicBlock>,
}

impl Loader {
    pub fn new() -> Loader {
        Loader {
            module: mr::Module::new(),
            function: None,
            block: None,
        }
    }

    pub fn initialize(&mut self, header: mr::ModuleHeader) {
        self.module.header = Some(header)
    }

    pub fn finalize(self) -> mr::Module {
        self.module
    }

    pub fn require_capability(&mut self, capability: mr::Operand) {
        if let mr::Operand::Capability(cap) = capability {
            self.module
                .capabilities
                .push(cap)

        } else {
            // TODO(antiagainst): we should return a suitable error here.
            panic!()
        }

    }

    pub fn enable_extension(&mut self, extension: mr::Operand) {
        if let mr::Operand::LiteralString(ext) = extension {
            self.module.extensions.push(ext)
        } else {
            panic!()
        }
    }

    pub fn attach_name(&mut self, id: mr::Operand, name: mr::Operand) {
        if let (mr::Operand::IdRef(id_ref), mr::Operand::LiteralString(name_str)) = (id, name) {
            self.module.names.insert(id_ref, name_str);
        } else {
            panic!()
        }
    }

    pub fn add_instruction(&mut self, opcode: u16, words: Vec<spirv::Word>) -> State {
        if let Some(grammar) = GInstTable::lookup_opcode(opcode) {
            let mut inst = decode_words_to_operands(grammar, words).unwrap();
            match inst.class.opcode {
                spirv::Op::Capability => self.require_capability(inst.operands.pop().unwrap()),
                spirv::Op::Extension => self.enable_extension(inst.operands.pop().unwrap()),
                spirv::Op::ExtInstImport => {
                    self.module
                        .ext_inst_imports
                        .push(inst)
                }
                spirv::Op::MemoryModel => {
                    let memory = inst.operands.pop().unwrap();
                    let address = inst.operands.pop().unwrap();
                    if let (mr::Operand::AddressingModel(am), mr::Operand::MemoryModel(mm)) =
                           (address, memory) {
                        self.module.memory_model = Some((am, mm))
                    }
                }
                spirv::Op::EntryPoint => {
                    self.module
                        .entry_points
                        .push(inst)
                }
                spirv::Op::ExecutionMode => {
                    self.module
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
                        .debugs
                        .push(inst)
                }
                opcode if grammar::reflect::is_annotation(opcode) => {
                    self.module
                        .annotations
                        .push(inst)
                }
                opcode if grammar::reflect::is_type(opcode) ||
                          grammar::reflect::is_constant(opcode) ||
                          grammar::reflect::is_variable(opcode) => {
                    self.module
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
                    self.module.functions.push(self.function.take().unwrap())
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
