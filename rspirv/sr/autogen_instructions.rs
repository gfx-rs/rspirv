// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use crate::sr::{storage::Token, Type};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Extension {
    pub(in crate::sr) name: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtInstImport {
    pub(in crate::sr) name: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryModel {
    pub(in crate::sr) addressing_model: spirv::AddressingModel,
    pub(in crate::sr) memory_model: spirv::MemoryModel,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryPoint {
    pub(in crate::sr) execution_model: spirv::ExecutionModel,
    pub(in crate::sr) entry_point: spirv::Word,
    pub(in crate::sr) name: String,
    pub(in crate::sr) interface: Vec<spirv::Word>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionMode {
    pub(in crate::sr) entry_point: spirv::Word,
    pub(in crate::sr) mode: spirv::ExecutionMode,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Capability {
    pub(in crate::sr) capability: spirv::Capability,
}
#[derive(Clone, Debug)]
pub struct Function {
    pub(in crate::sr) function_control: spirv::FunctionControl,
    pub(in crate::sr) function_type: Token<Type>,
}
#[derive(Clone, Debug)]
pub struct FunctionParameter {}
#[derive(Clone, Debug)]
pub struct FunctionEnd {}
#[derive(Clone, Debug)]
pub struct Label {}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionModeId {
    pub(in crate::sr) entry_point: spirv::Word,
    pub(in crate::sr) mode: spirv::ExecutionMode,
}
