// Copyright 2017 Google Inc.
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

extern crate regex;

use std::fs;
use std::io::Write;

#[cfg_attr(rustfmt, rustfmt_skip)]
static COPYRIGHT : &'static str = "\
// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the \"License\");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an \"AS IS\" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.";

#[cfg_attr(rustfmt, rustfmt_skip)]
static AUTOGEN_COMMENT : &'static str = "\
// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!";

pub static RUSTFMT_SKIP: &'static str = "#[cfg_attr(rustfmt, rustfmt_skip)]";
pub static RUSTFMT_SKIP_BANG: &'static str = "#![cfg_attr(rustfmt, rustfmt_skip)]";

pub fn write_copyright_autogen_comment(file: &mut fs::File) {
    file.write_all(COPYRIGHT.as_bytes()).unwrap();
    file.write_all(b"\n\n").unwrap();
    file.write_all(AUTOGEN_COMMENT.as_bytes()).unwrap();
    file.write_all(b"\n\n").unwrap();
}

/// Converts the given `symbol` to use snake case style.
pub fn snake_casify(symbol: &str) -> String {
    let re = regex::Regex::new(r"(?P<l>[a-z])(?P<u>[A-Z])").unwrap();
    re.replace_all(symbol, "$l-$u").replace("-", "_").to_lowercase()
}

/// Returns the corresponding operand kind in memory representation for the
/// given operand `kind` in the grammar.
pub fn get_mr_operand_kind(kind: &str) -> &str {
    if kind == "LiteralInteger" {
        "LiteralInt32"
    } else if kind == "LiteralContextDependentNumber" {
        // TODO: should use the correct type to decode
        "LiteralInt32"
    } else {
        kind
    }
}
