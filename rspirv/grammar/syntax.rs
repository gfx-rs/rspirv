use crate::spirv;

/// Grammar for a SPIR-V instruction.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Instruction<'a> {
    /// Opname.
    pub opname: &'a str,
    /// Opcode.
    pub opcode: spirv::Op,
    /// Capabilities required for this instruction.
    pub capabilities: &'a [spirv::Capability],
    /// Extensions required for this instruction.
    pub extensions: &'a [&'a str],
    /// Logical operands for this instruction.
    ///
    /// This includes result type id and result id.
    pub operands: &'a [LogicalOperand],
}

/// Grammar for an extended instruction.
pub struct ExtendedInstruction<'a> {
    /// OpName.
    pub opname: &'a str,
    /// Opcode.
    pub opcode: spirv::Word,
    /// Capabilities required for this instruction.
    pub capabilities: &'a [spirv::Capability],
    /// Extensions required for this instruction.
    pub extensions: &'a [&'a str],
    /// Logical operands for this instruction.
    pub operands: &'a [LogicalOperand],
}

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
    ($opname:ident, $opcode: expr, [$( $cap:ident ),*], [$( $ext:expr ),*],
     [$( ($kind:ident, $quant:ident) ),*]) => {
        ExtendedInstruction {
            opname: stringify!($opname),
            opcode: $opcode,
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
/// This table is staic data stored in the library.
pub struct CoreInstructionTable;

impl CoreInstructionTable {
    /// Looks up the given `opcode` in the instruction table and returns
    /// a reference to the instruction grammar entry if found.
    pub fn lookup_opcode(opcode: u16) -> Option<&'static Instruction<'static>> {
        INSTRUCTION_TABLE
            .iter()
            .find(|inst| (inst.opcode as u16) == opcode)
    }

    /// Returns a reference to the instruction grammar entry with the given
    /// `opcode`.
    pub fn get(opcode: spirv::Op) -> &'static Instruction<'static> {
        INSTRUCTION_TABLE
            .iter()
            .find(|inst| (inst.opcode == opcode))
            .expect("internal error")
    }

    pub fn iter() -> impl Iterator<Item = &'static Instruction<'static>> {
        INSTRUCTION_TABLE.iter()
    }
}

include!("autogen_table.rs");

/// The table for all `GLSLstd450` extended instructions.
///
/// This table is staic data stored in the library.
pub struct GlslStd450InstructionTable;

impl GlslStd450InstructionTable {
    /// Looks up the given `opcode` in the instruction table and returns
    /// a reference to the instruction grammar entry if found.
    pub fn lookup_opcode(opcode: u32) -> Option<&'static ExtendedInstruction<'static>> {
        GLSL_STD_450_INSTRUCTION_TABLE
            .iter()
            .find(|inst| inst.opcode == opcode)
    }

    /// Returns a reference to the instruction grammar entry with the given
    /// `opcode`.
    pub fn get(opcode: spirv::GLOp) -> &'static ExtendedInstruction<'static> {
        GLSL_STD_450_INSTRUCTION_TABLE
            .iter()
            .find(|inst| (inst.opcode == opcode as spirv::Word))
            .expect("internal error")
    }

    pub fn iter() -> impl Iterator<Item = &'static ExtendedInstruction<'static>> {
        GLSL_STD_450_INSTRUCTION_TABLE.iter()
    }
}

include!("autogen_glsl_std_450.rs");

/// The table for all `OpenCLstd100` extended instructions.
///
/// This table is staic data stored in the library.
#[allow(clippy::upper_case_acronyms)]
pub struct OpenCLStd100InstructionTable;

impl OpenCLStd100InstructionTable {
    /// Looks up the given `opcode` in the instruction table and returns
    /// a reference to the instruction grammar entry if found.
    pub fn lookup_opcode(opcode: u32) -> Option<&'static ExtendedInstruction<'static>> {
        OPENCL_STD_100_INSTRUCTION_TABLE
            .iter()
            .find(|inst| inst.opcode == opcode)
    }

    /// Returns a reference to the instruction grammar entry with the given
    /// `opcode`.
    pub fn get(opcode: spirv::CLOp) -> &'static ExtendedInstruction<'static> {
        OPENCL_STD_100_INSTRUCTION_TABLE
            .iter()
            .find(|inst| (inst.opcode == opcode as spirv::Word))
            .expect("internal error")
    }

    pub fn iter() -> impl Iterator<Item = &'static ExtendedInstruction<'static>> {
        OPENCL_STD_100_INSTRUCTION_TABLE.iter()
    }
}

include!("autogen_opencl_std_100.rs");
