use spirv;

#[derive(Debug)]
pub struct Instruction<'a> {
    pub opname: &'a str,
    pub opcode: spirv::Op,
    pub capabilities: &'a [spirv::Capability],
    pub operands: &'a [(OperandKind, OperandQuantifier)],
}

#[derive(Clone, Copy, Debug)]
pub enum OperandQuantifier {
    One,
    ZeroOrOne,
    ZeroOrMore,
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

pub struct InstructionTable;

impl InstructionTable {
    pub fn lookup_opcode(opcode: u16) -> Option<&'static Instruction<'static>> {
        INSTRUCTION_TABLE.iter().find(|&inst| (inst.opcode as u16) == opcode)
    }
}

include!("table.rs");
