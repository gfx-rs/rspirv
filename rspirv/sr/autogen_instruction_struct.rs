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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Extension {
    pub name: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtInstImport {
    pub name: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryModel {
    pub addressing_model: spirv::AddressingModel,
    pub memory_model: spirv::MemoryModel,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryPoint {
    pub execution_model: spirv::ExecutionModel,
    pub entry_point: Token<super::Function>,
    pub name: String,
    pub interface: Vec<Token<super::Variable>>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionMode {
    pub entry_point: Token<super::Function>,
    pub mode: spirv::ExecutionMode,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Capability {
    pub capability: spirv::Capability,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
    pub function_control: spirv::FunctionControl,
    pub function_type: Token<super::types::Function>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionParameter {}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionEnd {}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Label {}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionModeId {
    pub entry_point: Token<super::Function>,
    pub mode: spirv::ExecutionMode,
}
