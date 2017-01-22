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

use std::fs;
use std::io::Write;
use structs;

use utils::*;

/// Returns the name of the method for decoding the given operand `kind`
/// in grammar.
fn get_decode_method(kind: &str) -> String {
    if kind.starts_with("Id") {
        return "id".to_string()
    }

    let mut kind = kind;
    if kind.starts_with("Literal") {
        kind = &kind["Literal".len()..];
        if kind == "Integer" {
            return "int32".to_string()
        }
    }
    snake_casify(kind)
}

/// Writes the generated operand decoding errors for binary::Decoder by
/// walking the given SPIR-V operand kinds `grammar` to the file with
/// the given `filename`.
pub fn write_operand_decode_errors(grammar: &Vec<structs::OperandKind>,
                                   filename: &str) {
    let mut file = fs::File::create(filename).unwrap();

    { // Comments, attributes, uses.
        write_copyright_autogen_comment(&mut file);
        file.write_all(RUSTFMT_SKIP_BANG.as_bytes()).unwrap();
        file.write_all(b"\n\nuse spirv;\nuse std::{error, fmt};\n\n").unwrap();
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
        s="", errors=errors.join("\n"));
    file.write_all(&error_enum.into_bytes()).unwrap();

    // impl fmt::Display for the Error enum.
    let errors: Vec<String> = kinds.iter().map(|element| {
        format!("{s:12}Error::{kind}Unknown(index, word) => write!(\
                 f, \"unknown value {{}} for operand kind {kind} \
                 at index {{}}\", word, index),",
                s="",
                kind=element)
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
        s="", errors=errors.join("\n"));
    file.write_all(&display_impl.into_bytes()).unwrap();

    // impl error::Error for the Error enum.
    let error_impl = format!(
        "impl error::Error for Error {{\n\
         {s:4}fn description(&self) -> &str {{\n\
         {s:8}match *self {{\n\
         {s:12}Error::StreamExpected(_) => \"expected more bytes \
             in the stream\",\n\
         {s:12}_ => \"unknown operand value for the given kind\",\n\
         {s:8}}}\n{s:4}}}\n}}\n",
        s="");
    file.write_all(&error_impl.into_bytes()).unwrap();
}

/// Writes the generated operand decoding methods for binary::Decoder by
/// walking the given SPIR-V operand kinds `grammar to the file with
/// the given `filename`.
pub fn write_operand_decode_methods(grammar: &Vec<structs::OperandKind>,
                                    filename: &str) {
    let mut file = fs::File::create(filename).unwrap();

    write_copyright_autogen_comment(&mut file);

    file.write_all(b"use num::FromPrimitive;\n\n").unwrap();

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
             s="",
             fname=snake_casify(&element.kind),
             kind=element.kind,
             ty=if element.category == "BitEnum" { "bits" } else { "u32" })
    }).collect();

    let impl_code = format!("impl Decoder {{\n{}}}\n", methods.join("\n"));
    file.write_all(&impl_code.into_bytes()).unwrap();
}

/// Generates the methods for parsing parameters of operand kind enumerants.
/// Returns a vector of tuples, with the first element being the enumerant
/// symbol, and the second element being a list of parameter kinds to that
/// enumerant.
fn gen_operand_param_parse_methods(grammar: &Vec<structs::OperandKind>)
                                   -> Vec<(String, String)> {
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
            let up_kind = lo_kind.to_uppercase();

            // Compose bit-set-clear check for each bit requiring
            // associated parameters.
            let cases: Vec<String> = pairs.into_iter().map(|(symbol, params)| {
                let params: Vec<String> = params.iter().map(|element| {
                    format!("mr::Operand::{kind}(\
                             try_decode!(self.decoder.{decode}()))",
                            kind=get_mr_operand_kind(element),
                            decode=get_decode_method(element))
                }).collect();
                format!(
                    "{s:8}if {arg}.contains(spirv::{k}_{bit}) {{\n\
                         {s:12}params.append(&mut vec![{params}]);\n\
                     {s:8}}}",
                    s="", arg=lo_kind, k=up_kind,
                    bit=snake_casify(&symbol).to_uppercase(),
                    params=params.join(", "))
            }).collect();
            format!(
                "{s:4}fn parse_{k}_arguments(&mut self, {k}: \
                     spirv::{kind}) -> Result<Vec<mr::Operand>> {{\n\
                     {s:8}let mut params = vec![];\n\
                     {cases}\n\
                     {s:8}Ok(params)\n\
                 {s:4}}}",
                s="", cases=cases.join("\n"), k=lo_kind, kind=kind)
        } else {  // ValueEnum
            let cases: Vec<String> = pairs.into_iter().map(|(symbol, params)| {
                let params: Vec<String> = params.iter().map(|element| {
                    format!("mr::Operand::{kind}(\
                             try_decode!(self.decoder.{decode}()))",
                            kind=get_mr_operand_kind(element),
                            decode=get_decode_method(element))
                }).collect();
                format!(
                    "{s:12}spirv::{kind}::{symbol} => vec![{params}],",
                    s="", kind=kind, symbol=symbol,
                    params=params.join(", "))
            }).collect();
            format!(
                "{s:4}fn parse_{k}_arguments(&mut self, {k}: spirv::{kind})\
                     {s:1}-> Result<Vec<mr::Operand>> {{\n\
                     {s:8}Ok(match {k} {{\n\
                        {cases}\n\
                        {s:12}_ => vec![]\n\
                     {s:8}}})\n\
                 {s:4}}}",
                s="", kind=kind,
                k=snake_casify(kind),
                cases=cases.join("\n"))
        };
        Some((kind.to_string(), method))
    }).collect()
}

