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

#![cfg_attr(feature = "clippy", allow(too_many_arguments))]

use mr;
use spirv;

use std::result;
use super::Error;

type BuildResult<T> = result::Result<T, Error>;

/// The data representation builder.
///
/// Constructs a [`Module`](struct.Module.html) by aggregating results from
/// method calls for various instructions.
///
/// This builder is designed to be low level; its build methods' signatures
/// basically reflects the layout of the corresponding SPIR-V instructions
/// faithfully.
///
/// If a SPIR-V instruction generates a result id and the result id can be
/// forward referenced, the build method will take an optional `result_id`
/// parameter. Filling it with `Some(val)` will instruct the builder to use
/// the given `val` as the result id. For other cases, an unsed result id
/// will be automatically assigned from the builder.
///
/// So for instructions forward referencing an id, to avoid id collision,
/// you can either
///
/// * first append the target instruction generating that id and then append the
///   forward referencing instruction; or
/// * use the `id()` method to get an unused id from the builder, use it in the
///   forward referencing instruction, and then later fill the optional
///   `result_id` parameter of the target instruction with the same id.
///
/// Instructions belonging to the module (e.g., `OpDecorate`) can be appended
/// at any time, no matter that a basic block is currently under construction
/// or not. Intructions that can appear both in the module and basic block
/// (e.g., `OpVariable`) will be inserted to the current basic block under
/// construction first, if any.
///
/// # Errors
///
/// Methods in the builder implement little sanity check; only appending
/// instructions that violates the module structure is guarded. So methods
/// possibly returning errors are basically those related to function and basic
/// block construction (e.g., `OpFunction` and `OpLabel`).
///
/// Errors returned are enumerants related to function structure from the
/// [`Error`](enum.Error.html) enum.
///
/// # Examples
///
/// ```
/// extern crate rspirv;
/// extern crate spirv_headers as spirv;
///
/// use rspirv::binary::Disassemble;
///
/// fn main() {
///     let mut b = rspirv::mr::Builder::new();
///     b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::Simple);
///     let void = b.type_void();
///     let voidf = b.type_function(void, vec![void]);
///     b.begin_function(void,
///                      None,
///                      (spirv::FUNCTION_CONTROL_DONT_INLINE |
///                       spirv::FUNCTION_CONTROL_CONST),
///                      voidf)
///      .unwrap();
///     b.begin_basic_block(None).unwrap();
///     b.ret().unwrap();
///     b.end_function().unwrap();
///
///     assert_eq!(b.module().disassemble(),
///                "; SPIR-V\n\
///                 ; Version: 1.1\n\
///                 ; Generator: rspirv\n\
///                 ; Bound: 5\n\
///                 OpMemoryModel Logical Simple\n\
///                 %1 = OpTypeVoid\n\
///                 %2 = OpTypeFunction %1 %1\n\
///                 %3 = OpFunction  %1  DontInline|Const %2\n\
///                 %4 = OpLabel\n\
///                 OpReturn\n\
///                 OpFunctionEnd");
/// }
/// ```
#[derive(Default)]
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
        let mut module = self.module;
        module.header = Some(mr::ModuleHeader::new(self.next_id));
        module
    }

    /// Returns the next unused id.
    pub fn id(&mut self) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Begins building of a new function.
    ///
    /// If `function_id` is `Some(val)`, then `val` will be used as the result
    /// id of the function under construction; otherwise, an unused result id
    /// will be automatically assigned.
    pub fn begin_function(&mut self,
                          return_type: spirv::Word,
                          function_id: Option<spirv::Word>,
                          control: spirv::FunctionControl,
                          function_type: spirv::Word)
                          -> BuildResult<spirv::Word> {
        if self.function.is_some() {
            return Err(Error::NestedFunction);
        }

        let id = match function_id {
            Some(v) => v,
            None => self.id(),
        };

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

    /// Declares a formal parameter for the current function.
    pub fn function_parameter(&mut self, result_type: spirv::Word) -> BuildResult<spirv::Word> {
        if self.function.is_none() {
            return Err(Error::DetachedFunctionParameter);
        }
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::FunctionParameter,
                                        Some(result_type),
                                        Some(id),
                                        vec![]);
        self.function.as_mut().unwrap().parameters.push(inst);
        Ok(id)
    }

    /// Begins building of a new basic block.
    ///
    /// If `label_id` is `Some(val)`, then `val` will be used as the result
    /// id for the `OpLabel` instruction begining this basic block; otherwise,
    /// a unused result id will be automatically assigned.
    pub fn begin_basic_block(&mut self, label_id: Option<spirv::Word>) -> BuildResult<spirv::Word> {
        if self.function.is_none() {
            return Err(Error::DetachedBasicBlock);
        }
        if self.basic_block.is_some() {
            return Err(Error::NestedBasicBlock);
        }

        let id = match label_id {
            Some(v) => v,
            None => self.id(),
        };

        let mut bb = mr::BasicBlock::new();
        bb.label = Some(mr::Instruction::new(spirv::Op::Label, None, Some(id), vec![]));

        self.basic_block = Some(bb);
        Ok(id)
    }

    pub fn end_basic_block(&mut self, inst: mr::Instruction) -> BuildResult<()> {
        if self.basic_block.is_none() {
            return Err(Error::MismatchedTerminator);
        }

        self.basic_block.as_mut().unwrap().instructions.push(inst);
        Ok(self.function.as_mut().unwrap().basic_blocks.push(self.basic_block.take().unwrap()))
    }

    /// Appends an OpCapability instruction.
    pub fn capability(&mut self, capability: spirv::Capability) {
        let inst = mr::Instruction::new(spirv::Op::Capability,
                                        None,
                                        None,
                                        vec![mr::Operand::Capability(capability)]);
        self.module.capabilities.push(inst);
    }

    /// Appends an OpExtension instruction.
    pub fn extension<T: Into<String>>(&mut self, extension: T) {
        let inst = mr::Instruction::new(spirv::Op::Extension,
                                        None,
                                        None,
                                        vec![mr::Operand::LiteralString(extension.into())]);
        self.module.extensions.push(inst);
    }

    /// Appends an OpExtInstImport instruction and returns the result id.
    pub fn ext_inst_import<T: Into<String>>(&mut self, extended_inst_set: T) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::ExtInstImport,
                                        None,
                                        Some(id),
                                        vec![mr::Operand::LiteralString(extended_inst_set.into())]);
        self.module.ext_inst_imports.push(inst);
        id
    }

    /// Appends an OpMemoryModel instruction.
    pub fn memory_model(&mut self,
                        addressing_model: spirv::AddressingModel,
                        memory_model: spirv::MemoryModel) {
        let inst = mr::Instruction::new(spirv::Op::MemoryModel,
                                        None,
                                        None,
                                        vec![mr::Operand::AddressingModel(addressing_model),
                                             mr::Operand::MemoryModel(memory_model)]);
        self.module.memory_model = Some(inst);
    }

    /// Appends an OpEntryPoint instruction.
    pub fn entry_point<T: Into<String>>(&mut self,
                                        execution_model: spirv::ExecutionModel,
                                        entry_point: spirv::Word,
                                        name: T,
                                        interface: Vec<spirv::Word>) {
        let mut operands = vec![mr::Operand::ExecutionModel(execution_model),
                                mr::Operand::IdRef(entry_point),
                                mr::Operand::LiteralString(name.into())];
        for v in interface {
            operands.push(mr::Operand::IdRef(v));
        }

        let inst = mr::Instruction::new(spirv::Op::EntryPoint, None, None, operands);
        self.module.entry_points.push(inst);
    }

    /// Appends an OpExecutionMode instruction.
    pub fn execution_mode(&mut self,
                          entry_point: spirv::Word,
                          execution_mode: spirv::ExecutionMode,
                          params: Vec<u32>) {
        let mut operands = vec![mr::Operand::IdRef(entry_point),
                                mr::Operand::ExecutionMode(execution_mode)];
        for v in params {
            operands.push(mr::Operand::LiteralInt32(v));
        }

        let inst = mr::Instruction::new(spirv::Op::ExecutionMode, None, None, operands);
        self.module.execution_modes.push(inst);
    }
}

