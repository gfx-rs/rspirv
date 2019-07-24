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

use crate::binary;
use crate::mr;
use crate::spirv;
use crate::grammar;

use crate::binary::{ParseAction, ParseResult};
use std::{error, fmt};

/// Data representation loading errors.
#[derive(Debug)]
pub enum Error {
    NestedFunction,
    UnclosedFunction,
    MismatchedFunctionEnd,
    DetachedFunctionParameter,
    DetachedBasicBlock,
    NestedBasicBlock,
    UnclosedBasicBlock,
    MismatchedTerminator,
    DetachedInstruction,
    WrongOpCapabilityOperand,
    WrongOpExtensionOperand,
    WrongOpExtInstImportOperand,
    WrongOpMemoryModelOperand,
    WrongOpNameOperand,
}

impl Error {
    /// Gives an descriptive string for each error.
    ///
    /// This method is intended to be used by fmt::Display and error::Error to
    /// avoid duplication in implementation. So it's private.
    fn describe(&self) -> &str {
        match *self {
            Error::NestedFunction => "found nested function",
            Error::UnclosedFunction => "found unclosed function",
            Error::MismatchedFunctionEnd => "found mismatched OpFunctionEnd",
            Error::DetachedFunctionParameter => {
                "found function OpFunctionParameter not inside function"
            }
            Error::DetachedBasicBlock => "found basic block not inside function",
            Error::NestedBasicBlock => "found nested basic block",
            Error::UnclosedBasicBlock => "found basic block without terminator",
            Error::MismatchedTerminator => "found mismatched terminator",
            Error::DetachedInstruction => "found instruction not inside basic block",
            Error::WrongOpCapabilityOperand => "wrong OpCapability operand",
            Error::WrongOpExtensionOperand => "wrong OpExtension operand",
            Error::WrongOpExtInstImportOperand => "wrong OpExtInstImport operand",
            Error::WrongOpMemoryModelOperand => "wrong OpMemoryModel operand",
            Error::WrongOpNameOperand => "wrong OpName operand",
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.describe()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}

/// The data representation loader.
///
/// Constructs a [`Module`](struct.Module.html) from the module header and
/// instructions.
///
/// It implements the [`Consumer`](../binary/trait.Consumer.html) trait and
/// works with the [`Parser`](../binary/struct.Parser.html).
#[derive(Default)]
pub struct Loader {
    module: mr::Module,
    function: Option<mr::Function>,
    block: Option<mr::BasicBlock>,
}

impl Loader {
    /// Creates a new empty loader.
    pub fn new() -> Loader {
        Loader {
            module: mr::Module::new(),
            function: None,
            block: None,
        }
    }

    /// Returns the `Module` under construction.
    pub fn module(self) -> mr::Module {
        self.module
    }
}

/// Returns `$error` if `$condition` evaluates to false.
macro_rules! if_ret_err {
    ($condition: expr, $error: ident) => (if $condition {
        return ParseAction::Error(Box::new(Error::$error))
    });
}

impl binary::Consumer for Loader {
    fn initialize(&mut self) -> ParseAction {
        ParseAction::Continue
    }

    fn finalize(&mut self) -> ParseAction {
        if_ret_err!(self.block.is_some(), UnclosedBasicBlock);
        if_ret_err!(self.function.is_some(), UnclosedFunction);
        ParseAction::Continue
    }

    fn consume_header(&mut self, header: mr::ModuleHeader) -> ParseAction {
        self.module.header = Some(header);
        ParseAction::Continue
    }

    fn consume_instruction(&mut self, inst: mr::Instruction) -> ParseAction {
        let opcode = inst.class.opcode;
        match opcode {
            spirv::Op::Capability => self.module.capabilities.push(inst),
            spirv::Op::Extension => self.module.extensions.push(inst),
            spirv::Op::ExtInstImport => self.module.ext_inst_imports.push(inst),
            spirv::Op::MemoryModel => self.module.memory_model = Some(inst),
            spirv::Op::EntryPoint => self.module.entry_points.push(inst),
            spirv::Op::ExecutionMode => self.module.execution_modes.push(inst),
            opcode if grammar::reflect::is_nonlocation_debug(opcode) => {
                self.module.debugs.push(inst)
            }
            opcode if grammar::reflect::is_annotation(opcode) => self.module.annotations.push(inst),
            opcode if grammar::reflect::is_type(opcode) ||
                      grammar::reflect::is_constant(opcode) => {
                self.module.types_global_values.push(inst)
            }
            spirv::Op::Variable if self.function.is_none() => {
                self.module.types_global_values.push(inst)
            }
            spirv::Op::Undef if self.function.is_none() => {
                self.module.types_global_values.push(inst)
            }
            spirv::Op::Function => {
                if_ret_err!(self.function.is_some(), NestedFunction);
                let mut f = mr::Function::new();
                f.def = Some(inst);
                self.function = Some(f)
            }
            spirv::Op::FunctionEnd => {
                if_ret_err!(self.function.is_none(), MismatchedFunctionEnd);
                if_ret_err!(self.block.is_some(), UnclosedBasicBlock);
                self.function.as_mut().unwrap().end = Some(inst);
                self.module.functions.push(self.function.take().unwrap())
            }
            spirv::Op::FunctionParameter => {
                if_ret_err!(self.function.is_none(), DetachedFunctionParameter);
                self.function.as_mut().unwrap().parameters.push(inst);
            }
            spirv::Op::Label => {
                if_ret_err!(self.function.is_none(), DetachedBasicBlock);
                if_ret_err!(self.block.is_some(), NestedBasicBlock);
                let mut block = mr::BasicBlock::new();
                block.label = Some(inst);
                self.block = Some(block)
            }
            opcode if grammar::reflect::is_terminator(opcode) => {
                // Make sure the block exists here. Once the block exists,
                // we are certain the function exists because the above checks.
                if_ret_err!(self.block.is_none(), MismatchedTerminator);
                self.block.as_mut().unwrap().instructions.push(inst);
                self.function
                    .as_mut()
                    .unwrap()
                    .basic_blocks
                    .push(self.block.take().unwrap())
            }
            _ => {
                if_ret_err!(self.block.is_none(), DetachedInstruction);
                self.block.as_mut().unwrap().instructions.push(inst)
            }
        }
        ParseAction::Continue
    }
}

/// Loads the SPIR-V `binary` into memory and returns a `Module`.
///
/// # Examples
///
/// ```
/// use rspirv;
/// use rspirv::binary::Disassemble;
///
/// let buffer: Vec<u8> = vec![
///     // Magic number.           Version number: 1.0.
///     0x03, 0x02, 0x23, 0x07,    0x00, 0x00, 0x01, 0x00,
///     // Generator number: 0.    Bound: 0.
///     0x00, 0x00, 0x00, 0x00,    0x00, 0x00, 0x00, 0x00,
///     // Reserved word: 0.
///     0x00, 0x00, 0x00, 0x00,
///     // OpMemoryModel.          Logical.
///     0x0e, 0x00, 0x03, 0x00,    0x00, 0x00, 0x00, 0x00,
///     // GLSL450.
///     0x01, 0x00, 0x00, 0x00];
///
/// let dis = match rspirv::mr::load_bytes(buffer) {
///     Ok(module) => module.disassemble(),
///     Err(err) => format!("{}", err),
/// };
///
/// assert_eq!(dis,
///            "; SPIR-V\n\
///             ; Version: 1.0\n\
///             ; Generator: rspirv\n\
///             ; Bound: 0\n\
///             OpMemoryModel Logical GLSL450");
/// ```
pub fn load_bytes<T: AsRef<[u8]>>(binary: T) -> ParseResult<mr::Module> {
    let mut loader = Loader::new();
    binary::parse_bytes(binary, &mut loader)?;
    Ok(loader.module())
}

/// Loads the SPIR-V `binary` into memory and returns a `Module`.
///
/// # Examples
///
/// ```
/// use rspirv;
/// use rspirv::binary::Disassemble;
///
/// let buffer: Vec<u32> = vec![
///     0x07230203,  // Magic number
///     0x00010000,  // Version number: 1.0
///     0x00000000,  // Generator number: 0
///     0x00000000,  // Bound: 0
///     0x00000000,  // Reserved word: 0
///     0x0003000e,  // OpMemoryModel
///     0x00000000,  // Logical
///     0x00000001,  // GLSL450
/// ];
///
/// let dis = match rspirv::mr::load_words(buffer) {
///     Ok(module) => module.disassemble(),
///     Err(err) => format!("{}", err),
/// };
///
/// assert_eq!(dis,
///            "; SPIR-V\n\
///             ; Version: 1.0\n\
///             ; Generator: rspirv\n\
///             ; Bound: 0\n\
///             OpMemoryModel Logical GLSL450");
/// ```
pub fn load_words<T: AsRef<[u32]>>(binary: T) -> ParseResult<mr::Module> {
    let mut loader = Loader::new();
    binary::parse_words(binary, &mut loader)?;
    Ok(loader.module())
}

#[cfg(test)]
mod tests {
    use crate::mr;
    use crate::spirv;

    #[test]
    fn test_load_variable() {
        let mut b = mr::Builder::new();

        let void = b.type_void();
        let float = b.type_float(32);
        let voidfvoid = b.type_function(void, vec![void]);

        // Global variable
        let global = b.variable(float, None, spirv::StorageClass::Input, None);

        b.begin_function(void, None, spirv::FunctionControl::NONE, voidfvoid).unwrap();
        b.begin_basic_block(None).unwrap();
        // Local variable
        let local = b.variable(float, None, spirv::StorageClass::Function, None);
        b.ret().unwrap();
        b.end_function().unwrap();

        let m = b.module();

        assert_eq!(m.types_global_values.len(), 4);
        let inst = &m.types_global_values[3];
        assert_eq!(inst.class.opcode, spirv::Op::Variable);
        assert_eq!(inst.result_id.unwrap(), global);

        assert_eq!(m.functions.len(), 1);
        let f = &m.functions[0];
        assert_eq!(f.basic_blocks.len(), 1);
        let bb = &f.basic_blocks[0];
        assert!(bb.instructions.len() > 1);
        let inst = &bb.instructions[0];
        assert_eq!(inst.class.opcode, spirv::Op::Variable);
        assert_eq!(inst.result_id.unwrap(), local);
    }

    #[test]
    fn test_load_undef() {
        let mut b = mr::Builder::new();

        let void = b.type_void();
        let float = b.type_float(32);
        let voidfvoid = b.type_function(void, vec![void]);

        // Global variable
        let global = b.undef(float, None);

        b.begin_function(void, None, spirv::FunctionControl::NONE, voidfvoid).unwrap();
        b.begin_basic_block(None).unwrap();
        // Local variable
        let local = b.undef(float, None);
        b.ret().unwrap();
        b.end_function().unwrap();

        let m = b.module();

        assert_eq!(m.types_global_values.len(), 4);
        let inst = &m.types_global_values[3];
        assert_eq!(inst.class.opcode, spirv::Op::Undef);
        assert_eq!(inst.result_id.unwrap(), global);

        assert_eq!(m.functions.len(), 1);
        let f = &m.functions[0];
        assert_eq!(f.basic_blocks.len(), 1);
        let bb = &f.basic_blocks[0];
        assert!(bb.instructions.len() > 1);
        let inst = &bb.instructions[0];
        assert_eq!(inst.class.opcode, spirv::Op::Undef);
        assert_eq!(inst.result_id.unwrap(), local);
    }
}
