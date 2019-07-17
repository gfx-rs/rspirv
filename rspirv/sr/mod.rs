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

//! **S**tructured **r**epresentation of various SPIR-V language constructs.

pub use self::autogen_decoration::Decoration;
pub use self::constants::{Constant};
pub use self::context::{Context, Token};
pub use self::instructions::{Instruction, Terminator};
pub use self::items::{Function, Variable, Module};

mod autogen_decoration;
mod constants;
mod context;
mod instructions;
mod items;
pub mod structs;
pub mod types;


#[derive(Clone, Debug)]
pub enum OperandError {
    Wrong,
    Missing,
    Incomplete,
}

#[derive(Clone, Debug)]
pub enum LiftError {
    Class,
    OpCode,
    Operand(OperandError),
}

impl From<OperandError> for LiftError {
    fn from(error: OperandError) -> Self {
        LiftError::Operand(error)
    }
}
