#!/usr/bin/env python
# Copyright 2016 Google Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""Generates various info tables from the SPIR-V JSON grammar."""

from __future__ import print_function

import json
import re

COPYRIGHT = '''// Copyright 2016 Google Inc.
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
'''

AUTO_GEN_COMMENT = ('// This rust module is automatically generated '
                    'from the SPIR-V JSON grammar:')

SPIRV_GRAMMAR_URL = ('//   https://github.com/KhronosGroup/SPIRV-Headers/\n'
                     '//           blob/master/include/spirv/1.1/'
                     'spirv.core.grammar.json')


def convert_operand_quantifier(quantifier):
    assert quantifier in ['', '?', '*']

    if quantifier == '':
        return 'One'
    if quantifier == '?':
        return 'ZeroOrOne'
    if quantifier == '*':
        return 'ZeroOrMore'


class InstInitializer(object):
    """Instances holds a SPIR-V instruction suitable for printing
    as the initializer."""

    def __init__(self, opname, caps, operands):
        """Initialization.

        Arguments:
          - opname: opcode name (with the 'Op' prefix)
          - caps: a sequence of capability names required by this opcode
          - operands: a sequence of (operand-kind, operand-quantifier) tuples
        """
        assert opname.startswith('Op')
        self.opname = opname[2:]  # Remove the 'Op' prefix
        self.caps = caps
        self.operands = [(o[0], convert_operand_quantifier(o[1]))
                         for o in operands]

    def __str__(self):
        operand_strs = ['({}, {})'.format(*o) for o in self.operands]

        template = ['inst!({opname}', '[{caps}]', '[{operands}])']
        return ', '.join(template).format(
            opname=self.opname,
            caps=', '.join(self.caps),
            operands=', '.join(operand_strs))


def generate_instruction(inst):
    """Returns the Rust initializer for the given SPIR-V instruction."""
    opname = inst['opname']
    caps = inst.get('capabilities', [])
    operands = inst.get('operands', {})
    operands = [(o['kind'], o.get('quantifier', '')) for o in operands]

    return str(InstInitializer(opname, caps, operands))


def generate_instruction_table(inst_table):
    """Returns the info table containing all SPIR-V instructions."""
    table = [
        '#[cfg_attr(rustfmt, rustfmt_skip)]\n',
        "static INSTRUCTION_TABLE: &'static [Instruction<'static>] = &[\n    ",
        ',\n    '.join([generate_instruction(inst) for inst in inst_table]),
        '\n];']
    return ''.join(table)


def generate_operand_kind_table(enums):
    """Returns the Rust enum containing all SPIR-V operand kinds."""
    enums = [e['kind'] for e in enums]

    definition = ['/// All operand kinds in SPIR-V grammar.',
                  '#[derive(Clone, Copy, Debug)]',
                  'pub enum OperandKind {{',
                  '    {kinds}',
                  '}}\n']
    return '\n'.join(definition).format(kinds=',\n    '.join(enums))


def generate_operand_decode_errors(enums):
    """Returns the Rust Error enum containing all errors for decoding
    SPIR-V operand values and its implementations."""
    kinds = [e['kind'] for e in enums]

    # The Rust Error enum.
    errors = ['{}Unknown(usize, spirv::Word)'.format(k)
              for k in kinds
              if not (k.startswith('Pair') or k.startswith('Id') or
                      k.startswith('Literal'))]
    definition = ['#[derive(Debug, PartialEq)]',
                  'pub enum Error {{',
                  '    StreamExpected(usize),',
                  '    LimitReached(usize),',
                  '    {errors},',
                  '    {special_errors}',
                  '}}\n']
    enum = '\n'.join(definition).format(
        errors=',\n    '.join(errors),
        special_errors=('/// Failed to decode a string.\n\n'
                        '    /// For structured error handling, the second '
                        'element could be\n    /// `string::FromUtf8Error`, '
                        'but the will prohibit the compiler\n    /// from '
                        'generating `PartialEq` for this enum.\n'
                        '    DecodeStringFailed(usize, String)'))

    # impl fmt::Display for the Error enum.
    errors = ['Error::{}Unknown(index, word) => write!(f, "unknown value {{}} '
              'for operand kind {} at index {{}}", word, index)'.format(k, k)
              for k in kinds
              if not (k.startswith('Pair') or k.startswith('Id') or
                      k.startswith('Literal'))]
    definition = ['impl fmt::Display for Error {{',
                  '    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{',
                  '        match *self {{',
                  '            Error::StreamExpected(index) => write!(f, '
                  '"expected more bytes in the stream at index {{}}", index),',
                  '            Error::LimitReached(index) => write!(f, '
                  '"reached word limit at index {{}}", index),',
                  '            {errors},',
                  '            {special_errors}',
                  '        }}',
                  '    }}',
                  '}}\n']
    display_impl = '\n'.join(definition).format(
        errors=',\n            '.join(errors),
        special_errors=('Error::DecodeStringFailed(index, ref e) => write!(f, '
                        '"cannot decode string at index {}: {}", index, e)'))

    # impl error::Error for the Error enum.
    error_impl = '\n'.join([
        'impl error::Error for Error {',
        '    fn description(&self) -> &str {',
        '        match *self {',
        '            Error::StreamExpected(_) => '
        '"expected more bytes in the stream",',
        '            _ => "unknown operand value for the given kind",',
        '        }',
        '    }',
        '}\n'])

    no_format = '#![cfg_attr(rustfmt, rustfmt_skip)]\n'
    use = ('use spirv;\n\n'
           'use std::{error, fmt};\n')

    return '\n'.join([no_format, use, enum, display_impl, error_impl])


