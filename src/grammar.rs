use spirv;

#[derive(Debug)]
enum OperandClass {
    OptionalId,
    OptionalLiteralString,
    LiteralNumber,
    LiteralString,
    Source,
    Capability,
}

#[derive(Debug)]
struct Operand<'a> {
    name: &'a str,
    code: spirv::Word,
    capabilities: &'a [spirv::Capability],
    operands: &'a [OperandClass],
}

#[derive(Debug)]
pub struct Instruction<'a> {
    opname: &'a str,
    opcode: spirv::Op,
    capabilities: &'a [spirv::Capability],
    has_result_type: bool,
    has_result_id: bool,
    operands: &'a [OperandClass],
}

macro_rules! inst {
    ($op:ident, [$( $cap:ident ),*], $rtype:expr, $rid:expr, [$( $operand:ident ),*]) => {
        Instruction {
            opname: stringify!($op),
            opcode: spirv::Op::$op,
            capabilities: &[
                $(spirv::Capability::$cap),*
            ],
            has_result_type: $rtype,
            has_result_id: $rid,
            operands: &[
                $(OperandClass::$operand),*
            ],
        }
    }
}

static INSTRUCTION_TABLE: &'static [Instruction<'static>] = &[
    inst!(Nop, [], false, false, []),
    inst!(Undef, [], true, true, []),
    inst!(SourceContinued, [], false, false, [LiteralString]),
    inst!(Source, [], false, false, [Source, LiteralNumber, OptionalId, OptionalLiteralString]),
    inst!(String, [], false, true, [LiteralString]),
    inst!(Capability, [], false, false, [Capability]),
];

pub struct InstructionTable;

impl InstructionTable {
    pub fn lookup_opcode(opcode: u16) -> Option<&'static Instruction<'static>> {
        for inst in INSTRUCTION_TABLE {
            if (inst.opcode as u16) == opcode {
                return Some(inst);
            }
        }
        None
    }
}
