use crate::dr;
use std::convert::TryInto;

/// Trait for assembling functionalities.
pub trait Assemble {
    /// Assembles the current object into the `result` vector, reducing the need for lots of allocations
    fn assemble_into(&self, result: &mut Vec<u32>);

    /// Assembles the current object and returns the binary code.
    /// Helper method to remain backwards compatible, calls `assemble_into`
    fn assemble(&self) -> Vec<u32> {
        let mut v = vec![];
        self.assemble_into(&mut v);
        v
    }
}

impl Assemble for dr::ModuleHeader {
    fn assemble_into(&self, result: &mut Vec<u32>) {
        result.extend([
            self.magic_number,
            self.version,
            self.generator,
            self.bound,
            self.reserved_word,
        ])
    }
}

fn assemble_str(s: &str, result: &mut Vec<u32>) {
    let chunks = s.as_bytes().chunks_exact(4);
    let remainder = chunks.remainder();
    let mut last = [0; 4];
    last[..remainder.len()].copy_from_slice(remainder);
    result.extend(chunks.map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap())));
    result.push(u32::from_le_bytes(last));
}

impl Assemble for dr::Operand {
    fn assemble_into(&self, result: &mut Vec<u32>) {
        match *self {
            Self::ImageOperands(v) => result.push(v.bits()),
            Self::FPFastMathMode(v) => result.push(v.bits()),
            Self::SelectionControl(v) => result.push(v.bits()),
            Self::LoopControl(v) => result.push(v.bits()),
            Self::FunctionControl(v) => result.push(v.bits()),
            Self::MemorySemantics(v) => result.push(v.bits()),
            Self::MemoryAccess(v) => result.push(v.bits()),
            Self::KernelProfilingInfo(v) => result.push(v.bits()),
            Self::SourceLanguage(v) => result.push(v as u32),
            Self::ExecutionModel(v) => result.push(v as u32),
            Self::AddressingModel(v) => result.push(v as u32),
            Self::MemoryModel(v) => result.push(v as u32),
            Self::ExecutionMode(v) => result.push(v as u32),
            Self::StorageClass(v) => result.push(v as u32),
            Self::Dim(v) => result.push(v as u32),
            Self::SamplerAddressingMode(v) => result.push(v as u32),
            Self::SamplerFilterMode(v) => result.push(v as u32),
            Self::ImageFormat(v) => result.push(v as u32),
            Self::ImageChannelOrder(v) => result.push(v as u32),
            Self::ImageChannelDataType(v) => result.push(v as u32),
            Self::FPRoundingMode(v) => result.push(v as u32),
            Self::LinkageType(v) => result.push(v as u32),
            Self::AccessQualifier(v) => result.push(v as u32),
            Self::FunctionParameterAttribute(v) => result.push(v as u32),
            Self::Decoration(v) => result.push(v as u32),
            Self::BuiltIn(v) => result.push(v as u32),
            Self::Scope(v) => result.push(v as u32),
            Self::GroupOperation(v) => result.push(v as u32),
            Self::KernelEnqueueFlags(v) => result.push(v as u32),
            Self::Capability(v) => result.push(v as u32),
            Self::IdMemorySemantics(v)
            | Self::IdScope(v)
            | Self::IdRef(v)
            | Self::LiteralInt32(v)
            | Self::LiteralExtInstInteger(v) => result.push(v),
            Self::LiteralInt64(v) => result.extend([v as u32, (v >> 32) as u32]),
            Self::LiteralFloat32(v) => result.push(v.to_bits()),
            Self::LiteralFloat64(v) => {
                result.extend([v.to_bits() as u32, (v.to_bits() >> 32) as u32])
            }
            Self::LiteralSpecConstantOpInteger(v) => result.push(v as u32),
            Self::LiteralString(ref v) => assemble_str(v, result),
            Self::RayFlags(ref v) => result.push(v.bits()),
            Self::RayQueryIntersection(v) => result.push(v as u32),
            Self::RayQueryCommittedIntersectionType(v) => result.push(v as u32),
            Self::RayQueryCandidateIntersectionType(v) => result.push(v as u32),
            Self::FragmentShadingRate(v) => result.push(v.bits()),
            Self::FPDenormMode(v) => result.push(v as u32),
            Self::QuantizationModes(v) => result.push(v as u32),
            Self::FPOperationMode(v) => result.push(v as u32),
            Self::OverflowModes(v) => result.push(v as u32),
            Self::PackedVectorFormat(v) => result.push(v as u32),
        }
    }
}

