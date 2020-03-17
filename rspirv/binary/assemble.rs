use crate::dr;
use std::convert::TryInto;

/// Trait for assembling functionalities.
pub trait Assemble {
    /// Assembles the current object and returns the binary code.
    fn assemble(&self) -> Vec<u32>;
}

impl Assemble for dr::ModuleHeader {
    fn assemble(&self) -> Vec<u32> {
        vec![self.magic_number, self.version, self.generator, self.bound, self.reserved_word]
    }
}

fn assemble_str(s: &str) -> Vec<u32> {
    let chunks = s.as_bytes().chunks_exact(4);
    let remainder = chunks.remainder();
    let mut last = [0; 4];
    last[..remainder.len()].copy_from_slice(remainder);
    let mut words = Vec::with_capacity(chunks.len() + 1);
    words.extend(chunks.map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap())));
    words.push(u32::from_le_bytes(last));
    words
}

impl Assemble for dr::Operand {
    fn assemble(&self) -> Vec<u32> {
        match *self {
            dr::Operand::ImageOperands(v) => vec![v.bits()],
            dr::Operand::FPFastMathMode(v) => vec![v.bits()],
            dr::Operand::SelectionControl(v) => vec![v.bits()],
            dr::Operand::LoopControl(v) => vec![v.bits()],
            dr::Operand::FunctionControl(v) => vec![v.bits()],
            dr::Operand::MemorySemantics(v) => vec![v.bits()],
            dr::Operand::MemoryAccess(v) => vec![v.bits()],
            dr::Operand::KernelProfilingInfo(v) => vec![v.bits()],
            dr::Operand::SourceLanguage(v) => vec![v as u32],
            dr::Operand::ExecutionModel(v) => vec![v as u32],
            dr::Operand::AddressingModel(v) => vec![v as u32],
            dr::Operand::MemoryModel(v) => vec![v as u32],
            dr::Operand::ExecutionMode(v) => vec![v as u32],
            dr::Operand::StorageClass(v) => vec![v as u32],
            dr::Operand::Dim(v) => vec![v as u32],
            dr::Operand::SamplerAddressingMode(v) => vec![v as u32],
            dr::Operand::SamplerFilterMode(v) => vec![v as u32],
            dr::Operand::ImageFormat(v) => vec![v as u32],
            dr::Operand::ImageChannelOrder(v) => vec![v as u32],
            dr::Operand::ImageChannelDataType(v) => vec![v as u32],
            dr::Operand::FPRoundingMode(v) => vec![v as u32],
            dr::Operand::LinkageType(v) => vec![v as u32],
            dr::Operand::AccessQualifier(v) => vec![v as u32],
            dr::Operand::FunctionParameterAttribute(v) => vec![v as u32],
            dr::Operand::Decoration(v) => vec![v as u32],
            dr::Operand::BuiltIn(v) => vec![v as u32],
            dr::Operand::Scope(v) => vec![v as u32],
            dr::Operand::GroupOperation(v) => vec![v as u32],
            dr::Operand::KernelEnqueueFlags(v) => vec![v as u32],
            dr::Operand::Capability(v) => vec![v as u32],
            dr::Operand::IdMemorySemantics(v) |
            dr::Operand::IdScope(v) |
            dr::Operand::IdRef(v) |
            dr::Operand::LiteralInt32(v) |
            dr::Operand::LiteralExtInstInteger(v) => vec![v],
            dr::Operand::LiteralInt64(v) => vec![v as u32, (v >> 32) as u32],
            dr::Operand::LiteralFloat32(v) => vec![v.to_bits()],
            dr::Operand::LiteralFloat64(v) => vec![v.to_bits() as u32, (v.to_bits() >> 32) as u32],
            dr::Operand::LiteralSpecConstantOpInteger(v) => vec![v as u32],
            dr::Operand::LiteralString(ref v) => assemble_str(v),
        }
    }
}

impl Assemble for dr::Instruction {
    fn assemble(&self) -> Vec<u32> {
        let mut code = vec![self.class.opcode as u32];
        if let Some(r) = self.result_type {
            code.push(r);
        }
        if let Some(r) = self.result_id {
            code.push(r);
        }
        for operand in &self.operands {
            code.append(&mut operand.assemble());
        }
        code[0] |= (code.len() as u32) << 16;
        code
    }
}

impl Assemble for dr::Block {
    fn assemble(&self) -> Vec<u32> {
        let mut code = vec![];
        if let Some(ref l) = self.label {
            code.append(&mut l.assemble());
        }
        for inst in &self.instructions {
            code.append(&mut inst.assemble());
        }
        code
    }
}

impl Assemble for dr::Function {
    fn assemble(&self) -> Vec<u32> {
        let mut code = vec![];
        if let Some(ref d) = self.def {
            code.append(&mut d.assemble());
        }
        for param in &self.parameters {
            code.append(&mut param.assemble());
        }
        for bb in &self.blocks {
            code.append(&mut bb.assemble());
        }
        if let Some(ref e) = self.end {
            code.append(&mut e.assemble());
        }
        code
    }
}