/// Writes the generated operand parsing methods for binary::Parser by
/// walking the given SPIR-V operand kinds `grammar` to the file with
/// the given `filename`.
pub fn write_operand_parse_methods(grammar: &Vec<structs::OperandKind>,
                                   filename: &str) {
    let mut file = fs::File::create(filename).unwrap();

    write_copyright_autogen_comment(&mut file);

    // Operand kinds whose enumerants have parameters. For these kinds, we need
    // to decode more than just the enumerants themselves.
    let (further_parse_kinds, further_parse_methods): (Vec<_>, Vec<_>) =
        gen_operand_param_parse_methods(grammar).iter().cloned().unzip();
    let further_parse_cases: Vec<String> =
        further_parse_kinds.iter().map(|kind| {
            format!(
                "{s:12}GOpKind::{kind} => {{\n\
                 {s:16}let val = try_decode!(self.decoder.{decode}());\n\
                 {s:16}let mut ops = vec![mr::Operand::{kind}(val)];\n\
                 {s:16}ops.append(&mut try!(self.parse_{k}_arguments(val)));\n\
                 {s:16}ops\n\
                 {s:12}}}",
                s="",
                kind=kind,
                k=snake_casify(kind),
                decode=get_decode_method(kind))
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
                 mr::Operand::{k0}(try_decode!(self.decoder.{m0}())), \
                 mr::Operand::{k1}(try_decode!(self.decoder.{m1}()))\
                 ]\n{s:12}}}",
                s="",
                kind=format!("Pair{}{}", k0, k1),
                k0=get_mr_operand_kind(k0),
                k1=get_mr_operand_kind(k1),
                m0=get_decode_method(k0), m1=get_decode_method(k1))
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
            "{s:12}GOpKind::{gkind} => vec![mr::Operand::{mkind}\
             (try_decode!(self.decoder.{decode}()))],",
             s="",
             gkind=kind,
             mkind=get_mr_operand_kind(kind),
             decode=get_decode_method(kind))
    }).collect();

    let unused_cases: Vec<String> =
        manual_kinds.iter().map(|element| {
            format!("{s:12}GOpKind::{k} => panic!(),  // not handled here",
                    s="", k=element)
        }).collect();

    let impl_code = format!(
        "impl<'a> Parser<'a> {{\n\
         {s:4}fn parse_operand(&mut self, kind: GOpKind) \
             -> Result<Vec<mr::Operand>> {{\n\
             {s:8}Ok(match kind {{\n\
                 {normal_cases}\n\
                 {pair_cases}\n\
                 {further_parse_cases}\n\
                 {unused_cases}\n\
             {s:8}}})\n\
         {s:4}}}\n\n\
         {functions}\n\
         }}",
        s="",
        normal_cases=normal_cases.join("\n"),
        pair_cases=pair_cases.join("\n"),
        further_parse_cases=further_parse_cases.join("\n"),
        unused_cases=unused_cases.join("\n"),
        functions=further_parse_methods.join("\n\n"));

    file.write_all(&impl_code.into_bytes()).unwrap();
}