include!("build_type.rs");
include!("build_constant.rs");
include!("build_annotation.rs");
include!("build_terminator.rs");
include!("build_debug.rs");

impl Builder {
    /// Appends an OpDecorationGroup instruction and returns the result id.
    pub fn decoration_group(&mut self) -> spirv::Word {
        let id = self.id();
        self.module
            .annotations
            .push(mr::Instruction::new(spirv::Op::DecorationGroup, None, Some(id), vec![]));
        id
    }

    pub fn string<T: Into<String>>(&mut self, s: T) -> spirv::Word {
        let id = self.id();
        self.module.debugs.push(mr::Instruction::new(spirv::Op::String,
                                                     None,
                                                     Some(id),
                                                     vec![mr::Operand::LiteralString(s.into())]));
        id
    }

    #[allow(unused_variables)]
    pub fn line(&mut self, file: spirv::Word, line: spirv::Word, column: spirv::Word) {
        unimplemented!()
    }
    pub fn no_line(&mut self) {
        unimplemented!()
    }
}

impl Builder {
    /// Appends an OpTypeForwardPointer instruction.
    pub fn type_forward_pointer(&mut self,
                                pointer_type: spirv::Word,
                                storage_class: spirv::StorageClass) {
        self.module
            .types_global_values
            .push(mr::Instruction::new(spirv::Op::TypeForwardPointer,
                                       None,
                                       None,
                                       vec![mr::Operand::IdRef(pointer_type),
                                            mr::Operand::StorageClass(storage_class)]));
    }

