// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static NONSEMANTIC_GRAPH_DEBUGINFO_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[
    ext_inst!(
        NonsemanticGraphDebuginfo,
        NonsemanticGraphDebuginfoOp,
        DebugGraph,
        [],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        NonsemanticGraphDebuginfo,
        NonsemanticGraphDebuginfoOp,
        DebugOperation,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, ZeroOrMore)]
    ),
    ext_inst!(
        NonsemanticGraphDebuginfo,
        NonsemanticGraphDebuginfoOp,
        DebugTensor,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, ZeroOrOne)]
    ),
];
pub static NONSEMANTIC_GRAPH_DEBUGINFO_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(
        NONSEMANTIC_GRAPH_DEBUGINFO_INSTRUCTIONS,
        std::marker::PhantomData,
    );
