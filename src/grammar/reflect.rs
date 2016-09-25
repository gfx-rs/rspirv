use spirv;

pub fn is_location_debug(opcode: spirv::Op) -> bool {
    match opcode {
        spirv::Op::Line | spirv::Op::NoLine => true,
        _ => false,
    }
}

pub fn is_nonlocation_debug(opcode: spirv::Op) -> bool {
    match opcode {
        spirv::Op::SourceContinued |
        spirv::Op::Source |
        spirv::Op::SourceExtension |
        spirv::Op::Name |
        spirv::Op::MemberName |
        spirv::Op::String => true,
        _ => false,
    }
}

pub fn is_debug(opcode: spirv::Op) -> bool {
    is_location_debug(opcode) || is_nonlocation_debug(opcode)
}

pub fn is_annotation(opcode: spirv::Op) -> bool {
    match opcode {
        spirv::Op::Decorate |
        spirv::Op::MemberDecorate |
        spirv::Op::DecorationGroup |
        spirv::Op::GroupDecorate |
        spirv::Op::GroupMemberDecorate => true,
        _ => false,
    }
}


pub fn is_type(opcode: spirv::Op) -> bool {
    match opcode {
        spirv::Op::TypeVoid |
        spirv::Op::TypeBool |
        spirv::Op::TypeInt |
        spirv::Op::TypeFloat |
        spirv::Op::TypeVector |
        spirv::Op::TypeMatrix |
        spirv::Op::TypeImage |
        spirv::Op::TypeSampler |
        spirv::Op::TypeSampledImage |
        spirv::Op::TypeArray |
        spirv::Op::TypeRuntimeArray |
        spirv::Op::TypeStruct |
        spirv::Op::TypeOpaque |
        spirv::Op::TypePointer |
        spirv::Op::TypeFunction |
        spirv::Op::TypeEvent |
        spirv::Op::TypeDeviceEvent |
        spirv::Op::TypeReserveId |
        spirv::Op::TypeQueue |
        spirv::Op::TypePipe |
        spirv::Op::TypeForwardPointer => true,
        _ => false,
    }
}

pub fn is_constant(opcode: spirv::Op) -> bool {
    match opcode {

        spirv::Op::ConstantTrue |
        spirv::Op::ConstantFalse |
        spirv::Op::Constant |
        spirv::Op::ConstantComposite |
        spirv::Op::ConstantSampler |
        spirv::Op::ConstantNull |
        spirv::Op::SpecConstantTrue |
        spirv::Op::SpecConstantFalse |
        spirv::Op::SpecConstant |
        spirv::Op::SpecConstantComposite |
        spirv::Op::SpecConstantOp => true,
        _ => false,
    }
}

pub fn is_variable(opcode: spirv::Op) -> bool {
    opcode == spirv::Op::Variable
}