def generate_mr_operand_kinds(enums):
    """Returns the mr::Operand enum and its implementations."""
    kinds = [e['kind'] for e in enums]
    # We do not use Pair kinds in the mr::Operand.
    kinds = [k for k in kinds if not k.startswith('Pair')]

    id_kinds = [k for k in kinds if k.startswith('Id')]
    num_kinds = [k for k in kinds
                 if k.endswith('Integer') or k.endswith('Number')]
    str_kinds = [k for k in kinds if k.endswith('String')]
    normal_kinds = [k for k in kinds
                    if ((k not in id_kinds) and (k not in num_kinds) and
                        (k not in str_kinds))]

    normal_kinds = ['{k}(spirv::{k})'.format(k=k) for k in normal_kinds]
    id_kinds = ['{kind}(spirv::Word)'.format(kind=k) for k in id_kinds]
    num_kinds = ['{kind}(u32)'.format(kind=k) for k in num_kinds]
    str_kinds = ['{kind}(String)'.format(kind=k) for k in str_kinds]

    # The Rust enum.
    definition = ['/// Memory representation of a SPIR-V operand.',
                  '#[derive(Debug, PartialEq)]',
                  'pub enum Operand {{',
                  '    {normal_cases},',
                  '    {id_cases},',
                  '    {num_cases},',
                  '    {str_cases}',
                  '}}\n']
    enum = '\n'.join(definition).format(
        normal_cases=',\n    '.join(normal_kinds),
        id_cases=',\n    '.join(id_kinds),
        num_cases=',\n    '.join(num_kinds),
        str_cases=',\n    '.join(str_kinds))

    # impl fmt::Display for the enum.
    kinds = ['Operand::{}(ref v) => write!(f, "{{:?}}", v)'.format(k)
             for k in kinds]
    definition = ['impl fmt::Display for Operand {{',
                  '    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{',
                  '        match *self {{',
                  '            {cases},',
                  '        }}',
                  '    }}',
                  '}}\n']
    impl = '\n'.join(definition).format(
        cases=',\n            '.join(kinds))

    no_format = '#![cfg_attr(rustfmt, rustfmt_skip)]\n'
    use = ('use spirv;\n\n'
           'use std::fmt;\n')

    return '\n'.join([no_format, use, enum, impl])


def convert_symbol_to_snake_case(variable):
    return re.sub(r'([a-z])([A-Z])', r'\1_\2', variable).lower()


def generate_operand_decode_methods(enums):
    """Returns the implementation of various operand decoding methods."""
    kinds = [e['kind'] for e in enums]
    bitflag_kinds = set([e['kind'] for e in enums
                         if e['category'] == 'BitEnum'])

    # Method definition for decoding values of a particular operand kind.
    # If the operand kind belongs to BitEnum, we use from_bits(), otherwise,
    # from_u32().
    f = ['    /// Decodes and returns the next SPIR-V word as a SPIR-V',
         '    /// {kind} value.',
         '    pub fn {fname}(&mut self) -> Result<spirv::{kind}> {{',
         '        if let Ok(word) = self.word() {{',
         '            spirv::{kind}::from_{ty}(word)'
         '.ok_or(Error::{kind}Unknown(self.offset, word))',
         '        }} else {{',
         '            Err(Error::StreamExpected(self.offset))',
         '        }}',
         '    }}']
    functions = [
        '\n'.join(f).format(fname=convert_symbol_to_snake_case(k),
                            kind=k,
                            ty="bits" if k in bitflag_kinds else "u32")
        for k in kinds
        # For kinds whose values may occupy more than one word, we need to
        # implement manually.
        if not (k.startswith('Pair') or k.startswith('Id') or
                k.startswith('Literal'))]
    return '\n'.join(
        ['use num::FromPrimitive;\n',
         'impl Decoder {{',
         '{functions}',
         '}}']).format(functions='\n\n'.join(functions))


