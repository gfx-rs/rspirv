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

use crate::structs;
use crate::utils::*;

/// Returns the name of the method for decoding the given operand `kind`
/// in grammar.
fn get_decode_method(kind: &str) -> String {
    if kind.starts_with("Id") {
        return "id".to_string();
    }

    let mut kind = kind;
    if kind.starts_with("Literal") {
        kind = &kind["Literal".len()..];
        if kind == "Integer" {
            return "int32".to_string();
        }
    }
    snake_casify(kind).to_string()
}

/// Returns the generated operand decoding errors for binary::Decoder by
/// walking the given SPIR-V operand kinds `grammar`.
pub fn gen_operand_decode_errors(grammar: &Vec<structs::OperandKind>)
                                 -> String {
    let mut ret = String::new();

    { // Comments, attributes, uses.
        ret.push_str(RUSTFMT_SKIP_BANG);
        ret.push_str("\n\nuse spirv;\nuse std::{error, fmt};\n\n");
    }

    let kinds: Vec<&str> = grammar.iter().filter(|element| {
        !(element.kind.starts_with("Pair") ||
          element.kind.starts_with("Id") ||
          element.kind.starts_with("Literal"))
    }).map(|element| {
        element.kind.as_str()
    }).collect();

    // The Error enum.
    let errors: Vec<String> = kinds.iter().map(|element| {
        format!("    {}Unknown(usize, spirv::Word),", element)
    }).collect();
    let error_enum = format!(
        "/// Decoder Error.\n\
         #[derive(Debug, PartialEq)]\n\
         pub enum Error {{\n\
         {s:4}StreamExpected(usize),\n\
         {s:4}LimitReached(usize),\n\
         {errors}\n\
         {s:4}/// Failed to decode a string.\n\
         {s:4}///\n\
         {s:4}/// For structured error handling, the second element could be\n\
         {s:4}/// `string::FromUtf8Error`, but the will prohibit the compiler\n\
         {s:4}/// from generating `PartialEq` for this enum.\n\
         {s:4}DecodeStringFailed(usize, String),\n\
         }}\n\n",
        s = "",
        errors = errors.join("\n"));
    ret.push_str(&error_enum);

    // impl fmt::Display for the Error enum.
    let errors: Vec<String> = kinds.iter().map(|element| {
        format!("{s:12}Error::{kind}Unknown(index, word) => write!(\
                 f, \"unknown value {{}} for operand kind {kind} \
                 at index {{}}\", word, index),",
                s = "",
                kind = element)
    }).collect();
    let display_impl = format!(
        "impl fmt::Display for Error {{\n\
         {s:4}fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{\n\
         {s:8}match *self {{\n\
         {s:12}Error::StreamExpected(index) => write!(f, \"expected more \
             bytes in the stream at index {{}}\", index),\n\
         {s:12}Error::LimitReached(index) => write!(f, \"reached word limit \
             at index {{}}\", index),\n\
         {errors}\n\
         {s:12}Error::DecodeStringFailed(index, ref e) => write!(f, \
             \"cannot decode string at index {{}}: {{}}\", index, e),\n\
         {s:8}}}\n{s:4}}}\n}}\n\n",
        s = "",
        errors = errors.join("\n"));
    ret.push_str(&display_impl);

    // impl error::Error for the Error enum.
    let error_impl = format!(
        "impl error::Error for Error {{\n\
         {s:4}fn description(&self) -> &str {{\n\
         {s:8}match *self {{\n\
         {s:12}Error::StreamExpected(_) => \"expected more bytes \
             in the stream\",\n\
         {s:12}_ => \"unknown operand value for the given kind\",\n\
         {s:8}}}\n{s:4}}}\n}}\n",
        s = "");
    ret.push_str(&error_impl);

    ret
}

/// Returns the generated operand decoding methods for binary::Decoder by
/// walking the given SPIR-V operand kinds `grammar.
pub fn gen_operand_decode_methods(grammar: &Vec<structs::OperandKind>)
                                  -> String {
    let mut ret = String::new();

    ret.push_str("use num::FromPrimitive;\n\n");

    let methods: Vec<String> = grammar.iter().filter(|element| {
        // For kinds whose values may occupy more than one word, we need to
        // implement manually.
        !(element.kind.starts_with("Pair") ||
          element.kind.starts_with("Id") ||
          element.kind.starts_with("Literal"))
    }).map(|element| {
        // Method definition for decoding values of a particular operand
        // kind. If the operand kind belongs to BitEnum, we use from_bits(),
        // otherwise, from_u32().
        format!(
            "{s:4}/// Decodes and returns the next SPIR-V word as\n\
             {s:4}/// a SPIR-V {kind} value.\n\
             {s:4}pub fn {fname}(&mut self) -> Result<spirv::{kind}> {{\n\
             {s:8}if let Ok(word) = self.word() {{\n\
                 {s:12}spirv::{kind}::from_{ty}(word).ok_or(Error::\
                 {kind}Unknown(self.offset - WORD_NUM_BYTES, word))\n\
             {s:8}}} else {{\n\
                 {s:12}Err(Error::StreamExpected(self.offset))\n\
             {s:8}}}\n{s:4}}}\n",
             s = "",
             fname = snake_casify(&element.kind),
             kind = element.kind,
             ty = if element.category == "BitEnum" { "bits" } else { "u32" })
    }).collect();
    ret.push_str(&format!("impl<'a> Decoder<'a> {{\n{}}}\n", methods.join("\n")));

    ret
}

