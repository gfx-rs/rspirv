// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod binary;
mod header;
mod memrepr;
mod structs;
mod table;
mod utils;

use std::{env, fs, path};
use std::io::{Read, Write};
use utils::write_copyright_autogen_comment;

macro_rules! write {
    ($content: expr, $path: expr) => {
        {
            let mut f = fs::File::create($path.to_str().unwrap()).unwrap();
            write_copyright_autogen_comment(&mut f);
            f.write_all(&$content.into_bytes()).unwrap();
        }
    }
}

fn main() {
    if env::var("CARGO_FEATURE_REGEN").is_err() {
        println!("use existing code to compile since 'regen' not specified");
        return
    }

    // Path to the SPIR-V core grammar file.
    let mut path = path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("external");
    path.push("spirv.core.grammar.json");

    let mut contents = String::new();
    {
        let filename = path.to_str().unwrap();
        let mut file = fs::File::open(filename).unwrap();
        file.read_to_string(&mut contents).unwrap();
    }
    let grammar: structs::Grammar = serde_json::from_str(&contents).unwrap();

    {
        // Path to the generated SPIR-V header file.
        path.pop();
        path.pop();
        path.push("spirv.rs");
        let c = header::gen_spirv_header(&grammar);
        write!(c, path);
    }

    {
        // Path to the generated instruction table.
        path.pop();
        path.push("grammar");
        path.push("table.rs");
        let c = table::gen_grammar_inst_table_operand_kinds(&grammar);
        write!(c, path);
    }

    {
        // Path to the generated operands kind in memory representation.
        path.pop();
        path.pop();
        path.push("mr");
        path.push("operand.rs");
        let c = memrepr::gen_mr_operand_kinds(&grammar.operand_kinds);
        write!(c, path);
    }

    {
        // Path to the generated builder for memory representation.
        path.pop();
        path.push("build_type.rs");
        let c = memrepr::gen_mr_builder_types(&grammar.instructions);
        write!(c, path);
    }
    {
        // Path to the generated builder for memory representation.
        path.pop();
        path.push("build_terminator.rs");
        let c = memrepr::gen_mr_builder_terminator(&grammar.instructions);
        write!(c, path);
    }

    {
        // Path to the generated decoding errors.
        path.pop();
        path.pop();
        path.push("binary");
        path.push("error.rs");
        let c = binary::gen_operand_decode_errors(&grammar.operand_kinds);
        write!(c, path);
    }

    {
        // Path to the generated operand decoding methods.
        path.pop();
        path.push("decode_operand.rs");
        let c = binary::gen_operand_decode_methods(&grammar.operand_kinds);
        write!(c, path);
    }
    {
        // Path to the generated operand parsing methods.
        path.pop();
        path.push("parse_operand.rs");
        let c = binary::gen_operand_parse_methods(&grammar.operand_kinds);
        write!(c, path);
    }

    // For GLSLstd450 extended instruction set.
    path.pop();
    path.pop();
    path.push("external");
    path.push("extinst.glsl.std.450.grammar.json");

    {
        let filename = path.to_str().unwrap();
        let mut file = fs::File::open(filename).unwrap();
        contents.clear();
        file.read_to_string(&mut contents).unwrap();
    }
    let grammar: structs::GlslGrammar = serde_json::from_str(&contents).unwrap();

    {
        // Path to the generated GLSLstd450 extended instruction set header.
        path.pop();
        path.pop();
        path.push("grammar");
        path.push("glsl_std_450.rs");
        let c = table::gen_glsl_std_450_inst_table(&grammar);
        write!(c, path);
    }
}
