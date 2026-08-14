// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static NONSEMANTIC_DEBUGBREAK_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[ext_inst!(
    NonsemanticDebugbreak,
    NonsemanticDebugbreakOp,
    DebugBreak,
    [],
    [],
    []
)];
pub static NONSEMANTIC_DEBUGBREAK_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> = InstructionTable(
    NONSEMANTIC_DEBUGBREAK_INSTRUCTIONS,
    std::marker::PhantomData,
);
