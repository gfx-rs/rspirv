// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

include!("autogen_table.rs");
pub mod arm_motion_engine_100 {
    use super::*;
    include!("autogen_arm_motion_engine_100.rs");
}
pub use arm_motion_engine_100::ARM_MOTION_ENGINE_100_INSTRUCTION_TABLE;
pub mod debuginfo {
    use super::*;
    include!("autogen_debuginfo.rs");
}
pub use debuginfo::DEBUGINFO_INSTRUCTION_TABLE;
pub mod glsl_std_450 {
    use super::*;
    include!("autogen_glsl_std_450.rs");
}
pub use glsl_std_450::GLSL_STD_450_INSTRUCTION_TABLE;
pub mod nonsemantic_clspvreflection {
    use super::*;
    include!("autogen_nonsemantic_clspvreflection.rs");
}
pub use nonsemantic_clspvreflection::NONSEMANTIC_CLSPVREFLECTION_INSTRUCTION_TABLE;
pub mod nonsemantic_debugbreak {
    use super::*;
    include!("autogen_nonsemantic_debugbreak.rs");
}
pub use nonsemantic_debugbreak::NONSEMANTIC_DEBUGBREAK_INSTRUCTION_TABLE;
pub mod nonsemantic_debugprintf {
    use super::*;
    include!("autogen_nonsemantic_debugprintf.rs");
}
pub use nonsemantic_debugprintf::NONSEMANTIC_DEBUGPRINTF_INSTRUCTION_TABLE;
pub mod nonsemantic_shader_debuginfo_100 {
    use super::*;
    include!("autogen_nonsemantic_shader_debuginfo_100.rs");
}
pub use nonsemantic_shader_debuginfo_100::NONSEMANTIC_SHADER_DEBUGINFO_100_INSTRUCTION_TABLE;
pub mod nonsemantic_vkspreflection {
    use super::*;
    include!("autogen_nonsemantic_vkspreflection.rs");
}
pub use nonsemantic_vkspreflection::NONSEMANTIC_VKSPREFLECTION_INSTRUCTION_TABLE;
pub mod opencl_debuginfo_100 {
    use super::*;
    include!("autogen_opencl_debuginfo_100.rs");
}
pub use opencl_debuginfo_100::OPENCL_DEBUGINFO_100_INSTRUCTION_TABLE;
pub mod opencl_std_100 {
    use super::*;
    include!("autogen_opencl_std_100.rs");
}
pub use opencl_std_100::OPENCL_STD_100_INSTRUCTION_TABLE;
pub mod spv_amd_gcn_shader {
    use super::*;
    include!("autogen_spv_amd_gcn_shader.rs");
}
pub use spv_amd_gcn_shader::SPV_AMD_GCN_SHADER_INSTRUCTION_TABLE;
pub mod spv_amd_shader_ballot {
    use super::*;
    include!("autogen_spv_amd_shader_ballot.rs");
}
pub use spv_amd_shader_ballot::SPV_AMD_SHADER_BALLOT_INSTRUCTION_TABLE;
pub mod spv_amd_shader_explicit_vertex_parameter {
    use super::*;
    include!("autogen_spv_amd_shader_explicit_vertex_parameter.rs");
}
pub use spv_amd_shader_explicit_vertex_parameter::SPV_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_INSTRUCTION_TABLE;
pub mod spv_amd_shader_trinary_minmax {
    use super::*;
    include!("autogen_spv_amd_shader_trinary_minmax.rs");
}
pub use spv_amd_shader_trinary_minmax::SPV_AMD_SHADER_TRINARY_MINMAX_INSTRUCTION_TABLE;
pub mod tosa_001000_1 {
    use super::*;
    include!("autogen_tosa_001000_1.rs");
}
pub use tosa_001000_1::TOSA_001000_1_INSTRUCTION_TABLE;

pub fn ext_inst_table(set: &str) -> Option<&'static InstructionTable<ExtInstOp>> {
    Some(match set {
        "Arm.MotionEngine.100" => &ARM_MOTION_ENGINE_100_INSTRUCTION_TABLE,
        "DebugInfo" => &DEBUGINFO_INSTRUCTION_TABLE,
        "GLSL.std.450" => &GLSL_STD_450_INSTRUCTION_TABLE,
        "NonSemantic.ClspvReflection" => &NONSEMANTIC_CLSPVREFLECTION_INSTRUCTION_TABLE,
        "NonSemantic.DebugBreak" => &NONSEMANTIC_DEBUGBREAK_INSTRUCTION_TABLE,
        "NonSemantic.DebugPrintf" => &NONSEMANTIC_DEBUGPRINTF_INSTRUCTION_TABLE,
        "NonSemantic.Shader.DebugInfo.100" => &NONSEMANTIC_SHADER_DEBUGINFO_100_INSTRUCTION_TABLE,
        "NonSemantic.VkspReflection" => &NONSEMANTIC_VKSPREFLECTION_INSTRUCTION_TABLE,
        "OpenCL.DebugInfo.100" => &OPENCL_DEBUGINFO_100_INSTRUCTION_TABLE,
        "OpenCL.std" => &OPENCL_STD_100_INSTRUCTION_TABLE,
        "SPV_AMD_gcn_shader" => &SPV_AMD_GCN_SHADER_INSTRUCTION_TABLE,
        "SPV_AMD_shader_ballot" => &SPV_AMD_SHADER_BALLOT_INSTRUCTION_TABLE,
        "SPV_AMD_shader_explicit_vertex_parameter" => {
            &SPV_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_INSTRUCTION_TABLE
        }
        "SPV_AMD_shader_trinary_minmax" => &SPV_AMD_SHADER_TRINARY_MINMAX_INSTRUCTION_TABLE,
        "TOSA.001000.1" => &TOSA_001000_1_INSTRUCTION_TABLE,
        _ => return None,
    })
}
