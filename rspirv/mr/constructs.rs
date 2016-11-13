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
use std::collections::BTreeMap;
use super::operand::Operand;

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
    pub header: Option<ModuleHeader>,
    pub capabilities: Vec<spirv::Capability>,
    pub extensions: Vec<Instruction>,
    pub ext_inst_imports: Vec<Instruction>,
    /// Addressing model. A part of the OpMemoryModel instruction.
    pub addressing_model: Option<spirv::AddressingModel>,
    /// Memory model. A part of the OpMemoryModel instruction.
    pub memory_model: Option<spirv::MemoryModel>,
    pub entry_points: Vec<Instruction>,
    pub execution_modes: Vec<Instruction>,
    /// All non-location debug instructions except name instructions.
    pub debugs: Vec<Instruction>,
    /// All OpName and OpMemberName instructions.
    pub names: BTreeMap<Word, String>,
    pub annotations: Vec<Instruction>,
    /// All types, constants, and global variables.
    ///
    /// As per the specification, they have to be bundled together
    /// because they can depend on one another.
    pub types_global_values: Vec<Instruction>,
    pub functions: Vec<Function>,
}

/// Memory representation of a SPIR-V module header.
#[derive(Debug, PartialEq)]
pub struct ModuleHeader {
    magic_number: Word,
    version: Word,
    generator: Word,
    bound: Word,
    reserved_word: Word,
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

impl Module {
    /// Creates a new empty `Module` instance.
    pub fn new() -> Module {
        Module {
            header: None,
            capabilities: vec![],
            extensions: vec![],
            ext_inst_imports: vec![],
            addressing_model: None,
            memory_model: None,
            entry_points: vec![],
            execution_modes: vec![],
            debugs: vec![],
            names: BTreeMap::new(),
            annotations: vec![],
            types_global_values: vec![],
            functions: vec![],
        }
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
        (((self.version & 0xff0000) >> 16) as u8,
         ((self.version & 0xff00) >> 8) as u8)
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
