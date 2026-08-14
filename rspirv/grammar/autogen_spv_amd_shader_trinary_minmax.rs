// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static SPV_AMD_SHADER_TRINARY_MINMAX_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[
    ext_inst!(
        SpvAmdShaderTrinaryMinmax,
        SpvAmdShaderTrinaryMinmaxOp,
        FMin3AMD,
        [],
        ["SPV_AMD_shader_trinary_minmax"],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        SpvAmdShaderTrinaryMinmax,
        SpvAmdShaderTrinaryMinmaxOp,
        UMin3AMD,
        [],
        ["SPV_AMD_shader_trinary_minmax"],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        SpvAmdShaderTrinaryMinmax,
        SpvAmdShaderTrinaryMinmaxOp,
        SMin3AMD,
        [],
        ["SPV_AMD_shader_trinary_minmax"],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        SpvAmdShaderTrinaryMinmax,
        SpvAmdShaderTrinaryMinmaxOp,
        FMax3AMD,
        [],
        ["SPV_AMD_shader_trinary_minmax"],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        SpvAmdShaderTrinaryMinmax,
        SpvAmdShaderTrinaryMinmaxOp,
        UMax3AMD,
        [],
        ["SPV_AMD_shader_trinary_minmax"],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        SpvAmdShaderTrinaryMinmax,
        SpvAmdShaderTrinaryMinmaxOp,
        SMax3AMD,
        [],
        ["SPV_AMD_shader_trinary_minmax"],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        SpvAmdShaderTrinaryMinmax,
        SpvAmdShaderTrinaryMinmaxOp,
        FMid3AMD,
        [],
        ["SPV_AMD_shader_trinary_minmax"],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        SpvAmdShaderTrinaryMinmax,
        SpvAmdShaderTrinaryMinmaxOp,
        UMid3AMD,
        [],
        ["SPV_AMD_shader_trinary_minmax"],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        SpvAmdShaderTrinaryMinmax,
        SpvAmdShaderTrinaryMinmaxOp,
        SMid3AMD,
        [],
        ["SPV_AMD_shader_trinary_minmax"],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
];
pub static SPV_AMD_SHADER_TRINARY_MINMAX_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(
        SPV_AMD_SHADER_TRINARY_MINMAX_INSTRUCTIONS,
        std::marker::PhantomData,
    );
