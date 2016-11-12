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

//! The module containing the whole SPIR-V syntax grammar.
//!
//! It defines the syntax grammar of all instructions (their layouts
//! and operands).
//!
//! It also provides many reflect functions.

pub use self::syntax::{ExtendedInstruction, GlslStd450InstructionTable};
pub use self::syntax::{Instruction, InstructionTable};
pub use self::syntax::{OperandKind, OperandQuantifier};

pub mod reflect;
mod syntax;
