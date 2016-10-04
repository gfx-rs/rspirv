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

    definition = ['#[allow(dead_code)]',
                  '#[derive(Clone, Copy, Debug)]',
                  'pub enum OperandKind {{',
                  '    {kinds}',
                  '}}\n']
    return '\n'.join(definition).format(kinds=',\n    '.join(enums))


def generate_decode_error(enums):
    """Returns the Rust Error enum containing all errors for decoding
    SPIR-V operand values and its implementations."""
    kinds = [e['kind'] for e in enums]

    # The Rust Error enum.
    errors = ['{}Unknown(usize, spirv::Word)'.format(k) for k in kinds]
    definition = ['#[allow(dead_code)]',
                  '#[derive(Clone, Copy, Debug, PartialEq)]',
                  'pub enum Error {{',
                  '    StreamExpected(usize),',
                  '    {errors}',
                  '}}\n']
    enum = '\n'.join(definition).format(errors=',\n    '.join(errors))

    # impl fmt::Display for the Error enum.
    errors = ['Error::{}Unknown(index, word) => write!(f, "unknown value {{}} '
              'for operand kind {} at index {{}}", word, index)'.format(k, k)
              for k in kinds]
    definition = ['impl fmt::Display for Error {{',
                  '    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{',
                  '        match *self {{',
                  '            Error::StreamExpected(index) => write!(f, '
                  '"expected more bytes in the stream at index {{}}", index),',
                  '            {errors}',
                  '        }}',
                  '    }}',
                  '}}\n']
    display_impl = '\n'.join(definition).format(
        errors=',\n            '.join(errors))

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
        '}'])

    no_format = '#![cfg_attr(rustfmt, rustfmt_skip)]'
    use = 'use spirv;\nuse std::{error, fmt};'

    return '{}\n\n{}\n\n{}\n{}\n{}'.format(no_format, use, enum,
                                           display_impl, error_impl)


def convert_name_to_snake_case(variable):
    return re.sub(r'([a-z])([A-Z])', r'\1_\2', variable).lower()


def generate_decoder(enums):
    """Returns the implementation of various operand decoding methods."""
    kinds = [e['kind'] for e in enums]
    bitflag_kinds = set([e['kind'] for e in enums
                         if e['category'] == 'BitEnum'])

    # Method definition for decoding values of a particular operand kind.
    # If the operand kind belongs to BitEnum, we use from_bits(), otherwise,
    # from_u32().
    f = ['    pub fn {fname}(&mut self) -> Result<spirv::{kind}> {{',
         '        if let Some(word) = self.next() {{',
         '            spirv::{kind}::from_{ty}(word)'
         '.ok_or(Error::{kind}Unknown(self.index, word))',
         '        }} else {{',
         '            Err(Error::StreamExpected(self.index))',
         '        }}',
         '    }}']
    functions = [
        '\n'.join(f).format(fname=convert_name_to_snake_case(k),
                            kind=k,
                            ty="bits" if k in bitflag_kinds else "u32")
        for k in kinds
        # For kinds whose values may occupy more than one word, we need to
        # implement manually.
        if not (k.startswith('Pair') or k.startswith('Id') or
                k.startswith('Literal'))]
    return '\n'.join(
        ['use num::FromPrimitive;\n',
         'impl OperandDecoder {{',
         '{functions}',
         '}}']).format(functions='\n\n'.join(functions))


def main():
    import argparse
    parser = argparse.ArgumentParser(
        description='Generate SPIR-V grammar tables')
    parser.add_argument('-i', '--input', metavar='<path>',
                        type=str, required=True,
                        help='input JSON grammar file for core SPIR-V '
                        'instructions')
    parser.add_argument('-t', '--inst-table-output', metavar='<path>',
                        type=str, required=True,
                        help='output file for core SPIR-V instructions')
    parser.add_argument('-d', '--operand-decode-output', metavar='<path>',
                        type=str, required=True,
                        help='output file for SPIR-V operand decoding methods')
    parser.add_argument('-e', '--operand-error-output', metavar='<path>',
                        type=str, required=True,
                        help='output file for SPIR-V operand decoding errors')
    args = parser.parse_args()

    with open(args.input) as json_file:
        grammar = json.loads(json_file.read())

        core_grammar_output = open(args.inst_table_output, 'w')
        operand_decode_output = open(args.operand_decode_output, 'w')
        operand_error_output = open(args.operand_error_output, 'w')
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
        print(generate_decoder(grammar['operand_kinds']),
              file=operand_decode_output)

        print(COPYRIGHT, file=operand_error_output)
        print(AUTO_GEN_COMMENT, file=operand_error_output)
        print('{}\n'.format(SPIRV_GRAMMAR_URL), file=operand_error_output)
        print(generate_decode_error(grammar['operand_kinds']),
              file=operand_error_output)

if __name__ == '__main__':
    main()
