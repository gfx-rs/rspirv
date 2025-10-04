// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static NONSEMANTIC_DEBUGPRINTF_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[ext_inst!(
    DebugPrintF,
    DebugPrintFOp,
    DebugPrintf,
    [],
    [],
    [(IdRef, One), (IdRef, ZeroOrMore)]
)];
pub static NONSEMANTIC_DEBUGPRINTF_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(
        NONSEMANTIC_DEBUGPRINTF_INSTRUCTIONS,
        std::marker::PhantomData,
    );