    /// Appends an OpTypePointer instruction and returns the result id.
    pub fn type_pointer(&mut self,
                        result_id: Option<spirv::Word>,
                        storage_class: spirv::StorageClass,
                        pointee_type: spirv::Word)
                        -> spirv::Word {
        let id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        self.module
            .types_global_values
            .push(mr::Instruction::new(spirv::Op::TypePointer,
                                       None,
                                       Some(id),
                                       vec![mr::Operand::StorageClass(storage_class),
                                            mr::Operand::IdRef(pointee_type)]));
        id
    }

    /// Appends an OpTypeOpaque instruction and returns the result id.
    pub fn type_opaque<T: Into<String>>(&mut self, type_name: T) -> spirv::Word {
        let id = self.id();
        self.module
            .types_global_values
            .push(mr::Instruction::new(spirv::Op::TypeOpaque,
                                       None,
                                       Some(id),
                                       vec![mr::Operand::LiteralString(type_name.into())]));
        id
    }

    /// Appends an OpConstant instruction with the given 32-bit float `value`.
    /// or the module if no basic block is under construction.
    pub fn constant_f32(&mut self, result_type: spirv::Word, value: f32) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::Constant,
                                        Some(result_type),
                                        Some(id),
                                        vec![mr::Operand::LiteralFloat32(value)]);
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpConstant instruction with the given 32-bit integer `value`.
    /// or the module if no basic block is under construction.
    pub fn constant_u32(&mut self, result_type: spirv::Word, value: u32) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::Constant,
                                        Some(result_type),
                                        Some(id),
                                        vec![mr::Operand::LiteralInt32(value)]);
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpSpecConstant instruction with the given 32-bit float `value`.
    /// or the module if no basic block is under construction.
    pub fn spec_constant_f32(&mut self, result_type: spirv::Word, value: f32) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::SpecConstant,
                                        Some(result_type),
                                        Some(id),
                                        vec![mr::Operand::LiteralFloat32(value)]);
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpSpecConstant instruction with the given 32-bit integer `value`.
    /// or the module if no basic block is under construction.
    pub fn spec_constant_u32(&mut self, result_type: spirv::Word, value: u32) -> spirv::Word {
        let id = self.id();
        let inst = mr::Instruction::new(spirv::Op::SpecConstant,
                                        Some(result_type),
                                        Some(id),
                                        vec![mr::Operand::LiteralInt32(value)]);
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpVariable instruction to either the current basic block
    /// or the module if no basic block is under construction.
    pub fn variable(&mut self,
                    result_type: spirv::Word,
                    result_id: Option<spirv::Word>,
                    storage_class: spirv::StorageClass,
                    initializer: Option<spirv::Word>)
                    -> spirv::Word {
        let id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        let mut operands = vec![mr::Operand::StorageClass(storage_class)];
        if let Some(val) = initializer {
            operands.push(mr::Operand::IdRef(val));
        }
        let inst = mr::Instruction::new(spirv::Op::Variable, Some(result_type), Some(id), operands);

        match self.basic_block {
            Some(ref mut bb) => bb.instructions.push(inst),
            None => self.module.types_global_values.push(inst),
        }
        id
    }

    /// Appends an OpUndef instruction to either the current basic block
    /// or the module if no basic block is under construction.
    pub fn undef(&mut self,
                 result_type: spirv::Word,
                 result_id: Option<spirv::Word>)
                 -> spirv::Word {
        let id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        let inst = mr::Instruction::new(spirv::Op::Undef, Some(result_type), Some(id), vec![]);

        match self.basic_block {
            Some(ref mut bb) => bb.instructions.push(inst),
            None => self.module.types_global_values.push(inst),
        }
        id
    }
}

