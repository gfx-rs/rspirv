use std::marker::PhantomData;

use crate::spirv;

/// Grammar for a SPIR-V instruction.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BaseInstruction<'a, Op: Clone + Copy>
where
    Op: Into<spirv::Word>,
{
    /// Opname.
    pub opname: &'a str,
    /// Opcode.
    pub opcode: Op,
    /// Capabilities required for this instruction.
    pub capabilities: &'a [spirv::Capability],
    /// Extensions required for this instruction.
    pub extensions: &'a [&'a str],
    /// Logical operands for this instruction.
    ///
    /// This includes result type id and result id.
    pub operands: &'a [LogicalOperand],
}

pub type Instruction<'a> = BaseInstruction<'a, spirv::Op>;
pub type ExtendedInstruction<'a> = BaseInstruction<'a, ExtInstOp>;

/// Grammar for a SPIR-V logical operand.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LogicalOperand {
    /// The kind of this logical operand.
    pub kind: OperandKind,
    /// The repeat specification for this logical operand.
    pub quantifier: OperandQuantifier,
}

/// The repeat specification for a SPIR-V logical operand.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OperandQuantifier {
    /// This operand appears exactly one time.
    One,
    /// This operand can appear zero or one time.
    ZeroOrOne,
    /// This operand can appear zero or more times.
    ZeroOrMore,
}

/// Declares the grammar for an SPIR-V instruction.
macro_rules! inst {
    ($op:ident, [$( $cap:ident ),*], [$( $ext:expr ),*], [$( ($kind:ident, $quant:ident) ),*]) => {
        Instruction {
            opname: stringify!($op),
            opcode: spirv::Op::$op,
            capabilities: &[
                $( spirv::Capability::$cap ),*
            ],
            extensions: &[
                $( $ext ),*
            ],
            operands: &[
                $( LogicalOperand {
                    kind: OperandKind::$kind,
                    quantifier: OperandQuantifier::$quant }
                ),*
            ],
        }
    }
}

/// Declares the grammar for an extended instruction instruction.
macro_rules! ext_inst {
    ($variant:ident, $opconst:ident, $opname:ident, [$( $cap:ident ),*], [$( $ext:expr ),*],
     [$( ($kind:ident, $quant:ident) ),*]) => {
        ExtendedInstruction {
            opname: stringify!($opname),
            opcode: ExtInstOp::$variant(spirv::$opconst::$opname),
            capabilities: &[
                $( spirv::Capability::$cap ),*
            ],
            extensions: &[
                $( $ext ),*
            ],
            operands: &[
                $( LogicalOperand {
                    kind: OperandKind::$kind,
                    quantifier: OperandQuantifier::$quant }
                ),*
            ],
        }
    }
}

/// The table for all SPIR-V core instructions.
///
/// This table is static data stored in the library.
pub struct InstructionTable<Op: Into<spirv::Word> + Clone + Copy + Eq + 'static>(
    &'static [BaseInstruction<'static, Op>],
    PhantomData<Op>,
);

impl<Op: Into<spirv::Word> + Clone + Copy + Eq> InstructionTable<Op> {
    /// Looks up the given `opcode` in the instruction table and returns
    /// a reference to the instruction grammar entry if found.
    pub fn lookup_opcode(
        &self,
        opcode: spirv::Word,
    ) -> Option<&'static BaseInstruction<'static, Op>> {
        self.0.iter().find(|inst| inst.opcode.into() == opcode)
    }

    /// Returns a reference to the instruction grammar entry with the given
    /// `opcode`.
    pub fn get(&self, opcode: Op) -> &'static BaseInstruction<'static, Op> {
        self.0
            .iter()
            .find(|inst| inst.opcode == opcode)
            .expect("internal error")
    }

    pub fn iter(&self) -> impl Iterator<Item = &'static BaseInstruction<'static, Op>> {
        self.0.iter()
    }
}

include!("autogen_tables.rs");
