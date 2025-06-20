// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static SPV_AMD_SHADER_BALLOT_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[
    ext_inst!(
        SpvAmdShaderBallot,
        SpvAmdShaderBallotOp,
        SwizzleInvocationsAMD,
        [],
        ["SPV_AMD_shader_ballot"],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        SpvAmdShaderBallot,
        SpvAmdShaderBallotOp,
        SwizzleInvocationsMaskedAMD,
        [],
        ["SPV_AMD_shader_ballot"],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        SpvAmdShaderBallot,
        SpvAmdShaderBallotOp,
        WriteInvocationAMD,
        [],
        ["SPV_AMD_shader_ballot"],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        SpvAmdShaderBallot,
        SpvAmdShaderBallotOp,
        MbcntAMD,
        [],
        ["SPV_AMD_shader_ballot"],
        [(IdRef, One)]
    ),
];
pub static SPV_AMD_SHADER_BALLOT_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(SPV_AMD_SHADER_BALLOT_INSTRUCTIONS, std::marker::PhantomData);
