use crate::structs;
use crate::utils::*;

use heck::{ShoutySnakeCase, SnakeCase};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// Returns the name of the method for decoding the given operand `kind`
/// in grammar.
fn get_decode_method(kind: &str) -> Ident {
    if kind.starts_with("Id") {
        return as_ident("id");
    }

    let mut kind = kind;
    if kind.starts_with("Literal") {
        kind = &kind["Literal".len()..];
        if kind == "Integer" {
            return as_ident("int32");
        }
    }
    as_ident(&kind.to_snake_case())
}

/// Returns the generated operand decoding errors for binary::Decoder by
/// walking the given SPIR-V operand kinds `grammar`.
pub fn gen_operand_decode_errors(grammar: &Vec<structs::OperandKind>) -> TokenStream {
    let kinds: Vec<&str> = grammar
        .iter()
        .filter(|element| {
            !(element.kind.starts_with("Pair")
                || element.kind.starts_with("Id")
                || element.kind.starts_with("Literal"))
        })
        .map(|element| element.kind.as_str())
        .collect();

    let error_types = kinds.iter().map(|kind| {
        let error_name = as_ident(&format!("{}Unknown", kind));
        quote! { #error_name(usize, spirv::Word) }
    });

    let error_matches = kinds.iter().map(|kind| {
        let error_name = as_ident(&format!("{}Unknown", kind));
        let fmt_string = format!("unknown value {{}} for operand kind {} at index {{}}", kind);
        quote! {
            Error::#error_name(index, word) => write!(f, #fmt_string, word, index)
        }
    });

    quote! {
        use spirv;
        use std::{error, fmt};

        #[doc = "Decoder Error"]
        #[derive(Debug, PartialEq)]
        pub enum Error {
            StreamExpected(usize),
            LimitReached(usize),
            #(#error_types),*,
            ///Failed to decode a string.
            ///
            ///For structured error handling, the second element could be
            ///`string::FromUtf8Error`, but the will prohibit the compiler
            ///from generating `PartialEq` for this enum.
            DecodeStringFailed(usize, String),
        }

        impl fmt::Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Error::StreamExpected(index) => write!(f, "expected more bytes in the stream at index {}", index),
                    Error::LimitReached(index) => write!(f, "reached word limit at index {}", index),
                    #(#error_matches),*,
                    Error::DecodeStringFailed(index, ref e) => write!(f, "cannot decode string at index {}: {}", index, e),
                }
            }
        }

        impl error::Error for Error {}
    }
}

/// Returns the generated operand decoding methods for binary::Decoder by
/// walking the given SPIR-V operand kinds `grammar.
pub fn gen_operand_decode_methods(grammar: &Vec<structs::OperandKind>) -> TokenStream {
    let methods = grammar.iter().filter(|element| {
        // For kinds whose values may occupy more than one word, we need to
        // implement manually.
        !(element.kind.starts_with("Pair") ||
          element.kind.starts_with("Id") ||
          element.kind.starts_with("Literal"))
    }).map(|element| {
        // Method definition for decoding values of a particular operand
        // kind. If the operand kind belongs to BitEnum, we use from_bits(),
        // otherwise, from_u32().
        let kind = as_ident(&element.kind);
        let comment = format!("Decodes and returns the next SPIR-V word as\na SPIR-V {} value.", kind);
        let function_name = as_ident(&element.kind.to_snake_case());
        let convert = as_ident(&format!("from_{}", if element.category == structs::Category::BitEnum { "bits" } else { "u32" }));
        let error_name = as_ident(&format!("{}Unknown", kind));
        quote! {
            #[doc = #comment]
            pub fn #function_name(&mut self) -> Result<spirv::#kind> {
                if let Ok(word) = self.word() {
                    spirv::#kind::#convert(word).ok_or(Error::#error_name(self.offset - WORD_NUM_BYTES, word))
                } else {
                    Err(Error::StreamExpected(self.offset))
                }
            }
        }
    });

    quote! {
        use num_traits::FromPrimitive;

        impl<'a> Decoder<'a> {
            #(#methods)*
        }
    }
}

/// Generates the methods for parsing parameters of operand kind enumerants.
/// Returns a vector of tuples, with the first element being the enumerant
/// symbol, and the second element being a list of parameter kinds to that
/// enumerant.
fn gen_operand_param_parse_methods(
    grammar: &Vec<structs::OperandKind>,
) -> Vec<(&str, TokenStream)> {
    grammar.iter().filter(|element| {
        // Filter out all the operand kinds without any enumerants.
        element.enumerants.len() != 0
    }).filter_map(|element| {
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

        let kind = as_ident(&element.kind);
        let lo_kind = as_ident(&element.kind.to_snake_case());
        let function_name = as_ident(&format!("parse_{}_arguments", lo_kind));

        let method = if element.category == structs::Category::BitEnum {
            // For each operand kind in the BitEnum category, its
            // enumerants are bit masks. If a certain bit having associated
            // parameters is set, we also need to decode the corresponding
            // parameters. E.g., for MemoryAccess Aigned, an additional
            // LiteralInteger, which stands for the known alignment, should
            // be decoded.


            // Compose bit-set-clear check for each bit requiring
            // associated parameters.
            let cases = pairs.into_iter().map(|(symbol, params)| {
                let params = params.iter().map(|element| {
                    let op_kind = get_dr_operand_kind(element);
                    let decode = get_decode_method(element);
                    quote! { dr::Operand::#op_kind(self.decoder.#decode()?) }
                });
                let bit = as_ident(&symbol.to_shouty_snake_case());
                quote! {
                    if #lo_kind.contains(spirv::#kind::#bit) {
                        params.append(&mut vec![#(#params),*]);
                    }
                }
            });
            quote! {
                fn #function_name(&mut self, #lo_kind: spirv::#kind) -> Result<Vec<dr::Operand>> {
                    let mut params = vec![];
                    #(#cases)*
                    Ok(params)
                }
            }
        } else {  // ValueEnum
            let cases = pairs.into_iter().map(|(symbol, params)| {
                let params = params.iter().map(|element| {
                    let op_kind = get_dr_operand_kind(element);
                    let decode = get_decode_method(element);
                    quote! { dr::Operand::#op_kind(self.decoder.#decode()?) }
                });
                let symbol = as_ident(symbol);
                quote! {
                    spirv::#kind::#symbol => vec![#(#params),*]
                }
            });
            // TODO: filter duplicated symbols mapping to the same discriminator to avoid
            // unreachable patterns.
            quote! {
                #[allow(unreachable_patterns)]
                fn #function_name(&mut self, #lo_kind: spirv::#kind) -> Result<Vec<dr::Operand>> {
                    Ok(match #lo_kind {
                        #(#cases),*,
                        _ => vec![],
                    })
                }
            }
        };
        Some((element.kind.as_str(), method))
    }).collect()
}