impl Assemble for dr::Module {
    fn assemble(&self) -> Vec<u32> {
        let mut code = match self.header {
            Some(ref h) => h.assemble(),
            None => vec![],
        };
        for inst in self.global_inst_iter() {
            code.append(&mut inst.assemble());
        }
        for f in &self.functions {
            code.append(&mut f.assemble());
        }
        code
    }
}

#[cfg(test)]
mod tests {
    use crate::dr;
    use crate::spirv;

    use crate::binary::Assemble;
    use super::assemble_str;

    #[test]
    fn test_assemble_str() {
        assert_eq!(vec![0u32], assemble_str(""));
        assert_eq!(vec![u32::from_le_bytes(*b"h\0\0\0")], assemble_str("h"));
        assert_eq!(vec![u32::from_le_bytes(*b"hell"), 0u32], assemble_str("hell"));
        assert_eq!(vec![u32::from_le_bytes(*b"hell"), u32::from_le_bytes(*b"o\0\0\0")],
                   assemble_str("hello"));
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
        let v = spirv::FunctionControl::DONT_INLINE | spirv::FunctionControl::PURE |
                spirv::FunctionControl::CONST;
        assert_eq!(vec![v.bits()], dr::Operand::FunctionControl(v).assemble());
    }

    #[test]
    fn test_assemble_operand_enum() {
        assert_eq!(vec![spirv::BuiltIn::Position as u32],
                   dr::Operand::BuiltIn(spirv::BuiltIn::Position).assemble());
        assert_eq!(vec![spirv::BuiltIn::PointSize as u32],
                   dr::Operand::BuiltIn(spirv::BuiltIn::PointSize).assemble());
        assert_eq!(vec![spirv::BuiltIn::InstanceId as u32],
                   dr::Operand::BuiltIn(spirv::BuiltIn::InstanceId).assemble());
    }

    fn wc_op(wc: u32, op: spirv::Op) -> u32 {
        (wc << 16) | op as u32
    }

    // No operands
    #[test]
    fn test_assemble_inst_nop() {
        assert_eq!(vec![wc_op(1, spirv::Op::Nop)],
                   dr::Instruction::new(spirv::Op::Nop, None, None, vec![]).assemble());
    }

    // No result type and result id
    #[test]
    fn test_assemble_inst_memory_model() {
        let operands = vec![dr::Operand::AddressingModel(spirv::AddressingModel::Physical32),
                            dr::Operand::MemoryModel(spirv::MemoryModel::OpenCL)];
        assert_eq!(vec![wc_op(3, spirv::Op::MemoryModel),
                        spirv::AddressingModel::Physical32 as u32,
                        spirv::MemoryModel::OpenCL as u32],
                   dr::Instruction::new(spirv::Op::MemoryModel, None, None, operands).assemble());
    }

    // No result type, having result id
    #[test]
    fn test_assemble_inst_type_int() {
        let operands = vec![dr::Operand::LiteralInt32(32), dr::Operand::LiteralInt32(1)];
        assert_eq!(vec![wc_op(4, spirv::Op::TypeInt), 42, 32, 1],
                   dr::Instruction::new(spirv::Op::TypeInt, None, Some(42), operands).assemble());
    }

    // Having result type and id
    #[test]
    fn test_assemble_inst_iadd() {
        let operands = vec![dr::Operand::IdRef(0xef), dr::Operand::IdRef(0x78)];
        assert_eq!(vec![wc_op(5, spirv::Op::IAdd), 0xab, 0xcd, 0xef, 0x78],
                   dr::Instruction::new(spirv::Op::IAdd, Some(0xab), Some(0xcd), operands)
                       .assemble());
    }

    #[test]
    fn test_assemble_function_void() {
        let mut b = dr::Builder::new();
        b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::Simple);
        let void = b.type_void();
        let voidfvoid = b.type_function(void, vec![void]);
        b.begin_function(void, None, spirv::FunctionControl::CONST, voidfvoid).unwrap();
        b.begin_block(None).unwrap();
        b.ret().unwrap();
        b.end_function().unwrap();

        assert_eq!(vec![spirv::MAGIC_NUMBER,
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
                        wc_op(1, spirv::Op::FunctionEnd)],
                   b.module().assemble());
    }

    #[test]
    fn test_assemble_function_parameters() {
        let mut b = dr::Builder::new();
        b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::Simple);
        let float = b.type_float(32);
        let ptr = b.type_pointer(None, spirv::StorageClass::Function, float);
        let fff = b.type_function(float, vec![float, float]);
        b.begin_function(float, None, spirv::FunctionControl::CONST, fff).unwrap();
        let param1 = b.function_parameter(ptr).unwrap();
        let param2 = b.function_parameter(ptr).unwrap();
        b.begin_block(None).unwrap();
        let v1 = b.load(float, None, param1, None, vec![]).unwrap();
        let v2 = b.load(float, None, param2, None, vec![]).unwrap();
        let v = b.f_add(float, None, v1, v2).unwrap();
        b.ret_value(v).unwrap();
        b.end_function().unwrap();

        assert_eq!(vec![// Header
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
                        1, // result id
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
                        1, // result type id
                        10, // result id
                        8, // operand
                        9, // operand
                        wc_op(2, spirv::Op::ReturnValue),
                        10,
                        wc_op(1, spirv::Op::FunctionEnd)],
                   b.module().assemble());
    }
}
