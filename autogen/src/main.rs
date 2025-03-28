#![recursion_limit = "128"]

mod binary;
mod dr;
mod header;
mod sr;
mod structs;
mod table;
mod utils;

use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process,
};
use utils::write_autogen_comment;

fn write(path: &Path, contents: impl ToString) {
    let mut f = fs::File::create(path).unwrap_or_else(|_| panic!("cannot open file: {:?}", path));
    write_autogen_comment(&mut f);
    write!(f, "{}", contents.to_string()).unwrap()
}

fn write_formatted(path: &Path, contents: impl ToString) {
    write(path, contents);
    match process::Command::new("rustfmt").arg(path).status() {
        Ok(status) if !status.success() => {
            println!("cargo:warning=failed to rustfmt {:?}", path);
        }
        Ok(_) => {}
        Err(_) => {
            println!("cargo:warning=failed to execute rustfmt");
        }
    };
}

/// Maps some reserved instructions in the spec into their expected class in
/// order to generate the proper methods for those instructions.
fn map_reserved_instructions(grammar: &mut structs::Grammar) {
    for instruction in grammar
        .instructions
        .iter_mut()
        .filter(|i| i.class == Some(structs::Class::Reserved))
    {
        if instruction.opname.starts_with("OpType") {
            instruction.class = Some(structs::Class::Type);
        }
    }
}

fn opname_suffix_weight(opname: &str) -> u32 {
    const VENDORS: [&str; 36] = [
        "AMD",
        "AMDX",
        "ANDROID",
        "ARM",
        "BRCM",
        "CHROMIUM",
        "EXT",
        "FB",
        "FSL",
        "FUCHSIA",
        "GGP",
        "GOOGLE",
        "HUAWEI",
        "IMG",
        "INTEL",
        "JUICE",
        "KDAB",
        "KHX",
        "LUNARG",
        "MESA",
        "MVK",
        "NN",
        "NV",
        "NVX",
        "NXP",
        "NZXT",
        "QCOM",
        "QNX",
        "RASTERGRID",
        "RENDERDOC",
        "SAMSUNG",
        "SEC",
        "TIZEN",
        "VALVE",
        "VIV",
        "VSI",
    ];

    match opname {
        _ if opname.ends_with("KHR") => 1,
        _ if opname.ends_with("EXT") => 2,
        _ if VENDORS.iter().any(|v| opname.ends_with(v)) => 3,
        _ => 0,
    }
}

fn sort_instructions(grammar: &mut structs::Grammar) {
    grammar.instructions.sort_by(|l, r| {
        l.opcode
            .cmp(&r.opcode)
            .then_with(|| opname_suffix_weight(&l.opname).cmp(&opname_suffix_weight(&r.opname)))
    })
}