/// Returns the generated operand parsing methods for binary::Parser by
/// walking the given SPIR-V operand kinds `grammar`.
pub fn gen_operand_parse_methods(grammar: &Vec<structs::OperandKind>) -> TokenStream {
    // Operand kinds whose enumerants have parameters. For these kinds, we need
    // to decode more than just the enumerants themselves.
    let (further_parse_kinds, further_parse_methods): (Vec<_>, Vec<_>) =
        gen_operand_param_parse_methods(grammar).into_iter().unzip();
    let further_parse_cases = further_parse_kinds.iter().map(|kind| {
        let function_name = as_ident(&format!("parse_{}_arguments", kind.to_snake_case()));
        let decode = get_decode_method(kind);
        let kind = as_ident(kind);
        quote! {
            GOpKind::#kind => {
                let val = self.decoder.#decode()?;
                let mut ops = vec![dr::Operand::#kind(val)];
                ops.append(&mut self.#function_name(val)?);
                ops
            }
        }
    });

    // Logic operands that expand to concrete operand pairs,
    // that is, those operand kinds with 'Pair' name prefix.
    // We only have three cases. So hard code it.
    let pair_kinds = vec![
        ("LiteralInteger", "IdRef"),
        ("IdRef", "LiteralInteger"),
        ("IdRef", "IdRef"),
    ];
    let pair_cases = pair_kinds.iter().map(|&(k0, k1)| {
        let kind = as_ident(&format!("Pair{}{}", k0, k1));
        let kind0 = get_dr_operand_kind(k0);
        let kind1 = get_dr_operand_kind(k1);
        let method0 = get_decode_method(k0);
        let method1 = get_decode_method(k1);
        quote! {
            GOpKind::#kind => vec![
                dr::Operand::#kind0(self.decoder.#method0()?),
                dr::Operand::#kind1(self.decoder.#method1()?),
            ]
        }
    });

    // These kinds are manually handled.
    let manual_kinds = vec![
        "IdResultType",
        "IdResult",
        "LiteralContextDependentNumber",
        "LiteralSpecConstantOpInteger",
    ];

    // For the rest operand kinds, which takes exactly one word.
    let normal_cases = grammar
        .iter()
        .filter_map(|element| {
            if further_parse_kinds.contains(&element.kind.as_str())
                || manual_kinds.contains(&element.kind.as_str())
                || element.kind.starts_with("Pair")
            {
                None
            } else {
                Some(element.kind.as_str())
            }
        })
        .map(|kind| {
            let gkind = as_ident(kind);
            let dkind = get_dr_operand_kind(kind);
            let decode = get_decode_method(kind);
            quote! {
                GOpKind::#gkind => vec![dr::Operand::#dkind(self.decoder.#decode()?)]
            }
        });

    let manual_cases = manual_kinds.iter().map(|kind| {
        let kind = as_ident(kind);
        quote! {
            GOpKind::#kind => panic!() // not handled here
        }
    });

    quote! {
        impl<'c, 'd> Parser<'c, 'd> {
            fn parse_operand(&mut self, kind: GOpKind) -> Result<Vec<dr::Operand>> {
                Ok(match kind {
                    #(#normal_cases),*,
                    #(#pair_cases),*,
                    #(#further_parse_cases),*,
                    #(#manual_cases),*,
                })
            }
            #(#further_parse_methods)*
        }
    }
}

pub fn gen_disas_bit_enum_operands(grammar: &Vec<structs::OperandKind>) -> TokenStream {
    let elements = grammar
        .iter()
        .filter(|op_kind| op_kind.category == structs::Category::BitEnum)
        .map(|op_kind| {
            let kind = as_ident(&op_kind.kind);
            let checks = op_kind
                .enumerants
                .iter()
                .filter_map(|enumerant| {
                    if enumerant.value == 0x0000 {
                        None
                    } else {
                        let symbol = as_ident(
                            &enumerant
                                .symbol
                                .to_snake_case()
                                .replace("na_n", "nan")
                                .to_uppercase(),
                        );
                        Some((quote! { #kind::#symbol }, &enumerant.symbol))
                    }
                })
                .map(|(check, show)| {
                    quote! { if self.contains(spirv::#check) { bits.push(#show) } }
                });

            quote! {
                impl Disassemble for spirv::#kind {
                    fn disassemble(&self) -> String {
                        if self.is_empty() {
                            return "None".to_string();
                        }

                        let mut bits = vec![];
                        #(#checks)*
                        bits.join("|")
                    }
                }
            }
        });

    quote! {
        #(#elements)*
    }
}
