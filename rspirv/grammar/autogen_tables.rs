// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

include!("autogen_table.rs");
include!("autogen_glsl_std_450.rs");
include!("autogen_opencl_std_100.rs");
include!("autogen_nonsemantic_debugprintf.rs");

pub fn ext_inst_table(set: &str) -> Option<&'static InstructionTable<ExtInstOp>> {
    Some(match set {
        "GLSL.std.450" => &GLSL_STD_450_INSTRUCTION_TABLE,
        "OpenCL.std" => &OPENCL_STD_100_INSTRUCTION_TABLE,
        "NonSemantic.DebugPrintF" => &NONSEMANTIC_DEBUGPRINTF_INSTRUCTION_TABLE,
        _ => return None,
    })
}
