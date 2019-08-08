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

// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use crate::sr::{Token, Type};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Extension {
    name: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtInstImport {
    name: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryModel {
    addressing_model: spirv::AddressingModel,
    memory_model: spirv::MemoryModel,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryPoint {
    execution_model: spirv::ExecutionModel,
    entry_point: spirv::Word,
    name: String,
    interface: Vec<spirv::Word>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionMode {
    entry_point: spirv::Word,
    mode: spirv::ExecutionMode,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Capability {
    capability: spirv::Capability,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
    function_control: spirv::FunctionControl,
    function_type: Token<Type>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionParameter {}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionEnd {}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Label {}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionModeId {
    entry_point: spirv::Word,
    mode: spirv::ExecutionMode,
}
