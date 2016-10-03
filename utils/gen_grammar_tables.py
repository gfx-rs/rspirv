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

AUTO_GEN_COMMENT = '// This rust module is automatically generated from ' \
    'the SPIR-V JSON grammar:\n' \
    '//  https://raw.githubusercontent.com/KhronosGroup/SPIRV-Tools/' \
    'master/source/spirv.core.grammar.json\n'


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


def main():
    import argparse
    parser = argparse.ArgumentParser(
        description='Generate SPIR-V grammar tables')
    parser.add_argument('--spirv-core-grammar', metavar='<path>',
                        type=str, required=True,
                        help='input JSON grammar file for core SPIR-V '
                        'instructions')
    parser.add_argument('--core-grammar-output', metavar='<path>',
                        type=str, required=True,
                        help='output file for core SPIR-V instructions')
    args = parser.parse_args()

    with open(args.spirv_core_grammar) as json_file:
        grammar = json.loads(json_file.read())

        core_grammar_output = open(args.core_grammar_output, 'w')
        print(AUTO_GEN_COMMENT, file=core_grammar_output)
        print(generate_operand_kind_table(grammar['operand_kinds']),
              file=core_grammar_output)
        print(generate_instruction_table(grammar['instructions']),
              file=core_grammar_output)

if __name__ == '__main__':
    main()
