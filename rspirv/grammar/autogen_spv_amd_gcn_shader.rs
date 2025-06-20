// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static SPV_AMD_GCN_SHADER_INSTRUCTIONS: &[ExtendedInstruction<'static>] = &[
    ext_inst!(
        SpvAmdGcnShader,
        SpvAmdGcnShaderOp,
        CubeFaceIndexAMD,
        [],
        ["SPV_AMD_gcn_shader"],
        [(IdRef, One)]
    ),
    ext_inst!(
        SpvAmdGcnShader,
        SpvAmdGcnShaderOp,
        CubeFaceCoordAMD,
        [],
        ["SPV_AMD_gcn_shader"],
        [(IdRef, One)]
    ),
    ext_inst!(
        SpvAmdGcnShader,
        SpvAmdGcnShaderOp,
        TimeAMD,
        [],
        ["SPV_AMD_gcn_shader"],
        []
    ),
];
pub static SPV_AMD_GCN_SHADER_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(SPV_AMD_GCN_SHADER_INSTRUCTIONS, std::marker::PhantomData);
