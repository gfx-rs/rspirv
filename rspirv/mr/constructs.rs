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

use grammar;
use spirv;

use spirv::Word;
use std::{convert, fmt, iter};

/// Memory representation of a SPIR-V module.
///
/// Most of the fields are just vectors of `Instruction`s, but some fields
/// store values decomposed from `Instruction`s for better investigation.
///
/// The order of its fields basically reveal the requirements in the
/// [Logical Layout of a Module](https://goo.gl/2kVnfX) of the SPIR-V
/// of the SPIR-V specification.
#[derive(Debug)]
pub struct Module {
    /// The module header.
    pub header: Option<ModuleHeader>,
    /// All OpCapability instructions.
    pub capabilities: Vec<Instruction>,
    /// All OpExtension instructions.
    pub extensions: Vec<Instruction>,
    /// All OpExtInstImport instructions.
    pub ext_inst_imports: Vec<Instruction>,
    /// The OpMemoryModel instruction.
    ///
    /// Although it is required by the specification to appear exactly once
    /// per module, we keep it optional here to allow flexibility.
    pub memory_model: Option<Instruction>,
    /// All entry point declarations, using OpEntryPoint.
    pub entry_points: Vec<Instruction>,
    /// All execution mode declarations, using OpExecutionMode.
    pub execution_modes: Vec<Instruction>,
    /// All non-location debug instructions.
    pub debugs: Vec<Instruction>,
    /// All annotation instructions.
    pub annotations: Vec<Instruction>,
    /// All types, constants, and global variables.
    ///
    /// As per the specification, they have to be bundled together
    /// because they can depend on one another.
    pub types_global_values: Vec<Instruction>,
    /// All functions.
    pub functions: Vec<Function>,
}

/// Memory representation of a SPIR-V module header.
#[derive(Debug, PartialEq)]
pub struct ModuleHeader {
    pub magic_number: Word,
    pub version: Word,
    pub generator: Word,
    pub bound: Word,
    pub reserved_word: Word,
}

/// Memory representation of a SPIR-V function.
#[derive(Debug)]
pub struct Function {
    /// First (defining) instruction in this function.
    pub def: Option<Instruction>,
    /// Last (ending) instruction in this function.
    pub end: Option<Instruction>,
    /// Function parameters.
    pub parameters: Vec<Instruction>,
    /// Basic blocks in this function.
    pub basic_blocks: Vec<BasicBlock>,
}

/// Memory representation of a SPIR-V basic block.
#[derive(Debug)]
pub struct BasicBlock {
    /// The label starting this basic block.
    pub label: Option<Instruction>,
    /// Instructions in this basic block.
    pub instructions: Vec<Instruction>,
}

/// Memory representation of a SPIR-V instruction.
#[derive(Debug)]
pub struct Instruction {
    /// The class (grammar specification) of this instruction.
    pub class: &'static grammar::Instruction<'static>,
    /// Result type id.
    pub result_type: Option<Word>,
    /// Result id.
    pub result_id: Option<Word>,
    /// Operands.
    pub operands: Vec<Operand>,
}

/// Instruction iterator.
pub struct InstIter<'i> {
    instructions: Vec<&'i Instruction>,
    index: usize,
}

impl<'i> InstIter<'i> {
    pub fn new(insts: Vec<&'i Instruction>) -> InstIter<'i> {
        InstIter {
            instructions: insts,
            index: 0,
        }
    }
}

impl<'i> iter::Iterator for InstIter<'i> {
    type Item = &'i Instruction;

    fn next(&mut self) -> Option<&'i Instruction> {
        if self.index < self.instructions.len() {
            let inst = self.instructions[self.index];
            self.index += 1;
            Some(inst)
        } else {
            None
        }
    }
}

include!("operand.rs");

impl Module {
    /// Creates a new empty `Module` instance.
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
            annotations: vec![],
            types_global_values: vec![],
            functions: vec![],
        }
    }

    /// Returns an iterator over all global instructions.
    ///
    /// This method internally creates a vector of references to all global
    /// instructions, therefore it has some overheads.
    pub fn global_inst_iter(&self) -> InstIter {
        let mut insts = vec![];
        let mut i: Vec<&Instruction> = self.capabilities.iter().collect();
        insts.append(&mut i);
        let mut i: Vec<&Instruction> = self.extensions.iter().collect();
        insts.append(&mut i);
        let mut i: Vec<&Instruction> = self.ext_inst_imports.iter().collect();
        insts.append(&mut i);
        match self.memory_model {
            Some(ref i) => insts.push(&i),
            None => (),
        }
        let mut i: Vec<&Instruction> = self.entry_points.iter().collect();
        insts.append(&mut i);
        let mut i: Vec<&Instruction> = self.execution_modes.iter().collect();
        insts.append(&mut i);
        let mut i: Vec<&Instruction> = self.debugs.iter().collect();
        insts.append(&mut i);
        let mut i: Vec<&Instruction> = self.annotations.iter().collect();
        insts.append(&mut i);
        let mut i: Vec<&Instruction> = self.types_global_values.iter().collect();
        insts.append(&mut i);
        InstIter::new(insts)
    }
}

