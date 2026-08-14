// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static SPV_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_INSTRUCTIONS: &[ExtendedInstruction<'static>] =
    &[ext_inst!(
        SpvAmdShaderExplicitVertexParameter,
        SpvAmdShaderExplicitVertexParameterOp,
        InterpolateAtVertexAMD,
        [],
        ["SPV_AMD_shader_explicit_vertex_parameter"],
        [(IdRef, One), (IdRef, One)]
    )];
pub static SPV_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_INSTRUCTION_TABLE: InstructionTable<ExtInstOp> =
    InstructionTable(
        SPV_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_INSTRUCTIONS,
        std::marker::PhantomData,
    );
