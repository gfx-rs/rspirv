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

use binary::{ParseAction, ParseResult};
use std::{error, fmt, result};

/// Memory representation loading errors.
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
            Error::DetachedBasicBlock => {
                "found basic block not inside function"
            }
            Error::NestedBasicBlock => "found nested basic block",
            Error::UnclosedBasicBlock => "found basic block without terminator",
            Error::MismatchedTerminator => "found mismatched terminator",
            Error::DetachedInstruction => {
                "found instruction not inside basic block"
            }
            Error::WrongOpCapabilityOperand => "wrong OpCapability operand",
            Error::WrongOpExtensionOperand => "wrong OpExtension operand",
            Error::WrongOpExtInstImportOperand => {
                "wrong OpExtInstImport operand"
            }
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

type Result<T> = result::Result<T, Error>;

/// The memory representation loader.
///
/// Constructs a [`Module`](struct.Module.html) from the module header and
/// instructions.
///
/// It implements the [`Consumer`](../binary/trait.Consumer.html) trait and
/// works with the [`Parser`](../binary/struct.Parser.html).
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

    fn require_capability(&mut self,
                          capability: Option<mr::Operand>)
                          -> Result<()> {
        if let Some(mr::Operand::Capability(cap)) = capability {
            Ok(self.module.capabilities.push(cap))
        } else {
            Err(Error::WrongOpCapabilityOperand)
        }
    }

    fn attach_name(&mut self,
                   id: Option<mr::Operand>,
                   name: Option<mr::Operand>)
                   -> Result<()> {
        if let (Some(mr::Operand::IdRef(id_ref)),
                Some(mr::Operand::LiteralString(name_str))) = (id, name) {
            self.module.names.insert(id_ref, name_str);
            Ok(())
        } else {
            Err(Error::WrongOpNameOperand)
        }
    }
}

/// Returns `$error` if `$condition` evaluates to false.
macro_rules! if_ret_err {
    ($condition: expr, $error: ident) => (if $condition {
        return ParseAction::Error(Box::new(Error::$error))
    });
}

/// Converts possible error from the given `$result` to a ParseAction::Error
/// and returns it.
macro_rules! try_call {
    ($result: expr) => (match $result {
        Ok(_) => (),
        Err(err) => return ParseAction::Error(Box::new(err))
    })
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
        let mut inst = inst;
        let opcode = inst.class.opcode;
        match opcode {
            spirv::Op::Capability => {
                try_call!(self.require_capability(inst.operands.pop()))
            }
            spirv::Op::Extension => self.module.extensions.push(inst),
            spirv::Op::ExtInstImport => self.module.ext_inst_imports.push(inst),
            spirv::Op::MemoryModel => {
                if let Some(mr::Operand::MemoryModel(model)) = inst.operands
                    .pop() {
                    self.module.memory_model = Some(model)
                } else {
                    return ParseAction::Error(
                        Box::new(Error::WrongOpMemoryModelOperand));
                }
                if let Some(mr::Operand::AddressingModel(model)) = inst.operands
                    .pop() {
                    self.module.addressing_model = Some(model)
                } else {
                    return ParseAction::Error(
                        Box::new(Error::WrongOpMemoryModelOperand));
                }
            }
            spirv::Op::EntryPoint => self.module.entry_points.push(inst),
            spirv::Op::ExecutionMode => self.module.execution_modes.push(inst),
            spirv::Op::Name => {
                let name = inst.operands.pop();
                let id = inst.operands.pop();
                try_call!(self.attach_name(id, name))
            }
            opcode if grammar::reflect::is_nonlocation_debug(opcode) => {
                self.module.debugs.push(inst)
            }
            opcode if grammar::reflect::is_annotation(opcode) => {
                self.module.annotations.push(inst)
            }
            opcode if grammar::reflect::is_type(opcode) ||
                      grammar::reflect::is_constant(opcode) ||
                      grammar::reflect::is_variable(opcode) => {
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
pub fn load(binary: Vec<u8>) -> ParseResult<mr::Module> {
    let mut loader = Loader::new();
    try!(binary::parse(binary, &mut loader));
    Ok(loader.module())
}
