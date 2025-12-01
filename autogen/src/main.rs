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

    // (import_name, file_key, op, url)
    // import_name: canonical OpExtInstImport string (case-sensitive, per SPIRV-Tools)
    // file_key: grammar filename stem (lowercase), may differ from import_name
    let extended_instruction_sets = [
        ("GLSL.std.450", "glsl.std.450", "GLOp", "https://registry.khronos.org/SPIR-V/specs/unified1/GLSL.std.450.html"),
        ("OpenCL.std", "opencl.std.100", "CLOp", "https://registry.khronos.org/SPIR-V/specs/unified1/OpenCL.ExtendedInstructionSet.100.html"),
        ("NonSemantic.DebugPrintF", "nonsemantic.debugprintf", "DebugPrintFOp", "https://github.khronos.org/SPIRV-Registry/nonsemantic/NonSemantic.DebugPrintf.html"),
    ];
    let extended_instruction_sets = extended_instruction_sets.map(|(ext, file_key, op, url)| {
        let grammar: structs::ExtInstSetGrammar = serde_json::from_str(
            &std::fs::read_to_string(autogen_src_dir.join(format!(
                "external/SPIRV-Headers/include/spirv/unified1/extinst.{file_key}.grammar.json",
            )))
            .unwrap(),
        )
        .unwrap();
        (ext, file_key, op, url, grammar)
    });

    // SPIR-V header
    write_formatted(&autogen_src_dir.join("../spirv/autogen_spirv.rs"), {
        let core = header::gen_spirv_header(&grammar);
        let extended_instruction_sets =
            extended_instruction_sets
                .iter()
                .map(|(ext, _file_key, op, url, grammar)| {
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

    // Derive module and variant names for extensions with operand kinds
    let ext_info: Vec<_> = extended_instruction_sets
        .iter()
        .map(|(_ext, file_key, _, _, grammar)| {
            let module_name = file_key.replace(['.', '-'], "_");
            let variant_name: String = file_key
                .split(['.', '-'])
                .flat_map(|part| {
                    let mut chars = part.chars();
                    // Uppercase the first character
                    chars.next().unwrap().to_uppercase().chain(chars)
                })
                .collect();
            let has_operand_kinds = !grammar.operand_kinds.is_empty();
            (module_name, variant_name, has_operand_kinds)
        })
        .collect();

    // Wrapper variants for core OperandKind: only extensions with operand kinds
    let ext_wrapper_variants: Vec<(&str, &str)> = ext_info
        .iter()
        .filter(|(_, _, has)| *has)
        .map(|(module, variant, _)| (variant.as_str(), module.as_str()))
        .collect();

    // Collect ExtInstOp variant info: (variant_name, op_name) for all extended sets
    let ext_inst_variants: Vec<(&str, &str)> = extended_instruction_sets
        .iter()
        .map(|(_, _, op, _, _)| {
            let variant = op.strip_suffix("Op").unwrap_or(op);
            (variant, *op)
        })
        .collect();

    // Instruction table
    let mut tables = String::new();
    tables.push_str("include!(\"autogen_table.rs\");\n");
    let mut table_lookup = r#"pub fn ext_inst_table(set: &str) ->
            Option<&'static InstructionTable<ExtInstOp>> {
        Some(match set {"#
        .to_owned();
    write_formatted(
        &autogen_src_dir.join("../rspirv/grammar/autogen_table.rs"),
        table::gen_grammar_inst_table_operand_kinds(
            &grammar.operand_kinds,
            &grammar.instructions,
            &ext_wrapper_variants,
            &ext_inst_variants,
        ),
    );
    // Extended instruction sets
    for (
        (ext, _file_key, spirv_op, _, ext_grammar),
        (module_name, variant_name, _has_operand_kinds),
    ) in extended_instruction_sets.iter().zip(&ext_info)
    {
        let autogen_file = format!("autogen_{module_name}.rs");
        let table_name = format!("{}_INSTRUCTION", module_name.to_uppercase());
        let ext_variant_name = spirv_op.strip_suffix("Op").unwrap_or(spirv_op);
        write_formatted(
            &autogen_src_dir.join(format!("../rspirv/grammar/{autogen_file}")),
            table::gen_ext_instruction_file(
                &ext_grammar.operand_kinds,
                &ext_grammar.instructions,
                spirv_op,
                ext_variant_name,
                &table_name,
                variant_name,
            ),
        );
        tables.push_str(&format!(
            "pub mod {module_name} {{ use super::*; include!(\"{autogen_file}\"); }}\n\
             pub use {module_name}::{table_name}_TABLE;\n"
        ));
        table_lookup.push_str(&format!("\"{ext}\" => &{table_name}_TABLE,\n"));
    }
    write_formatted(
        &autogen_src_dir.join("../rspirv/grammar/autogen_tables.rs"),
        format!("{tables}\n{table_lookup} _ => return None,}})}}\n"),
    );

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
        {
            let variant_names: Vec<&str> = ext_wrapper_variants
                .iter()
                .map(|(variant, _)| *variant)
                .collect();
            binary::gen_operand_parse_methods(&grammar.operand_kinds, &variant_names)
        },
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