fn main() {
    // Path to the SPIR-V core grammar file.
    let env_var = env::var("CARGO_MANIFEST_DIR").unwrap();
    let autogen_src_dir = PathBuf::from(&env_var);

    if !autogen_src_dir.join("external/SPIRV-Headers").exists() {
        panic!("SPIRV-Headers missing - please checkout using git submodule");
    }

    let grammar: structs::Grammar = {
        let mut original = serde_json::from_str(
            &fs::read_to_string(
                autogen_src_dir
                    .join("external/SPIRV-Headers/include/spirv/unified1/spirv.core.grammar.json"),
            )
            .unwrap(),
        )
        .unwrap();
        map_reserved_instructions(&mut original);
        sort_instructions(&mut original);
        original
    };

    let extended_instruction_sets = [
        ("GLSL.std.450", "GLOp", "https://www.khronos.org/registry/spir-v/specs/unified1/GLSL.std.450.html"),
        ("OpenCL.std.100", "CLOp", "https://www.khronos.org/registry/spir-v/specs/unified1/OpenCL.ExtendedInstructionSet.100.html"),
        ("NonSemantic.DebugPrintF", "DebugPrintFOp", "https://github.com/KhronosGroup/Vulkan-ValidationLayers/blob/master/docs/debug_printf.md"),
    ];
    let extended_instruction_sets = extended_instruction_sets.map(|(ext, op, url)| {
        let grammar: structs::ExtInstSetGrammar = serde_json::from_str(
            &std::fs::read_to_string(autogen_src_dir.join(format!(
                "external/SPIRV-Headers/include/spirv/unified1/extinst.{}.grammar.json",
                ext.to_lowercase()
            )))
            .unwrap(),
        )
        .unwrap();
        (ext, op, url, grammar)
    });

    // SPIR-V header
    write_formatted(&autogen_src_dir.join("../spirv/autogen_spirv.rs"), {
        let core = header::gen_spirv_header(&grammar);
        let extended_instruction_sets =
            extended_instruction_sets
                .iter()
                .map(|(ext, op, url, grammar)| {
                    header::gen_opcodes(
                        op,
                        grammar,
                        &format!("[{}]({}) extended instruction opcode", ext, url),
                    )
                    .to_string()
                });
        format!(
            "{}\n{}",
            core,
            extended_instruction_sets.collect::<Vec<_>>().join("\n")
        )
    });

    // Instruction table
    write_formatted(
        &autogen_src_dir.join("../rspirv/grammar/autogen_table.rs"),
        table::gen_grammar_inst_table_operand_kinds(&grammar),
    );
    // Extended instruction sets
    for (ext, _, _, grammar) in extended_instruction_sets {
        write_formatted(
            &autogen_src_dir.join(format!(
                "../rspirv/grammar/autogen_{}.rs",
                ext.replace(".", "_").to_lowercase()
            )),
            table::gen_instruction_table(
                &grammar.instructions,
                &format!("{}_INSTRUCTION_TABLE", ext.replace(".", "_").to_uppercase()),
                true,
            ),
        );
    }

    // Path to the generated operands kind in data representation.
    write_formatted(
        &autogen_src_dir.join("../rspirv/dr/autogen_operand.rs"),
        dr::gen_dr_operand_kinds(&grammar.operand_kinds),
    );
    // Path to the generated builder for data representation.
    write_formatted(
        &autogen_src_dir.join("../rspirv/dr/build/autogen_type.rs"),
        dr::gen_dr_builder_types(&grammar),
    );
    // Path to the generated builder for data representation.
    write_formatted(
        &autogen_src_dir.join("../rspirv/dr/build/autogen_terminator.rs"),
        dr::gen_dr_builder_terminator(&grammar),
    );
    // Path to the generated builder for data representation.
    write_formatted(
        &autogen_src_dir.join("../rspirv/dr/build/autogen_annotation.rs"),
        dr::gen_dr_builder_annotation(&grammar),
    );
    // Path to the generated builder for data representation.
    write_formatted(
        &autogen_src_dir.join("../rspirv/dr/build/autogen_constant.rs"),
        dr::gen_dr_builder_constants(&grammar),
    );
    // Path to the generated builder for data representation.
    write_formatted(
        &autogen_src_dir.join("../rspirv/dr/build/autogen_debug.rs"),
        dr::gen_dr_builder_debug(&grammar),
    );
    // Path to the generated builder for data representation.
    write_formatted(
        &autogen_src_dir.join("../rspirv/dr/build/autogen_norm_insts.rs"),
        dr::gen_dr_builder_normal_insts(&grammar),
    );

    // Path to the generated decoding errors.
    write_formatted(
        &autogen_src_dir.join("../rspirv/binary/autogen_error.rs"),
        binary::gen_operand_decode_errors(&grammar.operand_kinds),
    );
    // Path to the generated operand decoding methods.
    write_formatted(
        &autogen_src_dir.join("../rspirv/binary/autogen_decode_operand.rs"),
        binary::gen_operand_decode_methods(&grammar.operand_kinds),
    );
    // Path to the generated operand parsing methods.
    write_formatted(
        &autogen_src_dir.join("../rspirv/binary/autogen_parse_operand.rs"),
        binary::gen_operand_parse_methods(&grammar.operand_kinds),
    );
    // Path to the generated operand parsing methods.
    write_formatted(
        &autogen_src_dir.join("../rspirv/binary/autogen_disas_operand.rs"),
        binary::gen_disas_bit_enum_operands(&grammar.operand_kinds),
    );

    let operands = sr::gen_sr_code_from_operand_kind_grammar(&grammar.operand_kinds);
    write_formatted(
        &autogen_src_dir.join("../rspirv/sr/autogen_decoration.rs"),
        operands.decoration,
    );
    let instructions = sr::gen_sr_code_from_instruction_grammar(&grammar.instructions);
    write_formatted(
        &autogen_src_dir.join("../rspirv/sr/autogen_types.rs"),
        instructions.types,
    );
    write_formatted(
        &autogen_src_dir.join("../rspirv/sr/autogen_instructions.rs"),
        instructions.instructions,
    );
    write_formatted(
        &autogen_src_dir.join("../rspirv/sr/autogen_ops.rs"),
        instructions.ops,
    );
    write_formatted(
        &autogen_src_dir.join("../rspirv/lift/autogen_context.rs"),
        instructions.lift_context,
    );
}