/// Generates the methods for parsing parameters of operand kind enumerants.
/// Returns a vector of tuples, with the first element being the enumerant
/// symbol, and the second element being a list of parameter kinds to that
/// enumerant.
fn gen_operand_param_parse_methods(grammar: &Vec<structs::OperandKind>)
                                   -> Vec<(&str, String)> {
    grammar.iter().filter(|element| {
        // Filter out all the operand kinds without any enumerants.
        element.enumerants.len() != 0
    }).filter_map(|element| {
        let kind = element.kind.as_str();
        // Get the symbol and all the parameters for each enumerant.
        let pairs: Vec<(&str, Vec<&str>)> =
            element.enumerants.iter().filter_map(|e| {
                let params: Vec<&str> = e.parameters.iter().map(
                    |p| { p.kind.as_str() }
                ).collect();
                if params.is_empty() {
                    // Filter out enumerants without further parameters.
                    None
                } else {
                    Some((e.symbol.as_str(),  params))
                }
            }).collect();

        if pairs.is_empty() {
            // Skip those operand kinds that don't have parameters for
            // further parsing.
            return None
        }

        let method = if element.category == "BitEnum" {
            // For each operand kind in the BitEnum category, its
            // enumerants are bit masks. If a certain bit having associated
            // parameters is set, we also need to decode the corresponding
            // parameters. E.g., for MemoryAccess Aigned, an additional
            // LiteralInteger, which stands for the known alignment, should
            // be decoded.

            let lo_kind = snake_casify(kind);

            // Compose bit-set-clear check for each bit requiring
            // associated parameters.
            let cases: Vec<String> = pairs.into_iter().map(|(symbol, params)| {
                let params: Vec<String> = params.iter().map(|element| {
                    format!("dr::Operand::{kind}(self.decoder.{decode}()?)",
                            kind = get_mr_operand_kind(element),
                            decode = get_decode_method(element))
                }).collect();
                format!(
                    "{s:8}if {arg}.contains(spirv::{kind}::{bit}) {{\n\
                         {s:12}params.append(&mut vec![{params}]);\n\
                     {s:8}}}",
                    s = "",
                    arg = lo_kind,
                    kind = kind,
                    bit = snake_casify(&symbol).to_string().to_uppercase(),
                    params = params.join(", "))
            }).collect();
            format!(
                "{s:4}fn parse_{k}_arguments(&mut self, {k}: \
                     spirv::{kind}) -> Result<Vec<dr::Operand>> {{\n\
                     {s:8}let mut params = vec![];\n\
                     {cases}\n\
                     {s:8}Ok(params)\n\
                 {s:4}}}",
                s = "",
                cases = cases.join("\n"),
                k = lo_kind,
                kind = kind)
        } else {  // ValueEnum
            let cases: Vec<String> = pairs.into_iter().map(|(symbol, params)| {
                let params: Vec<String> = params.iter().map(|element| {
                    format!("dr::Operand::{kind}(self.decoder.{decode}()?)",
                            kind = get_mr_operand_kind(element),
                            decode = get_decode_method(element))
                }).collect();
                format!(
                    "{s:12}spirv::{kind}::{symbol} => vec![{params}],",
                    s = "",
                    kind = kind,
                    symbol = symbol,
                    params = params.join(", "))
            }).collect();
            format!(
                "{s:4}fn parse_{k}_arguments(&mut self, {k}: spirv::{kind})\
                     {s:1}-> Result<Vec<dr::Operand>> {{\n\
                     {s:8}Ok(match {k} {{\n\
                        {cases}\n\
                        {s:12}_ => vec![]\n\
                     {s:8}}})\n\
                 {s:4}}}",
                s = "",
                kind = kind,
                k = snake_casify(kind),
                cases = cases.join("\n"))
        };
        Some((kind, method))
    }).collect()
}

