// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static ARM_EXPERIMENTAL_ML_OPERATIONS_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[ext_inst!(
    ArmExperimentalMlOperations,
    ArmExperimentalMlOperationsOp,
    CALL,
    [],
    [],
    [(LiteralInteger, One), (IdRef, ZeroOrMore)]
)];
pub static ARM_EXPERIMENTAL_ML_OPERATIONS_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(
        ARM_EXPERIMENTAL_ML_OPERATIONS_INSTRUCTIONS,
        std::marker::PhantomData,
    );