impl Assemble for dr::Instruction {
    fn assemble_into(&self, result: &mut Vec<u32>) {
        let start = result.len();
        result.push(self.class.opcode as u32);
        if let Some(r) = self.result_type {
            result.push(r);
        }
        if let Some(r) = self.result_id {
            result.push(r);
        }
        for operand in &self.operands {
            operand.assemble_into(result);
        }
        let end = result.len() - start;
        result[start] |= (end as u32) << 16;
    }
}

impl Assemble for dr::Block {
    fn assemble_into(&self, result: &mut Vec<u32>) {
        if let Some(ref l) = self.label {
            l.assemble_into(result);
        }
        for inst in &self.instructions {
            inst.assemble_into(result);
        }
    }
}

impl Assemble for dr::Function {
    fn assemble_into(&self, result: &mut Vec<u32>) {
        if let Some(ref d) = self.def {
            d.assemble_into(result);
        }
        for param in &self.parameters {
            param.assemble_into(result);
        }
        for bb in &self.blocks {
            bb.assemble_into(result);
        }
        if let Some(ref e) = self.end {
            e.assemble_into(result);
        }
    }
}

impl Assemble for dr::Module {
    fn assemble_into(&self, result: &mut Vec<u32>) {
        if let Some(ref h) = self.header {
            h.assemble_into(result);
        }

        for inst in self.global_inst_iter() {
            inst.assemble_into(result);
        }

        for f in &self.functions {
            f.assemble_into(result);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dr;
    use crate::spirv;

    use super::assemble_str;
    use crate::binary::Assemble;

    #[test]
    fn test_assemble_str() {
        fn assemble_str_helper(s: &str) -> Vec<u32> {
            let mut v = vec![];
            assemble_str(s, &mut v);
            v
        }
        assert_eq!(vec![0u32], assemble_str_helper(""));
        assert_eq!(
            vec![u32::from_le_bytes(*b"h\0\0\0")],
            assemble_str_helper("h")
        );
        assert_eq!(
            vec![u32::from_le_bytes(*b"hell"), 0u32],
            assemble_str_helper("hell")
        );
        assert_eq!(
            vec![
                u32::from_le_bytes(*b"hell"),
                u32::from_le_bytes(*b"o\0\0\0")
            ],
            assemble_str_helper("hello")
        );
    }

    #[test]
    fn test_assemble_operand_bitmask() {
        let v = spirv::FunctionControl::DONT_INLINE;
        assert_eq!(vec![v.bits()], dr::Operand::FunctionControl(v).assemble());
        let v = spirv::FunctionControl::PURE;
        assert_eq!(vec![v.bits()], dr::Operand::FunctionControl(v).assemble());
        let v = spirv::FunctionControl::CONST;
        assert_eq!(vec![v.bits()], dr::Operand::FunctionControl(v).assemble());
        let v = spirv::FunctionControl::DONT_INLINE | spirv::FunctionControl::CONST;
        assert_eq!(vec![v.bits()], dr::Operand::FunctionControl(v).assemble());
        let v = spirv::FunctionControl::DONT_INLINE
            | spirv::FunctionControl::PURE
            | spirv::FunctionControl::CONST;
        assert_eq!(vec![v.bits()], dr::Operand::FunctionControl(v).assemble());
    }

    #[test]
    fn test_assemble_operand_enum() {
        assert_eq!(
            vec![spirv::BuiltIn::Position as u32],
            dr::Operand::BuiltIn(spirv::BuiltIn::Position).assemble()
        );
        assert_eq!(
            vec![spirv::BuiltIn::PointSize as u32],
            dr::Operand::BuiltIn(spirv::BuiltIn::PointSize).assemble()
        );
        assert_eq!(
            vec![spirv::BuiltIn::InstanceId as u32],
            dr::Operand::BuiltIn(spirv::BuiltIn::InstanceId).assemble()
        );
    }

    fn wc_op(wc: u32, op: spirv::Op) -> u32 {
        (wc << 16) | op as u32
    }

    // No operands
    #[test]
    fn test_assemble_inst_nop() {
        assert_eq!(
            vec![wc_op(1, spirv::Op::Nop)],
            dr::Instruction::new(spirv::Op::Nop, None, None, vec![]).assemble()
        );
    }

    // No result type and result id
    #[test]
    fn test_assemble_inst_memory_model() {
        let operands = vec![
            dr::Operand::AddressingModel(spirv::AddressingModel::Physical32),
            dr::Operand::MemoryModel(spirv::MemoryModel::OpenCL),
        ];
        assert_eq!(
            vec![
                wc_op(3, spirv::Op::MemoryModel),
                spirv::AddressingModel::Physical32 as u32,
                spirv::MemoryModel::OpenCL as u32
            ],
            dr::Instruction::new(spirv::Op::MemoryModel, None, None, operands).assemble()
        );
    }

    // No result type, having result id
    #[test]
    fn test_assemble_inst_type_int() {
        let operands = vec![dr::Operand::LiteralInt32(32), dr::Operand::LiteralInt32(1)];
        assert_eq!(
            vec![wc_op(4, spirv::Op::TypeInt), 42, 32, 1],
            dr::Instruction::new(spirv::Op::TypeInt, None, Some(42), operands).assemble()
        );
    }

    // Having result type and id
    #[test]
    fn test_assemble_inst_iadd() {
        let operands = vec![dr::Operand::IdRef(0xef), dr::Operand::IdRef(0x78)];
        assert_eq!(
            vec![wc_op(5, spirv::Op::IAdd), 0xab, 0xcd, 0xef, 0x78],
            dr::Instruction::new(spirv::Op::IAdd, Some(0xab), Some(0xcd), operands).assemble()
        );
    }

    #[test]
    fn test_assemble_function_void() {
        let mut b = dr::Builder::new();
        b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::Simple);
        let void = b.type_void();
        let voidfvoid = b.type_function(void, vec![void]);
        b.begin_function(void, None, spirv::FunctionControl::CONST, voidfvoid)
            .unwrap();
        b.begin_block(None).unwrap();
        b.ret().unwrap();
        b.end_function().unwrap();

        assert_eq!(
            vec![
                spirv::MAGIC_NUMBER,
                (u32::from(spirv::MAJOR_VERSION) << 16) | (u32::from(spirv::MINOR_VERSION) << 8),
                0x000f0000,
                5,
                0,
                wc_op(3, spirv::Op::MemoryModel),
                spirv::AddressingModel::Logical as u32,
                spirv::MemoryModel::Simple as u32,
                wc_op(2, spirv::Op::TypeVoid),
                1,
                wc_op(4, spirv::Op::TypeFunction),
                2,
                1,
                1,
                wc_op(5, spirv::Op::Function),
                1,
                3,
                spirv::FunctionControl::CONST.bits(),
                2,
                wc_op(2, spirv::Op::Label),
                4,
                wc_op(1, spirv::Op::Return),
                wc_op(1, spirv::Op::FunctionEnd)
            ],
            b.module().assemble()
        );
    }

    #[test]
    fn test_assemble_function_parameters() {
        let mut b = dr::Builder::new();
        b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::Simple);
        let float = b.type_float(32);
        let ptr = b.type_pointer(None, spirv::StorageClass::Function, float);
        let fff = b.type_function(float, vec![float, float]);
        b.begin_function(float, None, spirv::FunctionControl::CONST, fff)
            .unwrap();
        let param1 = b.function_parameter(ptr).unwrap();
        let param2 = b.function_parameter(ptr).unwrap();
        b.begin_block(None).unwrap();
        let v1 = b.load(float, None, param1, None, vec![]).unwrap();
        let v2 = b.load(float, None, param2, None, vec![]).unwrap();
        let v = b.f_add(float, None, v1, v2).unwrap();
        b.ret_value(v).unwrap();
        b.end_function().unwrap();

        assert_eq!(
            vec![
                // Header
                spirv::MAGIC_NUMBER,
                (u32::from(spirv::MAJOR_VERSION) << 16) | (u32::from(spirv::MINOR_VERSION) << 8),
                0x000f0000,
                11, // bound
                0,
                // Instructions
                wc_op(3, spirv::Op::MemoryModel),
                spirv::AddressingModel::Logical as u32,
                spirv::MemoryModel::Simple as u32,
                wc_op(3, spirv::Op::TypeFloat),
                1,  // result id
                32, // bitwidth
                wc_op(4, spirv::Op::TypePointer),
                2, // result id
                spirv::StorageClass::Function as u32,
                1, // float result id
                wc_op(5, spirv::Op::TypeFunction),
                3, // result id
                1, // result type
                1, // parameter type
                1, // parameter type
                wc_op(5, spirv::Op::Function),
                1, // result type id
                4, // result id
                spirv::FunctionControl::CONST.bits(),
                3, // function type id
                wc_op(3, spirv::Op::FunctionParameter),
                2, // result type id
                5, // result id
                wc_op(3, spirv::Op::FunctionParameter),
                2, // result type id
                6, // result id
                wc_op(2, spirv::Op::Label),
                7, // result id
                wc_op(4, spirv::Op::Load),
                1, // result type id
                8, // result id
                5, // parameter id
                wc_op(4, spirv::Op::Load),
                1, // result type id
                9, // result id
                6, // parameter id
                wc_op(5, spirv::Op::FAdd),
                1,  // result type id
                10, // result id
                8,  // operand
                9,  // operand
                wc_op(2, spirv::Op::ReturnValue),
                10,
                wc_op(1, spirv::Op::FunctionEnd)
            ],
            b.module().assemble()
        );
    }
}
