// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

include!("autogen_table.rs");
pub mod glsl_std_450 {
    use super::*;
    include!("autogen_glsl_std_450.rs");
}
pub use glsl_std_450::GLSL_STD_450_INSTRUCTION_TABLE;
pub mod opencl_std_100 {
    use super::*;
    include!("autogen_opencl_std_100.rs");
}
pub use opencl_std_100::OPENCL_STD_100_INSTRUCTION_TABLE;
pub mod nonsemantic_debugprintf {
    use super::*;
    include!("autogen_nonsemantic_debugprintf.rs");
}
pub use nonsemantic_debugprintf::NONSEMANTIC_DEBUGPRINTF_INSTRUCTION_TABLE;

pub fn ext_inst_table(set: &str) -> Option<&'static InstructionTable<ExtInstOp>> {
    Some(match set {
        "GLSL.std.450" => &GLSL_STD_450_INSTRUCTION_TABLE,
        "OpenCL.std" => &OPENCL_STD_100_INSTRUCTION_TABLE,
        "NonSemantic.DebugPrintF" => &NONSEMANTIC_DEBUGPRINTF_INSTRUCTION_TABLE,
        _ => return None,
    })
}