def get_decode_method(kind):
    assert not kind.startswith('Pair')
    if kind.startswith('Id'):
        return 'id'
    elif kind.startswith('Literal'):
        kind = kind[len('Literal'):]
    return convert_symbol_to_snake_case(kind)


def generate_operand_param_parse_methods(enums):
    """Returns the implementation of various operand param parsing methods."""
    # These are operand kinds that may have additional parameters.
    # So we need additional parsing methods for them to parse their
    # additional arguments.
    further_parse_methods = []
    for kind in enums:
        enumerants = []
        if 'enumerants' not in kind:
            continue
        for e in kind['enumerants']:
            ename = e['enumerant']
            if 'parameters' in e:
                eparam = [p['kind'] for p in e['parameters']]
                enumerants.append((ename, eparam))
        if len(enumerants) != 0:
            if kind['category'] == 'BitEnum':
                template = [
                    '    #[allow(unused_variables)]',
                    '    fn parse_{k}_arguments(&mut self, {k}: spirv::{kind})'
                    ' -> Result<Vec<mr::Operand>> {{',
                    '        unimplemented!()',
                    '    }}']
                decode_method = '\n'.join(template).format(
                    k=convert_symbol_to_snake_case(kind['kind']),
                    kind=kind['kind'])
                further_parse_methods.append((kind['kind'], decode_method))
            else:
                template = [
                    '    fn parse_{k}_arguments(&mut self, {k}: spirv::{kind})'
                    ' -> Result<Vec<mr::Operand>> {{',
                    '        Ok(match {k} {{',
                    '            {cases},',
                    '            {default}',
                    '        }})',
                    '    }}']
                cases = [
                    'spirv::{kind}::{ename} => vec![{eparams}]'.format(
                        kind=kind['kind'],
                        ename=name,
                        eparams=', '.join([
                            'mr::Operand::{kind}'
                            '(try_decode!(self.decoder.{decode}()))'.format(
                                kind=p, decode=get_decode_method(p))
                            for p in params]))
                    for name, params in enumerants]
                decode_method = '\n'.join(template).format(
                    k=convert_symbol_to_snake_case(kind['kind']),
                    kind=kind['kind'],
                    cases=',\n            '.join(cases),
                    default='_ => vec![]')
                further_parse_methods.append((kind['kind'], decode_method))
    return further_parse_methods


def generate_operand_parse_methods(enums):
    """Returns the implementation of various operand parsing methods."""
    further_parse_methods = generate_operand_param_parse_methods(enums)

    all_kinds = [e['kind'] for e in enums]

    further_parse_kinds = [e[0] for e in further_parse_methods]
    case_template = [
        'GOpKind::{kind} => {{',
        '                let val = try_decode!(self.decoder.{decode}());',
        '                let mut ops = vec![mr::Operand::{kind}(val)];',
        '                ops.append(&mut try!(self.parse_{k}_arguments(val)));',
        '                ops',
        '            }}']
    further_parse_cases = [
        '\n'.join(case_template).format(
            k=convert_symbol_to_snake_case(k),
            kind=k,
            decode=get_decode_method(k))
        for k in further_parse_kinds]

    # Logic operands that do not expand to concrete operand pairs,
    # that is, those operand kinds without 'Pair' name prefix.
    non_pair_cases = ['GOpKind::{kind} => vec![mr::Operand::{kind}'
                      '(try_decode!(self.decoder.{decode}()))]'.format(
                          kind=k, decode=get_decode_method(k))
                      for k in all_kinds if not (k in further_parse_kinds or
                                                 k.startswith('Pair'))]

    # Logic operands that expand to concrete operand pairs,
    # that is, those operand kinds with 'Pair' name prefix.
    # We only have three cases. So hard code it.
    pair_kinds = [('LiteralInteger', 'IdRef'),
                  ('IdRef', 'LiteralInteger'),
                  ('IdRef', 'IdRef')]
    case_template = [
        'GOpKind::{kind} => {{',
        '                vec!['
        'mr::Operand::{kind0}(try_decode!(self.decoder.{decode0}())), '
        'mr::Operand::{kind1}(try_decode!(self.decoder.{decode1}()))'
        ']',
        '            }}']
    pair_cases = [
        '\n'.join(case_template).format(
            kind='Pair{}{}'.format(k0, k1),
            kind0=k0, decode0=get_decode_method(k0),
            kind1=k1, decode1=get_decode_method(k1))
        for (k0, k1) in pair_kinds]

    # Parser method for requesting values of a specific operand kind and
    # converting it to mr::Operand.
    parser = ['{no_format}',
              "impl<'a> Parser<'a> {{",
              '    fn parse_operand(&mut self, kind: GOpKind) '
              '-> Result<Vec<mr::Operand>> {{',
              '        Ok(match kind {{',
              '            {non_pair_cases},',
              '            {pair_cases}',
              '            {further_parse_cases}',
              '        }})',
              '    }}\n',
              '{functions}',
              '}}']

    return '\n'.join(parser).format(
        no_format='',
        non_pair_cases=',\n            '.join(non_pair_cases),
        pair_cases='\n            '.join(pair_cases),
        further_parse_cases='\n            '.join(further_parse_cases),
        functions='\n\n'.join([e[1] for e in further_parse_methods]))


