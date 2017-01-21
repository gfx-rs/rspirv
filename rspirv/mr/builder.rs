// Copyright 2017 Google Inc.
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

#![allow(dead_code)]

use mr;
use spirv;

use std::result;
use super::Error;

type BuildResult<T> = result::Result<T, Error>;

/// The memory representation builder.
///
/// Constructs a [`Module`](struct.Module.html) by aggregating results from
/// method calls for various instructions. Most of the methods return the
/// SPIR-V id assigned for that SPIR-V instruction.
pub struct Builder {
    module: mr::Module,
    next_id: u32,
    function: Option<mr::Function>,
    basic_block: Option<mr::BasicBlock>,
}

impl Builder {
    /// Creates a new empty builder.
    pub fn new() -> Builder {
        Builder {
            module: mr::Module::new(),
            next_id: 1,
            function: None,
            basic_block: None,
        }
    }

    /// Returns the `Module` under construction.
    pub fn module(self) -> mr::Module {
        self.module
    }

    /// Begins building of a new function.
    pub fn begin_function(&mut self,
                          return_type: spirv::Word,
                          control: spirv::FunctionControl,
                          function_type: spirv::Word)
                          -> BuildResult<spirv::Word> {
        if self.function.is_some() {
            return Err(Error::NestedFunction);
        }

        let id = self.next_id;
        self.next_id += 1;

        let mut f = mr::Function::new();
        f.def = Some(mr::Instruction::new(spirv::Op::Function,
                                          Some(return_type),
                                          Some(id),
                                          vec![mr::Operand::FunctionControl(control),
                                               mr::Operand::IdRef(function_type)]));
        self.function = Some(f);
        Ok(id)
    }

    /// Ends building of the current function.
    pub fn end_function(&mut self) -> BuildResult<()> {
        if self.function.is_none() {
            return Err(Error::MismatchedFunctionEnd);
        }

        let mut f = self.function.take().unwrap();
        f.end = Some(mr::Instruction::new(spirv::Op::FunctionEnd, None, None, vec![]));
        Ok(self.module.functions.push(f))
    }

    /// Begins building of a new basic block.
    pub fn begin_basic_block(&mut self) -> BuildResult<spirv::Word> {
        if self.function.is_none() {
            return Err(Error::DetachedBasicBlock);
        }
        if self.basic_block.is_some() {
            return Err(Error::NestedBasicBlock);
        }

        let id = self.next_id;
        self.next_id += 1;

        let mut bb = mr::BasicBlock::new();
        bb.label = Some(mr::Instruction::new(spirv::Op::Label, None, None, vec![]));

        self.basic_block = Some(bb);
        Ok(id)
    }

    fn end_basic_block(&mut self) {
        if self.basic_block.is_none() {
            panic!("internal error: end_basic_block() called without enclosing basic block");
        }

        self.function.as_mut().unwrap().basic_blocks.push(self.basic_block.take().unwrap());
    }
}

include!("build_type.rs");
