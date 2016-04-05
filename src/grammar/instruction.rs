use spirv;

#[derive(Clone, Copy, Debug)]
pub enum OperandQuantifier {
    One,
    ZeroOrOne,
    ZeroOrMore,
}

#[derive(Debug)]
pub struct Instruction<'a> {
    opname: &'a str,
    opcode: spirv::Op,
    capabilities: &'a [spirv::Capability],
    operands: &'a [(OperandKind, OperandQuantifier)],
}

macro_rules! inst {
    ($op:ident, [$( $cap:ident ),*], [$( ($kind:ident, $quantifier:ident) ),*]) => {
        Instruction {
            opname: stringify!($op),
            opcode: spirv::Op::$op,
            capabilities: &[
                $( spirv::Capability::$cap ),*
            ],
            operands: &[
                $( (OperandKind::$kind, OperandQuantifier::$quantifier) ),*
            ],
        }
    }
}

include!("table.rs");

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