/// Returns the generated operand parsing methods for binary::Parser by
/// walking the given SPIR-V operand kinds `grammar`.
pub fn gen_operand_parse_methods(grammar: &Vec<structs::OperandKind>) -> String {
    // Operand kinds whose enumerants have parameters. For these kinds, we need
    // to decode more than just the enumerants themselves.
    let (further_parse_kinds, further_parse_methods): (Vec<_>, Vec<_>) =
        gen_operand_param_parse_methods(grammar).iter().cloned().unzip();
    let further_parse_cases: Vec<String> =
        further_parse_kinds.iter().map(|kind| {
            format!(
                "{s:12}GOpKind::{kind} => {{\n\
                 {s:16}let val = self.decoder.{decode}()?;\n\
                 {s:16}let mut ops = vec![dr::Operand::{kind}(val)];\n\
                 {s:16}ops.append(&mut self.parse_{k}_arguments(val)?);\n\
                 {s:16}ops\n\
                 {s:12}}}",
                s = "",
                kind = kind,
                k = snake_casify(kind),
                decode = get_decode_method(kind))
        }).collect();

    // Logic operands that expand to concrete operand pairs,
    // that is, those operand kinds with 'Pair' name prefix.
    // We only have three cases. So hard code it.
    let pair_kinds = vec![
        ("LiteralInteger", "IdRef"),
        ("IdRef", "LiteralInteger"),
        ("IdRef", "IdRef"),
    ];
    let pair_cases: Vec<String> = pair_kinds.iter().map(|&(k0, k1)| {
        format!("{s:12}GOpKind::{kind} => {{\n\
                 {s:16}vec![\
                 dr::Operand::{k0}(self.decoder.{m0}()?), \
                 dr::Operand::{k1}(self.decoder.{m1}()?)\
                 ]\n{s:12}}}",
                s = "",
                kind = format!("Pair{}{}", k0, k1),
                k0 = get_mr_operand_kind(k0),
                k1 = get_mr_operand_kind(k1),
                m0 = get_decode_method(k0), m1=get_decode_method(k1))
    }).collect();

    // These kinds are manually handled.
    let manual_kinds = vec!["IdResultType", "IdResult",
                            "LiteralContextDependentNumber",
                            "LiteralSpecConstantOpInteger"];

    // For the rest operand kinds, which takes exactly one word.
    let normal_cases: Vec<String> = grammar.iter().filter_map(|element| {
        if further_parse_kinds.iter().any(|k| *k == element.kind) ||
            manual_kinds.iter().any(|k| *k == element.kind) ||
            element.kind.starts_with("Pair") {
                None
            } else {
                Some(element.kind.as_str())
            }
    }).map(|kind| {
        format!(
            "{s:12}GOpKind::{gkind} => vec![dr::Operand::{mkind}\
             (self.decoder.{decode}()?)],",
             s = "",
             gkind = kind,
             mkind = get_mr_operand_kind(kind),
             decode = get_decode_method(kind))
    }).collect();

    let manual_cases: Vec<String> =
        manual_kinds.iter().map(|element| {
            format!("{s:12}GOpKind::{k} => panic!(),  // not handled here",
                    s = "",
                    k = element)
        }).collect();

    format!(
        "impl<'c, 'd> Parser<'c, 'd> {{\n\
         {s:4}fn parse_operand(&mut self, kind: GOpKind) \
             -> Result<Vec<dr::Operand>> {{\n\
             {s:8}Ok(match kind {{\n\
                 {normal_cases}\n\
                 {pair_cases}\n\
                 {further_parse_cases}\n\
                 {manual_cases}\n\
             {s:8}}})\n\
         {s:4}}}\n\n\
         {functions}\n\
         }}",
        s = "",
        normal_cases = normal_cases.join("\n"),
        pair_cases = pair_cases.join("\n"),
        further_parse_cases = further_parse_cases.join("\n"),
        manual_cases = manual_cases.join("\n"),
        functions = further_parse_methods.join("\n\n"))
}

pub fn gen_disas_bit_enum_operands(grammar: &Vec<structs::OperandKind>) -> String {
    let elements: Vec<String> = grammar.iter().filter(|kind| {
        kind.category == "BitEnum"
    }).map(|kind| {
        let checks: Vec<String> = kind.enumerants.iter().filter_map(|enumerant| {
            if enumerant.value.string == "0x0000" {
                None
            } else {
                let mut symbol = snake_casify(&enumerant.symbol).to_string().to_uppercase();
                if &symbol == "NOT_NA_N" {
                    symbol = "NOT_NAN".to_string()
                }
                Some((format!("{}::{}", kind.kind, symbol), &enumerant.symbol))
            }
        }).map(|(check, show)| {
            format!("        if self.contains(spirv::{}) {{ bits.push(\"{}\") }}", check, show)
        }).collect();

        format!("impl Disassemble for spirv::{kind} {{\n\
                 {s:4}fn disassemble(&self) -> String {{\n\
                 {s:8}if self.is_empty() {{ return \"None\".to_string() }}\n\
                 {s:8}let mut bits = vec![];\n\
                 {checks}\n\
                 {s:8}bits.join(\"|\")\n\
                 {s:4}}}\n\
                 }}",
                s = " ",
                kind = kind.kind,
                checks = checks.join("\n"))
    }).collect();

    elements.join("\n\n")
}