def update(grammar_input, inst_table_output, operand_decode_output,
           operand_error_output, operand_parse_output, mr_operand_output):
    """Updates all generated tables using the JSON grammar."""
    with open(grammar_input) as json_file:
        grammar = json.loads(json_file.read())

        core_grammar_output = open(inst_table_output, 'w')
        operand_decode_output = open(operand_decode_output, 'w')
        operand_error_output = open(operand_error_output, 'w')
        operand_parse_output = open(operand_parse_output, 'w')
        mr_operand_output = open(mr_operand_output, 'w')

        print(COPYRIGHT, file=core_grammar_output)
        print(AUTO_GEN_COMMENT, file=core_grammar_output)
        print('{}\n'.format(SPIRV_GRAMMAR_URL), file=core_grammar_output)
        print(generate_operand_kind_table(grammar['operand_kinds']),
              file=core_grammar_output)
        print(generate_instruction_table(grammar['instructions']),
              file=core_grammar_output)

        print(COPYRIGHT, file=operand_decode_output)
        print(AUTO_GEN_COMMENT, file=operand_decode_output)
        print('{}\n'.format(SPIRV_GRAMMAR_URL), file=operand_decode_output)
        print(generate_operand_decode_methods(grammar['operand_kinds']),
              file=operand_decode_output)

        print(COPYRIGHT, file=operand_error_output)
        print(AUTO_GEN_COMMENT, file=operand_error_output)
        print('{}\n'.format(SPIRV_GRAMMAR_URL), file=operand_error_output)
        print(generate_operand_decode_errors(grammar['operand_kinds']),
              file=operand_error_output)

        print(COPYRIGHT, file=operand_parse_output)
        print(AUTO_GEN_COMMENT, file=operand_parse_output)
        print('{}\n'.format(SPIRV_GRAMMAR_URL), file=operand_parse_output)
        print(generate_operand_parse_methods(grammar['operand_kinds']),
              file=operand_parse_output)

        print(COPYRIGHT, file=mr_operand_output)
        print(AUTO_GEN_COMMENT, file=mr_operand_output)
        print('{}\n'.format(SPIRV_GRAMMAR_URL), file=mr_operand_output)
        print(generate_mr_operand_kinds(grammar['operand_kinds']),
              file=mr_operand_output)


def main():
    import argparse
    parser = argparse.ArgumentParser(
        description='Generate SPIR-V grammar tables')
    parser.add_argument('-i', '--input', metavar='<path>',
                        type=str, required=True,
                        help='input JSON file for core SPIR-V grammar')
    parser.add_argument('-t', '--inst-table-output', metavar='<path>',
                        type=str, required=True,
                        help='output file for core SPIR-V instruction table')
    parser.add_argument('-d', '--operand-decode-output', metavar='<path>',
                        type=str, required=True,
                        help='output file for SPIR-V operand decoding methods')
    parser.add_argument('-e', '--operand-error-output', metavar='<path>',
                        type=str, required=True,
                        help='output file for SPIR-V operand decoding errors')
    parser.add_argument('-p', '--operand-parse-output', metavar='<path>',
                        type=str, required=True,
                        help='output file for SPIR-V operand parsing methods')
    parser.add_argument('-m', '--mr-operand-output', metavar='<path>',
                        type=str, required=True,
                        help='output file for mr operand definition')
    args = parser.parse_args()

    update(args.input, args.inst_table_output, args.operand_decode_output,
           args.operand_error_output, args.operand_parse_output,
           args.mr_operand_output)

if __name__ == '__main__':
    main()
