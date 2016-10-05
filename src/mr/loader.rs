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

use binary;
use mr;
use spirv;
use grammar;

use std::result;

use binary::ParseAction;

#[derive(Debug)]
pub enum State {
    Normal,
    OpcodeUnknown,
    OperandExpected,
}

type Result<T> = result::Result<T, State>;

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

    pub fn module(self) -> mr::Module {
        self.module
    }

    fn require_capability(&mut self, capability: mr::Operand) {
        if let mr::Operand::Capability(cap) = capability {
            self.module
                .capabilities
                .push(cap)

        } else {
            // TODO(antiagainst): we should return a suitable error here.
            panic!()
        }

    }

    fn enable_extension(&mut self, extension: mr::Operand) {
        if let mr::Operand::LiteralString(ext) = extension {
            self.module.extensions.push(ext)
        } else {
            panic!()
        }
    }

    fn attach_name(&mut self, id: mr::Operand, name: mr::Operand) {
        if let (mr::Operand::IdRef(id_ref), mr::Operand::LiteralString(name_str)) = (id, name) {
            self.module.names.insert(id_ref, name_str);
        } else {
            panic!()
        }
    }
}

impl binary::Consumer for Loader {
    fn consume_header(&mut self, header: mr::ModuleHeader) -> ParseAction {
        self.module.header = Some(header);
        ParseAction::Continue
    }

    fn consume_instruction(&mut self, inst: mr::Instruction) -> ParseAction {
        let mut inst = inst;
        let opcode = inst.class.opcode;
        match opcode {
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
                if let (mr::Operand::AddressingModel(am), mr::Operand::MemoryModel(mm)) = (address,
                                                                                           memory) {
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
            opcode if grammar::reflect::is_type(opcode) || grammar::reflect::is_constant(opcode) ||
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
        ParseAction::Continue
    }
}

pub fn load(binary: Vec<u8>) -> Option<mr::Module> {
    let mut loader = Loader::new();
    if let Err(err) = binary::parse(binary, &mut loader) {
        println!("{:?}", err)
    }
    Some(loader.module())
}