include!("build_norm_insts.rs");

#[cfg(test)]
mod tests {
    use mr;
    use spirv;

    use std::f32;
    use super::Builder;

    use binary::Disassemble;

    fn has_only_one_global_inst(module: &mr::Module) -> bool {
        if !module.functions.is_empty() {
            return false;
        }
        (module.capabilities.len() + module.extensions.len() + module.ext_inst_imports.len() +
         module.entry_points.len() + module.types_global_values.len() +
         module.execution_modes.len() +
         module.debugs.len() + module.annotations.len()) +
        (if module.memory_model.is_some() {
            1
        } else {
            0
        }) == 1
    }

    #[test]
    fn test_memory_model() {
        let mut b = Builder::new();
        b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::Simple);
        let m = b.module();
        assert!(m.memory_model.is_some());
        let inst = m.memory_model.as_ref().unwrap();
        assert!(has_only_one_global_inst(&m));
        assert_eq!("MemoryModel", inst.class.opname);
        assert_eq!(2, inst.operands.len());
        assert_eq!(mr::Operand::from(spirv::AddressingModel::Logical),
                   inst.operands[0]);
        assert_eq!(mr::Operand::from(spirv::MemoryModel::Simple),
                   inst.operands[1]);
    }

    #[test]
    fn test_decoration_no_additional_params() {
        let mut b = Builder::new();
        b.member_decorate(1, 0, spirv::Decoration::RelaxedPrecision, vec![]);
        let m = b.module();
        assert!(has_only_one_global_inst(&m));
        let inst = m.annotations.last().unwrap();
        assert_eq!("MemberDecorate", inst.class.opname);
        assert_eq!(3, inst.operands.len());
        assert_eq!(mr::Operand::IdRef(1), inst.operands[0]);
        assert_eq!(mr::Operand::from(0u32), inst.operands[1]);
        assert_eq!(mr::Operand::from(spirv::Decoration::RelaxedPrecision),
                   inst.operands[2]);
    }

    #[test]
    fn test_decoration_with_additional_params() {
        let mut b = Builder::new();
        b.decorate(1,
                   spirv::Decoration::LinkageAttributes,
                   vec![mr::Operand::from("name"), mr::Operand::from(spirv::LinkageType::Export)]);
        let m = b.module();
        assert!(has_only_one_global_inst(&m));
        let inst = m.annotations.last().unwrap();
        assert_eq!("Decorate", inst.class.opname);
        assert_eq!(4, inst.operands.len());
        assert_eq!(mr::Operand::IdRef(1), inst.operands[0]);
        assert_eq!(mr::Operand::from(spirv::Decoration::LinkageAttributes),
                   inst.operands[1]);
        assert_eq!(mr::Operand::from("name"), inst.operands[2]);
        assert_eq!(mr::Operand::from(spirv::LinkageType::Export),
                   inst.operands[3]);
    }

    #[test]
    fn test_constant_f32() {
        let mut b = Builder::new();
        let float = b.type_float(32);
        // Normal numbers
        b.constant_f32(float, 3.14);
        b.constant_f32(float, 2e-10);
        // Zero
        b.constant_f32(float, 0.);
        // Inf
        b.constant_f32(float, f32::NEG_INFINITY);
        // Subnormal numbers
        b.constant_f32(float, -1.0e-40_f32);
        // Nan
        b.constant_f32(float, f32::NAN);
        let m = b.module();
        assert_eq!(7, m.types_global_values.len());

        let inst = &m.types_global_values[1];
        assert_eq!(spirv::Op::Constant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(2), inst.result_id);
        assert_eq!(mr::Operand::from(3.14f32), inst.operands[0]);

        let inst = &m.types_global_values[2];
        assert_eq!(spirv::Op::Constant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(3), inst.result_id);
        assert_eq!(mr::Operand::from(2e-10_f32), inst.operands[0]);

        let inst = &m.types_global_values[3];
        assert_eq!(spirv::Op::Constant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(4), inst.result_id);
        assert_eq!(mr::Operand::from(0.0f32), inst.operands[0]);

        let inst = &m.types_global_values[4];
        assert_eq!(spirv::Op::Constant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(5), inst.result_id);
        assert_eq!(mr::Operand::from(f32::NEG_INFINITY), inst.operands[0]);

        let inst = &m.types_global_values[5];
        assert_eq!(spirv::Op::Constant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(6), inst.result_id);
        assert_eq!(mr::Operand::from(-1.0e-40_f32), inst.operands[0]);

        let inst = &m.types_global_values[6];
        assert_eq!(spirv::Op::Constant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(7), inst.result_id);
        // NaN != NaN
        match inst.operands[0] {
            mr::Operand::LiteralFloat32(f) => assert!(f.is_nan()),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_spec_constant_f32() {
        let mut b = Builder::new();
        let float = b.type_float(32);
        // Normal numbers
        b.spec_constant_f32(float, 10.);
        // Zero
        b.spec_constant_f32(float, -0.);
        // Inf
        b.spec_constant_f32(float, f32::INFINITY);
        // Subnormal numbers
        b.spec_constant_f32(float, 1.0e-40_f32);
        // Nan
        b.spec_constant_f32(float, f32::NAN);
        let m = b.module();
        assert_eq!(6, m.types_global_values.len());

        let inst = &m.types_global_values[1];
        assert_eq!(spirv::Op::SpecConstant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(2), inst.result_id);
        assert_eq!(mr::Operand::from(10.0f32), inst.operands[0]);

        let inst = &m.types_global_values[2];
        assert_eq!(spirv::Op::SpecConstant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(3), inst.result_id);
        assert_eq!(mr::Operand::from(-0.0f32), inst.operands[0]);

        let inst = &m.types_global_values[3];
        assert_eq!(spirv::Op::SpecConstant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(4), inst.result_id);
        assert_eq!(mr::Operand::from(f32::INFINITY), inst.operands[0]);

        let inst = &m.types_global_values[4];
        assert_eq!(spirv::Op::SpecConstant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(5), inst.result_id);
        assert_eq!(mr::Operand::from(1.0e-40_f32), inst.operands[0]);

        let inst = &m.types_global_values[5];
        assert_eq!(spirv::Op::SpecConstant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(6), inst.result_id);
        // NaN != NaN
        match inst.operands[0] {
            mr::Operand::LiteralFloat32(f) => assert!(f.is_nan()),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_forward_ref_pointer_type() {
        let mut b = Builder::new();
        let float = b.type_float(32); // 1
        // Let builder generate
        let p1 = b.type_pointer(None, spirv::StorageClass::Input, float); // 2
        // We supply
        let pointee = b.id(); // 3
        b.type_forward_pointer(pointee, spirv::StorageClass::Output);
        let p2 = b.type_pointer(Some(pointee), spirv::StorageClass::Output, float);
        let m = b.module();
        assert_eq!(1, float);
        assert_eq!(2, p1);
        assert_eq!(pointee, p2); // Return the same id
        assert_eq!(4, m.types_global_values.len());

        let inst = &m.types_global_values[0];
        assert_eq!(spirv::Op::TypeFloat, inst.class.opcode);
        assert_eq!(None, inst.result_type);
        assert_eq!(Some(1), inst.result_id);
        assert_eq!(vec![mr::Operand::LiteralInt32(32)], inst.operands);

        let inst = &m.types_global_values[1];
        assert_eq!(spirv::Op::TypePointer, inst.class.opcode);
        assert_eq!(None, inst.result_type);
        assert_eq!(Some(2), inst.result_id);
        assert_eq!(vec![mr::Operand::from(spirv::StorageClass::Input), mr::Operand::IdRef(1)],
                   inst.operands);

        let inst = &m.types_global_values[2];
        assert_eq!(spirv::Op::TypeForwardPointer, inst.class.opcode);
        assert_eq!(None, inst.result_type);
        assert_eq!(None, inst.result_id);
        assert_eq!(vec![mr::Operand::IdRef(3), mr::Operand::from(spirv::StorageClass::Output)],
                   inst.operands);

        let inst = &m.types_global_values[3];
        assert_eq!(spirv::Op::TypePointer, inst.class.opcode);
        assert_eq!(None, inst.result_type);
        assert_eq!(Some(3), inst.result_id);
        assert_eq!(vec![mr::Operand::from(spirv::StorageClass::Output), mr::Operand::IdRef(1)],
                   inst.operands);
    }

    #[test]
    fn test_forward_ref_phi() {
        let mut b = Builder::new();

        let float = b.type_float(32);
        assert_eq!(1, float);
        let f32ff32 = b.type_function(float, vec![float]);
        assert_eq!(2, f32ff32);
        let c0 = b.constant_f32(float, 0.0f32);
        assert_eq!(3, c0);

        let fid = b.begin_function(float, None, spirv::FUNCTION_CONTROL_NONE, f32ff32).unwrap();
        assert_eq!(4, fid);

        let epid = b.begin_basic_block(None).unwrap(); // Entry block id
        assert_eq!(5, epid);
        let target1 = b.id();
        assert_eq!(6, target1);
        assert!(b.branch(target1).is_ok());

        let pbid = b.begin_basic_block(Some(target1)).unwrap(); // Phi block id
        assert_eq!(target1, pbid);
        let target2 = b.id();
        assert_eq!(7, target2);
        let fr_add = b.id();
        assert_eq!(8, fr_add);
        // OpPhi can forward reference ids for both labels and results
        let phi = b.phi(float,
                        None,
                        // From above, from this, from below
                        vec![(c0, epid), (fr_add, pbid), (c0, target2)])
                   .unwrap();
        assert_eq!(9, phi);
        let res_add = b.fadd(float, Some(fr_add), c0, c0).unwrap();
        assert_eq!(res_add, fr_add);
        assert!(b.branch(target2).is_ok());

        let exid = b.begin_basic_block(Some(target2)).unwrap(); // Exit block id
        assert_eq!(exid, target2);
        assert!(b.ret_value(c0).is_ok());

        assert!(b.end_function().is_ok());

        let m = b.module();
        assert_eq!(1, m.functions.len());
        assert_eq!(m.functions.first().unwrap().disassemble(),
                   "%4 = OpFunction  %1  None %2\n\
                    %5 = OpLabel\n\
                    OpBranch %6\n\
                    %6 = OpLabel\n\
                    %9 = OpPhi  %1  %3 %5 %8 %6 %3 %7\n\
                    %8 = OpFAdd  %1  %3 %3\n\
                    OpBranch %7\n\
                    %7 = OpLabel\n\
                    OpReturnValue %3\n\
                    OpFunctionEnd");
    }

    #[test]
    fn test_build_variables() {
        let mut b = Builder::new();

        let void = b.type_void();
        assert_eq!(1, void);
        let float = b.type_float(32);
        assert_eq!(2, float);
        let ifp = b.type_pointer(None, spirv::StorageClass::Input, float);
        assert_eq!(3, ifp);
        let ffp = b.type_pointer(None, spirv::StorageClass::Function, float);
        assert_eq!(4, ffp);
        let voidfvoid = b.type_function(void, vec![void]);
        assert_eq!(5, voidfvoid);

        // Global variable
        let v1 = b.variable(ifp, None, spirv::StorageClass::Input, None);
        assert_eq!(6, v1);

        let f = b.begin_function(void, None, spirv::FUNCTION_CONTROL_NONE, voidfvoid).unwrap();
        assert_eq!(7, f);
        let bb = b.begin_basic_block(None).unwrap();
        assert_eq!(8, bb);
        // Local variable
        let v2 = b.variable(ffp, None, spirv::StorageClass::Function, None);
        assert_eq!(9, v2);
        assert!(b.ret().is_ok());
        assert!(b.end_function().is_ok());

        // Global variable again
        let v3 = b.variable(ifp, None, spirv::StorageClass::Input, None);
        assert_eq!(10, v3);

        assert_eq!(b.module().disassemble(),
                   "; SPIR-V\n; Version: 1.1\n; Generator: rspirv\n; Bound: 11\n\
                    %1 = OpTypeVoid\n\
                    %2 = OpTypeFloat 32\n\
                    %3 = OpTypePointer Input %2\n\
                    %4 = OpTypePointer Function %2\n\
                    %5 = OpTypeFunction %1 %1\n\
                    %6 = OpVariable  %3  Input\n\
                    %10 = OpVariable  %3  Input\n\
                    %7 = OpFunction  %1  None %5\n\
                    %8 = OpLabel\n\
                    %9 = OpVariable  %4  Function\n\
                    OpReturn\n\
                    OpFunctionEnd");
    }

    #[test]
    fn test_build_undefs() {
        let mut b = Builder::new();

        let void = b.type_void();
        assert_eq!(1, void);
        let float = b.type_float(32);
        assert_eq!(2, float);
        let voidfvoid = b.type_function(void, vec![void]);
        assert_eq!(3, voidfvoid);

        // Global undef
        let v1 = b.undef(float, None);
        assert_eq!(4, v1);

        let f = b.begin_function(void, None, spirv::FUNCTION_CONTROL_NONE, voidfvoid).unwrap();
        assert_eq!(5, f);
        let bb = b.begin_basic_block(None).unwrap();
        assert_eq!(6, bb);
        // Local undef
        let v2 = b.undef(float, None);
        assert_eq!(7, v2);
        assert!(b.ret().is_ok());
        assert!(b.end_function().is_ok());

        // Global undef again
        let v3 = b.undef(float, None);
        assert_eq!(8, v3);

        assert_eq!(b.module().disassemble(),
                   "; SPIR-V\n; Version: 1.1\n; Generator: rspirv\n; Bound: 9\n\
                    %1 = OpTypeVoid\n\
                    %2 = OpTypeFloat 32\n\
                    %3 = OpTypeFunction %1 %1\n\
                    %4 = OpUndef  %2 \n\
                    %8 = OpUndef  %2 \n\
                    %5 = OpFunction  %1  None %3\n\
                    %6 = OpLabel\n\
                    %7 = OpUndef  %2 \n\
                    OpReturn\n\
                    OpFunctionEnd");
    }
}