impl ModuleHeader {
    /// Creates a new `ModuleHeader` instance.
    pub fn new(magic_number: Word,
               version: Word,
               generator: Word,
               bound: Word,
               reserved_word: Word)
               -> ModuleHeader {
        ModuleHeader {
            magic_number: magic_number,
            version: version,
            generator: generator,
            bound: bound,
            reserved_word: reserved_word,
        }
    }

    /// Returns the major and minor version numbers as a tuple.
    pub fn version(&self) -> (u8, u8) {
        (((self.version & 0xff0000) >> 16) as u8, ((self.version & 0xff00) >> 8) as u8)
    }

    /// Returns the generator's name and version as a tuple.
    pub fn generator(&self) -> (&str, u16) {
        let vendor = (self.generator & 0xffff0000) >> 16;
        let version = (self.generator & 0xffff) as u16;
        let vendor: &str = match vendor {
            0 => "Khronos Group",
            1 => "LunarG",
            2 => "Valve",
            3 => "Codeplay",
            4 => "NVIDIA",
            5 => "ARM",
            6 => "LLVM/SPIR-V Translator",
            7 => "SPIRV-Tools",
            8 => "Glslang",
            9 => "Qualcomm",
            10 => "AMD",
            11 => "Intel",
            _ => "Unknown",
        };
        (vendor, version)
    }

    /// Returns the id bound.
    pub fn bound(&self) -> Word {
        self.bound
    }

    /// Sets the id bound to the given `bound`.
    pub fn set_bound(&mut self, bound: Word) {
        self.bound = bound
    }
}

impl Function {
    /// Creates a new empty `Function` instance.
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
    /// Creates a new empty `BasicBlock` instance.
    pub fn new() -> BasicBlock {
        BasicBlock {
            label: None,
            instructions: vec![],
        }
    }
}

impl Instruction {
    /// Creates a new `Instruction` instance.
    pub fn new(opcode: spirv::Op,
               result_type: Option<Word>,
               result_id: Option<Word>,
               operands: Vec<Operand>)
               -> Instruction {
        Instruction {
            class: grammar::InstructionTable::get(opcode),
            result_type: result_type,
            result_id: result_id,
            operands: operands,
        }
    }
}

// Sadly cannot use impl<T: Into<String>> here.
impl<'a> convert::From<&'a str> for Operand {
    fn from(val: &'a str) -> Self {
        Operand::LiteralString(val.to_string())
    }
}

impl convert::From<String> for Operand {
    fn from(val: String) -> Self {
        Operand::LiteralString(val)
    }
}

impl convert::From<u32> for Operand {
    fn from(val: u32) -> Self {
        Operand::LiteralInt32(val)
    }
}

impl convert::From<u64> for Operand {
    fn from(val: u64) -> Self {
        Operand::LiteralInt64(val)
    }
}

impl convert::From<f32> for Operand {
    fn from(val: f32) -> Self {
        Operand::LiteralFloat32(val)
    }
}

impl convert::From<f64> for Operand {
    fn from(val: f64) -> Self {
        Operand::LiteralFloat64(val)
    }
}

#[cfg(test)]
mod tests {
    use mr;
    use spirv;

    #[test]
    fn test_convert_from_string() {
        assert_eq!(mr::Operand::LiteralString("wow".to_string()),
                   mr::Operand::from("wow"));
        assert_eq!(mr::Operand::LiteralString("wow".to_string()),
                   mr::Operand::from("wow".to_string()));
    }

    #[test]
    fn test_convert_from_numbers() {
        assert_eq!(mr::Operand::LiteralInt32(16u32), mr::Operand::from(16u32));
        assert_eq!(mr::Operand::LiteralInt64(128934u64),
                   mr::Operand::from(128934u64));
        assert_eq!(mr::Operand::LiteralFloat32(3.14f32),
                   mr::Operand::from(3.14f32));
        assert_eq!(mr::Operand::LiteralFloat64(10.4235f64),
                   mr::Operand::from(10.4235f64));
    }

    #[test]
    fn test_convert_from_bit_enums() {
        assert_eq!(mr::Operand::LoopControl(spirv::LOOP_CONTROL_DONT_UNROLL |
                                            spirv::LOOP_CONTROL_UNROLL),
                   mr::Operand::from(spirv::LOOP_CONTROL_DONT_UNROLL | spirv::LOOP_CONTROL_UNROLL));
        assert_eq!(mr::Operand::MemoryAccess(spirv::MEMORY_ACCESS_NONE),
                   mr::Operand::from(spirv::MEMORY_ACCESS_NONE));
    }

    #[test]
    fn test_convert_from_value_enums() {
        assert_eq!(mr::Operand::BuiltIn(spirv::BuiltIn::Position),
                   mr::Operand::from(spirv::BuiltIn::Position));
        assert_eq!(mr::Operand::Capability(spirv::Capability::Pipes),
                   mr::Operand::from(spirv::Capability::Pipes));
    }
}
