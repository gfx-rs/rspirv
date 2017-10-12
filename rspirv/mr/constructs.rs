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

/// Data representation of a SPIR-V module.
///
/// Most of the fields are just vectors of `Instruction`s, but some fields
/// store values decomposed from `Instruction`s for better investigation.
///
/// The order of its fields basically reveal the requirements in the
/// [Logical Layout of a Module](https://goo.gl/2kVnfX) of the SPIR-V
/// of the SPIR-V specification.
#[derive(Debug, Default)]
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

/// Data representation of a SPIR-V module header.
#[derive(Debug, PartialEq)]
pub struct ModuleHeader {
    pub magic_number: Word,
    pub version: Word,
    pub generator: Word,
    pub bound: Word,
    pub reserved_word: Word,
}

/// Data representation of a SPIR-V function.
#[derive(Debug, Default)]
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

/// Data representation of a SPIR-V basic block.
#[derive(Debug, Default)]
pub struct BasicBlock {
    /// The label starting this basic block.
    pub label: Option<Instruction>,
    /// Instructions in this basic block.
    pub instructions: Vec<Instruction>,
}

/// Data representation of a SPIR-V instruction.
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
        if let Some(ref i) = self.memory_model {
            insts.push(i);
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
    pub fn new(bound: Word) -> ModuleHeader {
        ModuleHeader {
            magic_number: spirv::MAGIC_NUMBER,
            version: (spirv::MAJOR_VERSION << 16) | (spirv::MAJOR_VERSION << 8),
            generator: 0x000f0000, // TODO: lower 16-bit: tool version number
            bound: bound,
            reserved_word: 0,
        }
    }

    /// Returns the major and minor version numbers as a tuple.
    pub fn version(&self) -> (u8, u8) {
        (((self.version & 0xff0000) >> 16) as u8, ((self.version & 0xff00) >> 8) as u8)
    }

    /// Returns the generator's name and version as a tuple.
    pub fn generator(&self) -> (&str, u16) {
        let tool = (self.generator & 0xffff0000) >> 16;
        let version = (self.generator & 0xffff) as u16;
        let tool: &str = match tool {
            0 => "The Khronos Group",
            1 => "LunarG",
            2 => "Valve",
            3 => "Codeplay",
            4 => "NVIDIA",
            5 => "ARM",
            6 => "LLVM/SPIR-V Translator",
            7 => "SPIR-V Tools Assembler",
            8 => "Glslang",
            9 => "Qualcomm",
            10 => "AMD",
            11 => "Intel",
            12 => "Imagination",
            13 => "Shaderc",
            14 => "spiregg",
            15 => "rspirv",
            _ => "Unknown",
        };
        (tool, version)
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
            class: grammar::CoreInstructionTable::get(opcode),
            result_type: result_type,
            result_id: result_id,
            operands: operands,
        }
    }
}

// Sadly cannot use impl<T: Into<String>> here.
impl<'a> convert::From<&'a str> for Operand {
    fn from(val: &'a str) -> Self {
        Operand::LiteralString(val.to_owned())
    }
}

impl convert::From<u32> for Operand {
    /// Converts the given `u32` `val` into an `Operand::LiteralInt32`.
    fn from(val: u32) -> Self {
        Operand::LiteralInt32(val)
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
        assert_eq!(mr::Operand::LoopControl(spirv::LoopControl::DONT_UNROLL |
                                            spirv::LoopControl::UNROLL),
                   mr::Operand::from(spirv::LoopControl::DONT_UNROLL | spirv::LoopControl::UNROLL));
        assert_eq!(mr::Operand::MemoryAccess(spirv::MemoryAccess::NONE),
                   mr::Operand::from(spirv::MemoryAccess::NONE));
    }

    #[test]
    fn test_convert_from_value_enums() {
        assert_eq!(mr::Operand::BuiltIn(spirv::BuiltIn::Position),
                   mr::Operand::from(spirv::BuiltIn::Position));
        assert_eq!(mr::Operand::Capability(spirv::Capability::Pipes),
                   mr::Operand::from(spirv::Capability::Pipes));
    }

    #[test]
    fn test_convert_from_op() {
        assert_eq!(mr::Operand::LiteralSpecConstantOpInteger(spirv::Op::IAdd),
                   mr::Operand::from(spirv::Op::IAdd));
    }
}
